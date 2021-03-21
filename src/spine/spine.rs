use std::{collections::HashMap};
use std::default::Default;
use std::io::{BufReader, Read};

use anyhow::Error;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Spine {
    pub slots: Vec<Slot>,
    pub skeleton: Skeleton,
    pub animations: HashMap<String, Animation>,
    pub skins: Vec<Skin>,
    pub bones: Vec<Bone>,
    #[serde(default)]
    pub transform: Vec<TransformConstraints>,
    #[serde(default)]
    pub ik: Vec<IkConstraints>,
    #[serde(default)]
    pub path: Vec<PathConstraints>,
    #[serde(default)]
    pub events: HashMap<String, Event>,
}

impl Spine {
    pub fn parse<R: Read>(reader: R) -> Result<Spine, Error> {
        let r = BufReader::new(reader);
        let spine: Spine = from_reader(r)?;
        Ok(spine)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Skeleton {
    pub hash: Option<String>,
    pub spine: String,
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
    pub width: f32,
    pub height: f32,
    #[serde(default = "default_fps")]
    pub fps: u32,
    pub images: String,
    pub audio: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Bone {
    pub name: String,
    pub parent: Option<String>,
    #[serde(default)]
    pub length: f32,
    #[serde(default)]
    pub transform: InheritTransform,
    #[serde(default)]
    pub skin: bool,
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
    #[serde(default)]
    pub rotation: f32,
    #[serde(default = "one_f32")]
    pub scale_y: f32,
    #[serde(default = "one_f32")]
    pub scale_x: f32,
    #[serde(default)]
    pub shear_x: f32,
    #[serde(default)]
    pub shear_y: f32,
    #[serde(default = "yes")]
    pub inherit_scale: bool,
    #[serde(default = "yes")]
    pub inherit_rotation: bool,
    #[serde(default = "bone_default_color")]
    pub color: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Slot {
    pub name: String,
    pub bone: String,
    #[serde(default = "white_color")]
    pub color: String,
    pub dark: Option<String>,
    pub attachment: Option<String>,
    pub blend: Option<SlotBlend>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum SlotBlend {
    Normal,
    Additive,
    Multiply,
    Screen,
}

fn default_fps() -> u32 {
    30
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TransformConstraints {
    pub name: String,
    #[serde(default)]
    pub order: u32,
    #[serde(default)]
    pub skin: bool,
    pub bones: Vec<String>,
    pub target: String,
    #[serde(default)]
    pub rotation: f32,
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub scale_y: f32,
    #[serde(default)]
    pub scale_x: f32,
    #[serde(default)]
    pub shear_x: f32,
    #[serde(default)]
    pub shear_y: f32,
    #[serde(default)]
    pub y: f32,
    #[serde(default = "one_f32")]
    pub rotate_mix: f32,
    #[serde(default = "one_f32")]
    pub translate_mix: f32,
    #[serde(default = "one_f32")]
    pub scale_mix: f32,
    #[serde(default = "one_f32")]
    pub shear_mix: f32,
    #[serde(default)]
    pub local: bool,
    #[serde(default)]
    pub relative: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PathConstraints {
    pub name: String,
    #[serde(default)]
    pub order: usize,
    #[serde(default)]
    pub skin: bool,
    pub bones: Vec<String>,
    pub target: String,
    #[serde(default)]
    pub position_mode: PositionMode,
    #[serde(default)]
    pub spacing_mode: SpacingMode,
    #[serde(default)]
    pub rotate_mode: RotateMode,
    #[serde(default)]
    pub rotation: f32,
    #[serde(default)]
    pub position: f32,
    #[serde(default)]
    pub spacing: f32,
    #[serde(default = "one_f32")]
    pub rotate_mix: f32,
    #[serde(default = "one_f32")]
    pub translate_mix: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum PositionMode {
    Fixed,
    Percent,
}

impl Default for PositionMode {
    fn default() -> Self {
        PositionMode::Percent
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SpacingMode {
    Length,
    Fixed,
    Percent,
}

impl Default for SpacingMode {
    fn default() -> Self {
        SpacingMode::Length
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum RotateMode {
    Tangent,
    Chain,
    ChainScale,
}

impl Default for RotateMode {
    fn default() -> Self {
        RotateMode::Tangent
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct Animation {
    pub slots: HashMap<String, AnimationSlot>,
    pub bones: HashMap<String, AnimationBone>,
    pub ik: HashMap<String, Vec<AnimationIk>>,
    pub transform: HashMap<String, Vec<AnimationTransform>>,
    pub path: HashMap<String, AnimationPath>,
    pub deform: HashMap<String, HashMap<String, HashMap<String, Vec<AnimationDeform>>>>,
    pub draw_order: Vec<AnimationDrawOrder>,
    pub events: Vec<AnimationEvent>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
#[serde(default, deny_unknown_fields,  rename_all = "camelCase")]
pub struct AnimationSlot {
    pub attachment: Vec<AttachmentKeyframe>,
    pub color: Vec<ColorKeyframe>,
    pub two_color: Vec<TwoColorKeyframe>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct AttachmentKeyframe {
    pub time: f32,
    pub name: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct ColorKeyframe {
    pub time: f32,
    pub color: String,
    #[serde(flatten, with = "keyframe_interpolation")]
    pub curve: Interpolation,
}
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct TwoColorKeyframe {
    pub time: f32,
    pub light: String,
    pub dark: String,
    #[serde(flatten, with = "keyframe_interpolation")]
    pub curve: Interpolation,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct AnimationBone {
    pub scale: Vec<ScaleKeyframe>,
    pub rotate: Vec<RotateKeyframe>,
    pub translate: Vec<TranslateKeyframe>,
    pub shear: Vec<ShearKeyframe>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct ScaleKeyframe {
    pub time: f32,
    pub y: f32,
    pub x: f32,
    #[serde(flatten, with = "keyframe_interpolation")]
    pub curve: Interpolation,
}

impl Default for ScaleKeyframe {
    fn default() -> Self {
        Self {
            time: 0.0,
            y: 1.0,
            x: 1.0,
            curve: Default::default(),
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct RotateKeyframe {
    pub time: f32,
    pub angle: f32,
    #[serde(flatten, with = "keyframe_interpolation")]
    pub curve: Interpolation,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct TranslateKeyframe {
    pub time: f32,
    pub x: f32,
    pub y: f32,
    #[serde(flatten, with = "keyframe_interpolation")]
    pub curve: Interpolation,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct ShearKeyframe {
    pub time: f32,
    pub x: f32,
    pub y: f32,
    #[serde(flatten, with = "keyframe_interpolation")]
    pub curve: Interpolation,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Interpolation {
    Linear,
    Stepped,
    BezierCurve([f32; 4]),
}

impl Default for Interpolation {
    fn default() -> Self {
        Interpolation::Linear
    }
}

mod keyframe_interpolation {
    use std::fmt;

    use super::*;
    use serde::{
        self,
        de::{Error, MapAccess, Visitor},
        ser::SerializeMap,
        Deserializer, Serializer,
    };

    pub fn serialize<S>(interpolation: &Interpolation, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match interpolation {
            // Should be skipped
            Interpolation::Linear => unreachable!(),
            Interpolation::Stepped => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("curve", "stepped")?;
                map.end()
            }
            Interpolation::BezierCurve(p) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("curve", &p[0])?;
                map.serialize_entry("c2", &p[1])?;
                map.serialize_entry("c3", &p[2])?;
                map.serialize_entry("c4", &p[3])?;
                map.end()
            }
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Interpolation, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct InterpolationVisitor;

        impl<'de> Visitor<'de> for InterpolationVisitor {
            type Value = Interpolation;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("not a valid format interpolation format")
            }

            #[allow(unused_assignments)]
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut kind = 0;
                let mut curve = [0.0, 0.0, 1.0, 1.0];

                while let Some(name) = map.next_key::<String>()? {
                    match name.as_str() {
                        "curve" => match map.next_value::<serde_json::Value>()? {
                            serde_json::Value::Number(n) => {
                                curve[0] = n.as_f64().unwrap_or(0.0) as f32;
                                kind = 2;
                            }
                            serde_json::Value::String(s) => {
                                if s == "stepped" {
                                    kind = 1;
                                } else {
                                    Err(A::Error::custom(format!(
                                        "invalid curve type \"{}\", expected: \"stepped\"",
                                        s
                                    )))?
                                }
                            }
                            v => 
                            Err(A::Error::custom(format!(
                                "invalid curve format \"{:?}\", expected: `Value::String` or `Value::Number`",
                                v
                            )))?,
                        },
                        "c2" => curve[1] = map.next_value()?,
                        "c3" => curve[2] = map.next_value()?,
                        "c4" => curve[3] = map.next_value()?,
                        k => Err(A::Error::unknown_field(k, &["curve", "c2", "c3", "c4"]))?,
                    }
                }

                match kind {
                    0 => Ok(Interpolation::Linear),
                    1 => Ok(Interpolation::Stepped),
                    2 => Ok(Interpolation::BezierCurve(curve)),
                    _ => unreachable!(),
                }
            }
        }

        deserializer.deserialize_map(InterpolationVisitor)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct AnimationIk {
    /// The time in seconds for the keyframe.
    pub time: f32,
    /// The IK constraint mix for the keyframe. Assume 1 if omitted.
    pub mix: f32,
    /// A value for two bone IK, the distance from the maximum reach of the bones that rotation will slow. Assume 0 if omitted.
    pub softness: f32,
    /// The IK constraint bend direction for the keyframe. Assume false if omitted.
    pub bend_positive: bool,
    /// If true, and only a single bone is being constrained, if the target is too close, the bone is scaled to reach it. Assume false if omitted.
    pub compress: bool,
    /// If true, and if the target is out of range, the parent bone is scaled to reach it. If more than one bone is being constrained and the parent bone has local nonuniform scale, stretch is not applied. Assume false if omitted.
    pub stretch: bool,
}

impl Default for AnimationIk {
    fn default() -> Self {
        Self {
            time: 0.0,
            mix: 1.0,
            softness: 0.0,
            bend_positive: false,
            compress: false,
            stretch: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct AnimationTransform {
    pub time: f32,
    pub rotate_mix: f32,
    pub translate_mix: f32,
    pub scale_mix: f32,
    pub shear_mix: f32,
    #[serde(flatten, with = "keyframe_interpolation")]
    pub curve: Interpolation,
}

impl Default for AnimationTransform {
    fn default() -> Self {
        Self {
            time: 0.0,
            rotate_mix: 1.0,
            translate_mix: 1.0,
            scale_mix: 1.0,
            shear_mix: 1.0,
            curve: Default::default(),
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct AnimationPath {
    pub position: Vec<PathPositionKeyframe>,
    pub spacing: Vec<PathSpacingKeyframe>,
    pub mix: Vec<PathMixKeyframe>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct PathPositionKeyframe {
    pub time: f32,
    pub position: f32,
    #[serde(flatten, with = "keyframe_interpolation")]
    pub curve: Interpolation,
}

impl Default for PathPositionKeyframe {
    fn default() -> Self {
        Self {
            time: 0.0,
            position: 1.0,
            curve: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct PathSpacingKeyframe {
    pub time: f32,
    pub spacing: f32,
    #[serde(flatten, with = "keyframe_interpolation")]
    pub curve: Interpolation,
}

impl Default for PathSpacingKeyframe {
    fn default() -> Self {
        Self {
            time: 0.0,
            spacing: 1.0,
            curve: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct PathMixKeyframe {
    pub time: f32,
    pub rotate_mix: f32,
    pub translate_mix: f32,
    #[serde(flatten, with = "keyframe_interpolation")]
    pub curve: Interpolation,
}

impl Default for PathMixKeyframe {
    fn default() -> Self {
        Self {
            time: 0.0,
            rotate_mix: 1.0,
            translate_mix: 1.0,
            curve: Default::default(),
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct AnimationDeform {
    pub time: f32,
    pub vertices: Vec<f32>,
    pub offset: i32,
    #[serde(flatten, with = "keyframe_interpolation")]
    pub curve: Interpolation,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AnimationDrawOrder {
    pub time: f32,
    #[serde(default)]
    pub offsets: Vec<DrawOrderOffset>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DrawOrderOffset {
    pub slot: String,
    pub offset: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct AnimationEvent {
    pub name: Option<String>,
    pub time: f32,
    pub int: isize,
    pub float: f32,
    pub string: Option<String>,
    pub audio: Option<String>,
    pub volume: f32,
    pub balance: f32,
}

impl Default for AnimationEvent {
    fn default() -> Self {
        Self {
            name: None,
            time: 0.0,
            int: 0,
            float: 0.0,
            string: None,
            audio: None,
            volume: 1.0,
            balance: 0.0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Skin {
    pub name: String,
    #[serde(default)]
    pub bones: Vec<String>,
    #[serde(default)]
    pub ik: Vec<String>,
    #[serde(default)]
    pub transform: Vec<String>,
    #[serde(default)]
    pub path: Vec<String>,
    #[serde(default)]
    pub attachments: HashMap<String, HashMap<String, SkinAttachment>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged, rename_all = "camelCase")]
pub enum SkinAttachment {
    // "tag" is omitted 
    #[serde(rename_all = "camelCase")]
    Region {
        path: Option<String>,
        #[serde(default)]
        x: f32,
        #[serde(default)]
        y: f32,
        #[serde(default = "one_f32")]
        scale_x: f32,
        #[serde(default = "one_f32")]
        scale_y: f32,
        #[serde(default)]
        rotation: f32,
        #[serde(default)]
        width: u32,
        #[serde(default)]
        height: u32,
        #[serde(default = "white_color")]
        color: String,
    },
    // "tag": "mesh"
    #[serde(rename_all = "camelCase")]
    Mesh {
        path: Option<String>,
        uvs: Vec<f32>,
        triangles: Vec<u32>,
        vertices: Vec<f32>,
        hull: u32,
        edges: Option<Vec<u32>>,
        #[serde(default = "white_color")]
        color: String,
        width: Option<u32>,
        height: Option<u32>,
    },
    // TODO: implement other variants
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum InheritTransform {
    Normal,
    OnlyTranslation,
    NoRotationOrReflection,
    NoScale,
    NoScaleOrReflection,
}

impl Default for InheritTransform {
    fn default() -> Self {
        InheritTransform::Normal
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct IkConstraints {
    pub name: String,
    #[serde(default)]
    pub order: usize,
    pub bones: Vec<String>,
    pub target: String,
    #[serde(default = "one_f32")]
    pub mix: f32,
    #[serde(default)]
    pub bend_positive: bool,
    #[serde(default)]
    pub compress: bool,
    #[serde(default)]
    pub stretch: bool,
    #[serde(default)]
    pub uniform: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Event {
    #[serde(default)]
    int: i64,
    #[serde(default)]
    float: f32,
    #[serde(default)]
    string: Option<String>,
}

fn one_f32() -> f32 {
    1.0
}

fn yes() -> bool {
    true
}

fn bone_default_color() -> String {
    "989898FF".to_string()
}

fn white_color() -> String {
    "FFFFFFFF".to_owned()
}
