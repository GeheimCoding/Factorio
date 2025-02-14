#[derive(Debug, serde::Deserialize)]
pub struct WaterTileEffectParameters {
    animation_scale: WaterTileEffectParametersAnimationScale,
    animation_speed: f32,
    dark_threshold: WaterTileEffectParametersDarkThreshold,
    #[serde(default = "default_far_zoom")]
    far_zoom: f32,
    foam_color: crate::types::Color,
    foam_color_multiplier: f32,
    #[serde(default = "default_near_zoom")]
    near_zoom: f32,
    reflection_threshold: WaterTileEffectParametersReflectionThreshold,
    #[serde(default = "default_secondary_texture_variations_columns")]
    secondary_texture_variations_columns: u8,
    #[serde(default = "default_secondary_texture_variations_rows")]
    secondary_texture_variations_rows: u8,
    #[serde(default = "default_shader_variation")]
    shader_variation: crate::types::EffectVariation,
    specular_lightness: crate::types::Color,
    specular_threshold: WaterTileEffectParametersSpecularThreshold,
    #[serde(default = "default_texture_variations_columns")]
    texture_variations_columns: u8,
    #[serde(default = "default_texture_variations_rows")]
    texture_variations_rows: u8,
    textures: crate::vec::Vec<crate::types::EffectTexture>,
    tick_scale: f32,
}
#[derive(Debug, serde::Deserialize)]
pub enum WaterTileEffectParametersAnimationScale {
    #[serde(untagged)]
    F32(f32),
    #[serde(untagged)]
    F32F32((f32, f32)),
}
#[derive(Debug, serde::Deserialize)]
pub enum WaterTileEffectParametersDarkThreshold {
    #[serde(untagged)]
    F32(f32),
    #[serde(untagged)]
    F32F32((f32, f32)),
}
fn default_far_zoom() -> f32 {
    0.5
}
fn default_near_zoom() -> f32 {
    2.0
}
#[derive(Debug, serde::Deserialize)]
pub enum WaterTileEffectParametersReflectionThreshold {
    #[serde(untagged)]
    F32(f32),
    #[serde(untagged)]
    F32F32((f32, f32)),
}
fn default_secondary_texture_variations_columns() -> u8 {
    1
}
fn default_secondary_texture_variations_rows() -> u8 {
    1
}
fn default_shader_variation() -> crate::types::EffectVariation {
    crate::types::EffectVariation::Water
}
#[derive(Debug, serde::Deserialize)]
pub enum WaterTileEffectParametersSpecularThreshold {
    #[serde(untagged)]
    F32(f32),
    #[serde(untagged)]
    F32F32((f32, f32)),
}
fn default_texture_variations_columns() -> u8 {
    1
}
fn default_texture_variations_rows() -> u8 {
    1
}
