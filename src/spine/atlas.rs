//! Format described here http://pt.esotericsoftware.com/spine-atlas-format

use std::io::{BufReader, Read};

use anyhow::Error;
use bevy::math::Vec2;
use nom::*;

// TODO: Multiple page support
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Atlas {
    pub name: String,
    pub size: Size,
    pub format: Format,
    pub filter: Filter,
    pub repeat: Repeat,
    // TODO: pub pma: bool,
    pub regions: Vec<Region>,
}

impl Atlas {
    pub fn parse<R: Read>(reader: R) -> Result<Atlas, Error> {
        use self::parser::atlas;

        let mut r = BufReader::new(reader);
        let mut buf = Vec::new();
        r.read_to_end(&mut buf)?;
        Ok(atlas(&buf).to_result()?)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Format {
    Alpha,
    Intensity,
    LuminanceAlpha,
    RGB565,
    RGBA4444,
    RGB888,
    RGBA8888,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Filter {
    pub minification: FilterSetting,
    pub magnification: FilterSetting,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FilterSetting {
    Nearest,
    Linear,
    MipMap,
    MipMapNearestNearest,
    MipMapLinearNearest,
    MipMapNearestLinear,
    MipMapLinearLinear,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Repeat {
    X,
    Y,
    XY,
    No,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Size {
    pub width: u64,
    pub height: u64,
}

impl Into<Vec2> for Size {
    fn into(self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Region {
    pub name: String,
    pub rotate: bool,
    pub xy: Point,
    pub size: Size,
    pub orig: Point,
    pub offset: Point,
    pub index: i64,
    pub split: Option<Split>,
    pub pad: Option<Pad>,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Split {
    pub left: u64,
    pub right: u64,
    pub top: u64,
    pub bottom: u64,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Pad {
    pub left: i64,
    pub right: i64,
    pub top: i64,
    pub bottom: i64,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Into<Vec2> for Point {
    fn into(self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }
}

mod parser {
    use super::*;
    use nom::{digit, not_line_ending};
    use std::str;

    named!(pub decimal<usize>,
       map_res!(
           map_res!(
               call!(digit),
               str::from_utf8),
           |s| usize::from_str_radix(s, 10)
       )
    );

    named!(
        int64<i64>,
        map!(
            do_parse!(sign: opt!(tag!("-")) >> dec: decimal >> (sign, dec)),
            |(sign, dec): (Option<&[u8]>, usize)| {
                match sign {
                    None => dec as i64,
                    Some(b"-") => -(dec as i64),
                    _ => unreachable!(),
                }
            }
        )
    );

    named!(
        parse_bool<bool>,
        map_res!(alt!(tag!("false") | tag!("true")), |s: &[u8]| match s {
            b"false" => Ok(false),
            b"true" => Ok(true),
            _ => Err(error_code!(1)),
        })
    );

    named!(
        point<Point>,
        do_parse!(
            pair: ws!(separated_pair!(int64, tag!(","), int64))
                >> (Point {
                    x: pair.0,
                    y: pair.1
                })
        )
    );

    enum RValue {
        RRotate(bool),
        RXY(Point),
        RSize(Size),
        ROrig(Point),
        ROffset(Point),
        RIndex(i64),
        RSplit(Split),
        RPad(Pad),
    }

    named!(
        rotate<RValue>,
        map!(
            ws!(separated_pair!(
                tag!("rotate"),
                tag!(":"),
                call!(parse_bool)
            )),
            |(_, v)| RValue::RRotate(v)
        )
    );

    named!(
        xy<RValue>,
        map!(
            ws!(separated_pair!(tag!("xy"), tag!(":"), call!(point))),
            |(_, v)| RValue::RXY(v)
        )
    );

    named!(
        size_kv<RValue>,
        map!(
            ws!(separated_pair!(tag!("size"), tag!(":"), call!(size))),
            |(_, v)| RValue::RSize(v)
        )
    );

    named!(
        orig<RValue>,
        map!(
            ws!(separated_pair!(tag!("orig"), tag!(":"), call!(point))),
            |(_, v)| RValue::ROrig(v)
        )
    );

    named!(
        offset<RValue>,
        map!(
            ws!(separated_pair!(tag!("offset"), tag!(":"), call!(point))),
            |(_, v)| RValue::ROffset(v)
        )
    );

    named!(
        index<RValue>,
        map!(
            ws!(separated_pair!(tag!("index"), tag!(":"), call!(int64))),
            |(_, v)| RValue::RIndex(v)
        )
    );

    named!(
        split_kv<RValue>,
        map!(
            ws!(separated_pair!(tag!("split"), tag!(":"), call!(split))),
            |(_, v)| RValue::RSplit(v)
        )
    );

    named!(
        pad_kv<RValue>,
        map!(
            ws!(separated_pair!(tag!("pad"), tag!(":"), call!(pad))),
            |(_, v)| RValue::RPad(v)
        )
    );

    named!(
        region_value<RValue>,
        alt!(rotate | xy | size_kv | orig | offset | index | split_kv | pad_kv)
    );

    named!(
        split<Split>,
        do_parse!(
            a: call!(decimal)
                >> tag!(",")
                >> b: call!(decimal)
                >> tag!(",")
                >> c: call!(decimal)
                >> tag!(",")
                >> d: call!(decimal)
                >> (Split {
                    left: a as u64,
                    right: b as u64,
                    top: c as u64,
                    bottom: d as u64,
                })
        )
    );

    named!(
        pad<Pad>,
        do_parse!(
            a: call!(int64)
                >> tag!(",")
                >> b: call!(int64)
                >> tag!(",")
                >> c: call!(int64)
                >> tag!(",")
                >> d: call!(int64)
                >> (Pad {
                    left: a,
                    right: b,
                    top: c,
                    bottom: d,
                })
        )
    );

    fn named_region(name: &str) -> Region {
        Region {
            name: name.to_string(),
            rotate: false,
            xy: Point { x: 0, y: 0 },
            size: Size {
                width: 0,
                height: 0,
            },
            orig: Point { x: 0, y: 0 },
            offset: Point { x: 0, y: 0 },
            index: 0,
            split: None,
            pad: None,
        }
    }

    named!(
        region<Region>,
        do_parse!(
            name: map_res!(call!(not_line_ending), str::from_utf8)
                >> reg: fold_many1!(
                    region_value,
                    named_region(name),
                    |mut region: Region, value| {
                        match value {
                            RValue::RRotate(rot) => region.rotate = rot,
                            RValue::RXY(pt) => region.xy = pt,
                            RValue::RSize(sz) => region.size = sz,
                            RValue::ROrig(og) => region.orig = og,
                            RValue::ROffset(pt) => region.offset = pt,
                            RValue::RIndex(idx) => region.index = idx,
                            RValue::RSplit(sp) => region.split = Some(sp),
                            RValue::RPad(pd) => region.pad = Some(pd),
                        };
                        region
                    }
                )
                >> (reg)
        )
    );

    named!(
        header_format<Format>,
        map_res!(
            alt!(
                tag!("Alpha")
                    | tag!("Intensity")
                    | tag!("LuminanceAlpha")
                    | tag!("RGB565")
                    | tag!("RGBA4444")
                    | tag!("RGB888")
                    | tag!("RGBA8888")
            ),
            |s: &[u8]| match s {
                b"Alpha" => Ok(Format::Alpha),
                b"Intensity" => Ok(Format::Intensity),
                b"LuminanceAlpha" => Ok(Format::LuminanceAlpha),
                b"RGB565" => Ok(Format::RGB565),
                b"RGBA4444" => Ok(Format::RGBA4444),
                b"RGB888" => Ok(Format::RGB888),
                b"RGBA8888" => Ok(Format::RGBA8888),
                _ => Err(error_code!(2)),
            }
        )
    );

    named!(
        filter_setting<FilterSetting>,
        map_res!(
            alt!(
                tag!("Nearest")
                    | tag!("Linear")
                    | tag!("MipMap")
                    | tag!("MipMapNearestNearest")
                    | tag!("MipMapLinearNearest")
                    | tag!("MipMapNearestLinear")
                    | tag!("MipMapLinearLinear")
            ),
            |s: &[u8]| match s {
                b"Nearest" => Ok(FilterSetting::Nearest),
                b"Linear" => Ok(FilterSetting::Linear),
                b"MipMap" => Ok(FilterSetting::MipMap),
                b"MipMapNearestNearest" => Ok(FilterSetting::MipMapNearestNearest),
                b"MipMapLinearNearest" => Ok(FilterSetting::MipMapLinearNearest),
                b"MipMapNearestLinear" => Ok(FilterSetting::MipMapNearestLinear),
                b"MipMapLinearLinear" => Ok(FilterSetting::MipMapLinearLinear),
                _ => Err(error_code!(3)),
            }
        )
    );

    named!(
        filter<Filter>,
        do_parse!(
            pair: ws!(separated_pair!(
                call!(filter_setting),
                tag!(","),
                call!(filter_setting)
            )) >> (Filter {
                minification: pair.0,
                magnification: pair.1
            })
        )
    );

    named!(
        repeat_setting<Repeat>,
        map_res!(
            alt!(tag!("x") | tag!("y") | tag!("xy") | tag!("none")),
            |s: &[u8]| match s {
                b"x" => Ok(Repeat::X),
                b"y" => Ok(Repeat::Y),
                b"xy" => Ok(Repeat::XY),
                b"none" => Ok(Repeat::No),
                _ => Err(error_code!(4)),
            }
        )
    );

    named!(
        size<Size>,
        do_parse!(
            pair: ws!(separated_pair!(decimal, tag!(","), decimal))
                >> (Size {
                    width: pair.0 as u64,
                    height: pair.1 as u64
                })
        )
    );

    named!(pub atlas<Atlas>, ws!(do_parse!(
        name: map_res!(call!(not_line_ending), str::from_utf8) >>
        size_p: ws!(separated_pair!(tag!("size"), tag!(":"), size)) >>
        format_p: ws!(separated_pair!(tag!("format"), tag!(":"), header_format)) >>
        filter_p: ws!(separated_pair!(tag!("filter"), tag!(":"), filter)) >>
        repeat_p: ws!(separated_pair!(tag!("repeat"), tag!(":"), repeat_setting)) >>
        regions: fold_many0!(region, Vec::new(), |mut acc: Vec<_>, item| {
            acc.push(item);
            acc
        }) >>

        (Atlas {
            name: name.to_string(),
            size: size_p.1,
            format: format_p.1,
            filter: filter_p.1,
            repeat: repeat_p.1,
            regions,
        })
    )));

    #[cfg(test)]
    mod tests {
        use super::super::*;
        use super::{atlas, region};
        use nom::IResult::Done;

        #[test]
        fn parse_full_atlas() {
            let answer = Atlas {
                name: "BugSpine_tex.png".to_string(),
                size: Size {
                    width: 0,
                    height: 0,
                },
                format: Format::RGBA8888,
                filter: Filter {
                    minification: FilterSetting::Linear,
                    magnification: FilterSetting::Linear,
                },
                repeat: Repeat::No,
                regions: vec![
                    Region {
                        name: "bug_leg".to_string(),
                        rotate: false,
                        xy: Point { x: 1, y: 47 },
                        size: Size {
                            width: 31,
                            height: 70,
                        },
                        orig: Point { x: 35, y: 71 },
                        offset: Point { x: -2, y: 0 },
                        index: -1,
                        split: None,
                        pad: None,
                    },
                    Region {
                        name: "bug_body".to_string(),
                        rotate: false,
                        xy: Point { x: 1, y: 1 },
                        size: Size {
                            width: 62,
                            height: 44,
                        },
                        orig: Point { x: 64, y: 45 },
                        offset: Point { x: -1, y: 0 },
                        index: -1,
                        split: None,
                        pad: None,
                    },
                    Region {
                        name: "bug_eye".to_string(),
                        rotate: false,
                        xy: Point { x: 34, y: 47 },
                        size: Size {
                            width: 19,
                            height: 24,
                        },
                        orig: Point { x: 19, y: 24 },
                        offset: Point { x: 0, y: 0 },
                        index: -1,
                        split: None,
                        pad: None,
                    },
                ],
            };

            let result = atlas(
                b"BugSpine_tex.png
size: 0,0
format: RGBA8888
filter: Linear,Linear
repeat: none
bug_leg
  rotate: false
  xy: 1 ,47
  size: 31 ,70
  orig: 35 ,71
  offset: -2 ,0
  index: -1
bug_body
  rotate: false
  xy: 1 ,1
  size: 62 ,44
  orig: 64 ,45
  offset: -1 ,0
  index: -1
bug_eye
  rotate: false
  xy: 34 ,47
  size: 19 ,24
  orig: 19 ,24
  offset: 0,0
  index: -1",
            );

            assert_eq!(result, Done(&b""[..], answer));
        }

        #[test]
        fn parse_atlas() {
            let answer = Atlas {
                name: "BugSpine_tex.png".to_string(),
                size: Size {
                    width: 0,
                    height: 0,
                },
                format: Format::RGBA8888,
                filter: Filter {
                    minification: FilterSetting::Linear,
                    magnification: FilterSetting::Linear,
                },
                repeat: Repeat::No,
                regions: Vec::new(),
            };
            let result = atlas(
                b"BugSpine_tex.png
  size: 0,0
  format: RGBA8888
  filter: Linear,Linear
  repeat: none",
            );

            assert_eq!(result, Done(&b""[..], answer));
        }

        #[test]
        fn parse_partial_region() {
            let answer = Region {
                name: "bug_body".to_string(),
                rotate: true,
                xy: Point { x: 12, y: 11 },
                size: Size {
                    width: 62,
                    height: 44,
                },
                orig: Point { x: 64, y: 45 },
                offset: Point { x: -1, y: 0 },
                index: -1,
                split: None,
                pad: None,
            };
            let input = b"bug_body
  rotate: true
  xy: 12 ,11
  size: 62 ,44
  orig: 64 ,45
  offset: -1 ,0
  index: -1";

            let result = region(input);

            assert_eq!(result, Done(&b""[..], answer));
        }

        #[test]
        fn parse_full_region() {
            let answer = Region {
                name: "bug_body".to_string(),
                rotate: true,
                xy: Point { x: 12, y: 11 },
                size: Size {
                    width: 62,
                    height: 44,
                },
                orig: Point { x: 64, y: 45 },
                offset: Point { x: -1, y: 0 },
                index: -1,
                split: Some(Split {
                    left: 1,
                    right: 3,
                    top: 5,
                    bottom: 6,
                }),
                pad: Some(Pad {
                    left: 9,
                    right: 7,
                    top: 5,
                    bottom: 3,
                }),
            };
            let input = b"bug_body
  rotate: true
  xy: 12 ,11
  size: 62 ,44
  orig: 64 ,45
  offset: -1 ,0
  index: -1
  split: 1,3,5,6
  pad: 9,7,5,3";

            let result = region(input);

            assert_eq!(result, Done(&b""[..], answer));
        }
    }
}
