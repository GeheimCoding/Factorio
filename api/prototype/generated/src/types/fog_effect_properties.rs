#[derive(serde::Deserialize)]
pub struct FogEffectProperties {
    #[serde(rename = "color1")]
    // default: {1, 1, 1, 1}
    color_1: Option<crate::types::Color>,
    #[serde(rename = "color2")]
    // default: {1, 1, 1, 1}
    color_2: Option<crate::types::Color>,
    detail_noise_texture: crate::types::EffectTexture,
    #[serde(default = "default_fog_type")]
    fog_type: FogEffectPropertiesFogType,
    shape_noise_texture: crate::types::EffectTexture,
}
#[derive(serde::Deserialize)]
pub enum FogEffectPropertiesFogType {
    #[serde(rename = "vulcanus")]
    Vulcanus,
    #[serde(rename = "gleba")]
    Gleba,
}
fn default_fog_type() -> FogEffectPropertiesFogType {
    FogEffectPropertiesFogType::Vulcanus
}
