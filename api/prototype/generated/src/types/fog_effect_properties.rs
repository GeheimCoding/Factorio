#[derive(serde::Deserialize)]
pub struct FogEffectProperties {
    color1: crate::types::Color,
    color2: crate::types::Color,
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
