#[derive(Debug, serde::Deserialize)]
pub struct FogEffectProperties {
    // default: {1, 1, 1, 1}
    #[serde(rename = "color1")]
    color_1: Option<crate::types::Color>,
    // default: {1, 1, 1, 1}
    #[serde(rename = "color2")]
    color_2: Option<crate::types::Color>,
    detail_noise_texture: crate::types::EffectTexture,
    #[serde(default = "default_fog_type")]
    fog_type: FogEffectPropertiesFogType,
    shape_noise_texture: crate::types::EffectTexture,
    #[serde(default = "default_tick_factor")]
    tick_factor: f32,
}
#[derive(Debug, serde::Deserialize)]
pub enum FogEffectPropertiesFogType {
    #[serde(rename = "vulcanus")]
    Vulcanus,
    #[serde(rename = "gleba")]
    Gleba,
}
fn default_fog_type() -> FogEffectPropertiesFogType {
    FogEffectPropertiesFogType::Vulcanus
}
fn default_tick_factor() -> f32 {
    0.0
}
