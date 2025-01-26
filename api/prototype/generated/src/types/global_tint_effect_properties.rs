#[derive(Debug, serde::Deserialize)]
pub struct GlobalTintEffectProperties {
    global_intensity: f32,
    global_scale: f32,
    intensity: crate::types::Vector4f,
    noise_texture: crate::types::EffectTexture,
    offset: crate::types::Vector4f,
    scale_u: crate::types::Vector4f,
    scale_v: crate::types::Vector4f,
    zoom_factor: f32,
    zoom_intensity: f32,
}
