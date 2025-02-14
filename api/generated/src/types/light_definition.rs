#[derive(Debug, serde::Deserialize)]
pub enum LightDefinition {
    #[serde(untagged)]
    LightDefinitionStruct(LightDefinitionStruct),
    #[serde(untagged)]
    VecLightDefinitionStruct(crate::vec::Vec<LightDefinitionStruct>),
}
#[derive(Debug, serde::Deserialize)]
pub struct LightDefinitionStruct {
    #[serde(default = "default_add_perspective")]
    add_perspective: bool,
    // default: `{r=1, g=1, b=1}`
    color: Option<Box<crate::types::Color>>,
    #[serde(default = "default_flicker_interval")]
    flicker_interval: u8,
    // default: Value of `flicker_min_modifier`
    flicker_max_modifier: Option<f32>,
    #[serde(default = "default_flicker_min_modifier")]
    flicker_min_modifier: f32,
    intensity: f32,
    #[serde(default = "default_minimum_darkness")]
    minimum_darkness: f32,
    #[serde(default = "default_offset_flicker")]
    offset_flicker: bool,
    picture: Option<Box<crate::types::Sprite>>,
    #[serde(default = "default_rotation_shift")]
    rotation_shift: crate::types::RealOrientation,
    shift: Option<Box<crate::types::Vector>>,
    size: f32,
    #[serde(default = "default_source_orientation_offset")]
    source_orientation_offset: crate::types::RealOrientation,
    #[serde(rename = "type")]
    #[serde(default = "default_type_")]
    type_: LightDefinitionType,
}
fn default_add_perspective() -> bool {
    false
}
fn default_flicker_interval() -> u8 {
    0
}
fn default_flicker_min_modifier() -> f32 {
    1.0
}
fn default_minimum_darkness() -> f32 {
    0.0
}
fn default_offset_flicker() -> bool {
    false
}
fn default_rotation_shift() -> crate::types::RealOrientation {
    0.0
}
fn default_source_orientation_offset() -> crate::types::RealOrientation {
    0.0
}
#[derive(Debug, serde::Deserialize)]
pub enum LightDefinitionType {
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "oriented")]
    Oriented,
}
fn default_type_() -> LightDefinitionType {
    LightDefinitionType::Basic
}
