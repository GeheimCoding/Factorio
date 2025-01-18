#[derive(serde::Deserialize)]
pub struct CloudsEffectProperties {
    additional_density_sample: crate::types::CloudsTextureCoordinateTransformation,
    #[serde(default = "default_density")]
    density: f32,
    // default: Value of `density`
    density_at_night: f32,
    #[serde(default = "default_detail_exponent")]
    detail_exponent: f32,
    #[serde(default = "default_detail_factor")]
    detail_factor: f32,
    // default: Value of `detail_factor`
    detail_factor_at_night: f32,
    detail_noise_texture: crate::types::EffectTexture,
    detail_sample_1: crate::types::CloudsTextureCoordinateTransformation,
    detail_sample_2: crate::types::CloudsTextureCoordinateTransformation,
    #[serde(default = "default_detail_sample_morph_duration")]
    detail_sample_morph_duration: u32,
    #[serde(default = "default_movement_speed_multiplier")]
    movement_speed_multiplier: f32,
    #[serde(default = "default_opacity")]
    opacity: f32,
    #[serde(default = "default_opacity_at_night")]
    opacity_at_night: f32,
    #[serde(default = "default_scale")]
    scale: f32,
    #[serde(default = "default_shape_factor")]
    shape_factor: f32,
    shape_noise_texture: crate::types::EffectTexture,
    #[serde(default = "default_shape_warp_strength")]
    shape_warp_strength: f32,
    #[serde(default = "default_shape_warp_weight")]
    shape_warp_weight: f32,
    warp_sample_1: crate::types::CloudsTextureCoordinateTransformation,
    warp_sample_2: crate::types::CloudsTextureCoordinateTransformation,
    warped_shape_sample: crate::types::CloudsTextureCoordinateTransformation,
}
fn default_density() -> f32 {
    0.0
}
fn default_detail_exponent() -> f32 {
    0.1
}
fn default_detail_factor() -> f32 {
    1.5
}
fn default_detail_sample_morph_duration() -> u32 {
    256
}
fn default_movement_speed_multiplier() -> f32 {
    0.8
}
fn default_opacity() -> f32 {
    0.2
}
fn default_opacity_at_night() -> f32 {
    0.0
}
fn default_scale() -> f32 {
    1.0
}
fn default_shape_factor() -> f32 {
    -1.0
}
fn default_shape_warp_strength() -> f32 {
    0.1
}
fn default_shape_warp_weight() -> f32 {
    0.4
}
