#[derive(serde::Deserialize)]
pub enum LightDefinition {
    #[serde(untagged)]
    LightDefinitionStruct(LightDefinitionStruct),
    #[serde(untagged)]
    VecLightDefinitionStruct(Vec<LightDefinitionStruct>),
}
#[derive(serde::Deserialize)]
pub struct LightDefinitionStruct {
    add_perspective: bool,
    color: crate::types::Color,
    flicker_interval: u8,
    flicker_max_modifier: f32,
    flicker_min_modifier: f32,
    intensity: f32,
    minimum_darkness: f32,
    offset_flicker: bool,
    picture: crate::types::Sprite,
    rotation_shift: crate::types::RealOrientation,
    shift: crate::types::Vector,
    size: f32,
    source_orientation_offset: crate::types::RealOrientation,
    #[serde(rename = "type")]
    type_: LightDefinitionType,
}
#[derive(serde::Deserialize)]
pub enum LightDefinitionType {
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "oriented")]
    Oriented,
}
