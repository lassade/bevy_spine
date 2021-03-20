use std::collections::HashMap;
use std::default::Default;
use std::io::{BufReader, Read};

use failure::Error;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Spine {
    pub slots: Vec<Slot>,
    pub skeleton: Skeleton,
    pub animations: HashMap<String, Animation>,
    pub skins: HashMap<String, HashMap<String, HashMap<String, SkinAttachment>>>,
    pub bones: Vec<Bone>,
    #[serde(default)]
    pub transform: Vec<TransformConstraints>,
    #[serde(default)]
    pub ik: Vec<()>,
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
#[serde(rename_all = "camelCase")]
pub struct Skeleton {
    pub hash: Option<String>,
    pub spine: String,
    pub width: Option<f64>,
    pub height: Option<f64>,
    #[serde(default = "default_fps")]
    pub fps: u64,
    pub images: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Bone {
    pub name: String,
    pub parent: Option<String>,
    #[serde(default)]
    pub length: i64,
    #[serde(default)]
    pub transform: InheritTransform,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default)]
    pub rotation: f64,
    #[serde(default = "one_f64")]
    pub scale_y: f64,
    #[serde(default = "one_f64")]
    pub scale_x: f64,
    #[serde(default)]
    pub shear_x: f64,
    #[serde(default)]
    pub shear_y: f64,
    #[serde(default = "yes")]
    pub inherit_scale: bool,
    #[serde(default = "yes")]
    pub inherit_rotation: bool,
    #[serde(default = "bone_default_color")]
    pub color: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub enum SlotBlend {
    Normal,
    Additive,
    Multiply,
    Screen,
}

fn default_fps() -> u64 {
    30
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransformConstraints {
    pub name: String,
    pub order: u64,
    pub bone: String,
    pub target: String,
    #[serde(default)]
    pub rotation: f64,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub scale_y: f64,
    #[serde(default)]
    pub scale_x: f64,
    #[serde(default)]
    pub shear_x: f64,
    #[serde(default)]
    pub shear_y: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default = "one_f64")]
    pub rotate_mix: f64,
    #[serde(default = "one_f64")]
    pub translate_mix: f64,
    #[serde(default = "one_f64")]
    pub scale_mix: f64,
    #[serde(default = "one_f64")]
    pub shear_mix: f64,
    #[serde(default)]
    pub local: bool,
    #[serde(default)]
    pub relative: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PathConstraints {
    pub name: String,
    pub order: u64,
    pub bones: Vec<String>,
    pub target: String,
    #[serde(default)]
    pub position_mode: PositionMode,
    #[serde(default)]
    pub spacing_mode: SpacingMode,
    #[serde(default)]
    pub rotation: f64,
    #[serde(default)]
    pub position: f64,
    #[serde(default)]
    pub spacing: f64,
    #[serde(default = "one_f64")]
    pub rotate_mix: f64,
    #[serde(default = "one_f64")]
    pub translate_mix: f64,
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
pub struct Animation {
    #[serde(default)]
    pub slots: HashMap<String, AnimationSlot>,
    #[serde(default)]
    pub bones: HashMap<String, AnimationBone>,
    #[serde(default)]
    pub transform: HashMap<String, AnimationTransform>,
    #[serde(default)]
    pub deform: HashMap<String, HashMap<String, HashMap<String, AnimationTransform>>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnimationSlot {
    pub attachment: Vec<AttachmentKeyframe>,
    pub color: Vec<ColorKeyframe>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentKeyframe {
    pub time: f64,
    pub name: String,
    pub curve: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ColorKeyframe {
    pub time: f64,
    pub color: String,
    #[serde(default)]
    pub curve: Interpolation,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnimationBone {
    #[serde(default)]
    pub scale: Vec<ScaleKeyframe>,
    #[serde(default)]
    pub rotate: Vec<RotateKeyframe>,
    #[serde(default)]
    pub translate: Vec<TranslateKeyframe>,
    #[serde(default)]
    pub shear: Vec<ShearKeyframe>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScaleKeyframe {
    pub time: f64,
    #[serde(default = "one_f64")]
    pub y: f64,
    #[serde(default = "one_f64")]
    pub x: f64,
    #[serde(default)]
    pub curve: Interpolation,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RotateKeyframe {
    pub time: f64,
    #[serde(default)]
    pub angle: f64,
    #[serde(default)]
    pub curve: Interpolation,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TranslateKeyframe {
    pub time: f64,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default)]
    pub curve: Interpolation,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShearKeyframe {
    pub time: f64,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default)]
    pub curve: Interpolation,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Interpolation {
    Linear,
    Stepped,
    BezierCurve([f64; 4]),
}

impl Default for Interpolation {
    fn default() -> Self {
        Interpolation::Linear
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnimationTransform {
    pub time: f64,
    #[serde(default = "one_f64")]
    pub rotate_mix: f64,
    #[serde(default = "one_f64")]
    pub translate_mix: f64,
    #[serde(default = "one_f64")]
    pub scale_mix: f64,
    #[serde(default = "one_f64")]
    pub shear_mix: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnimationDeform {
    pub time: f64,
    pub vertices: Vec<f64>,
    #[serde(default)]
    pub offset: u64,
    #[serde(default)]
    pub curve: Interpolation,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnimationDrawOrder {
    pub time: f64,
    #[serde(default)]
    pub offsets: Vec<DrawOrderOffset>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DrawOrderOffset {
    pub slot: String,
    pub offset: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum SkinAttachment {
    #[serde(rename_all = "camelCase")]
    Region {
        name: String,
        #[serde(rename = "type")]
        kind: Option<String>,
        path: Option<String>,
        #[serde(default)]
        x: f64,
        #[serde(default)]
        y: f64,
        #[serde(default = "one_f64")]
        scale_x: f64,
        #[serde(default = "one_f64")]
        scale_y: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default)]
        width: u64,
        #[serde(default)]
        height: u64,
        #[serde(default = "white_color")]
        color: String,
    },
    #[serde(rename_all = "camelCase")]
    Mesh {
        #[serde(rename = "type")]
        kind: String,
        name: String,
        path: Option<String>,
        uvs: Vec<[u64; 2]>,
        triangles: Vec<u64>,
        vertices: Vec<[u64; 2]>,
        hull: u64,
        edges: Option<Vec<[u64; 2]>>,
        #[serde(default = "white_color")]
        color: String,
        width: Option<u64>,
        height: Option<u64>,
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
#[serde(rename_all = "camelCase")]
pub struct Event {
    #[serde(default)]
    int: i64,
    #[serde(default)]
    float: f64,
    #[serde(default)]
    string: Option<String>,
}

fn one_f64() -> f64 {
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use serde_json::from_slice;

//     #[test]
//     fn test_serialize() {
//         let file = include_bytes!("../Cowbot/Cowbot.json");

//         from_slice::<Spine>(file).expect("Failed to parse Spine file");
//     }
// }
