pub struct WaterTileEffectParameters {
    animation_scale: WaterTileEffectParametersAnimationScale,
    animation_speed: f32,
    dark_threshold: WaterTileEffectParametersDarkThreshold,
    far_zoom: f32,
    foam_color: crate::types::Color,
    foam_color_multiplier: f32,
    near_zoom: f32,
    reflection_threshold: WaterTileEffectParametersReflectionThreshold,
    secondary_texture_variations_columns: u8,
    secondary_texture_variations_rows: u8,
    shader_variation: crate::types::EffectVariation,
    specular_lightness: crate::types::Color,
    specular_threshold: WaterTileEffectParametersSpecularThreshold,
    texture_variations_columns: u8,
    texture_variations_rows: u8,
    textures: Vec<crate::types::EffectTexture>,
    tick_scale: f32,
}
pub enum WaterTileEffectParametersAnimationScale {
    F32(f32),
    F32F32((f32, f32)),
}
pub enum WaterTileEffectParametersDarkThreshold {
    F32(f32),
    F32F32((f32, f32)),
}
pub enum WaterTileEffectParametersReflectionThreshold {
    F32(f32),
    F32F32((f32, f32)),
}
pub enum WaterTileEffectParametersSpecularThreshold {
    F32(f32),
    F32F32((f32, f32)),
}
