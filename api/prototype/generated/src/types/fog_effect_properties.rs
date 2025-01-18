#[derive(serde::Deserialize)]
pub struct FogEffectProperties {
    #[serde(rename = "color1")]
    color_1: crate::types::Color,
    #[serde(rename = "color2")]
    color_2: crate::types::Color,
    detail_noise_texture: crate::types::EffectTexture,
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
