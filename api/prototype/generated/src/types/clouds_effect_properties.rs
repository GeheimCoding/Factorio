pub struct CloudsEffectProperties {
    additional_density_sample: crate::types::CloudsTextureCoordinateTransformation,
    density: f32,
    density_at_night: f32,
    detail_exponent: f32,
    detail_factor: f32,
    detail_factor_at_night: f32,
    detail_noise_texture: crate::types::EffectTexture,
    detail_sample_1: crate::types::CloudsTextureCoordinateTransformation,
    detail_sample_2: crate::types::CloudsTextureCoordinateTransformation,
    detail_sample_morph_duration: u32,
    movement_speed_multiplier: f32,
    opacity: f32,
    opacity_at_night: f32,
    scale: f32,
    shape_factor: f32,
    shape_noise_texture: crate::types::EffectTexture,
    shape_warp_strength: f32,
    shape_warp_weight: f32,
    warp_sample_1: crate::types::CloudsTextureCoordinateTransformation,
    warp_sample_2: crate::types::CloudsTextureCoordinateTransformation,
    warped_shape_sample: crate::types::CloudsTextureCoordinateTransformation,
}
