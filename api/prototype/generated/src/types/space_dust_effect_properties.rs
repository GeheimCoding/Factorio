#[derive(serde::Deserialize)]
pub struct SpaceDustEffectProperties {
    animation_speed: f32,
    asteroid_normal_texture: crate::types::EffectTexture,
    asteroid_texture: crate::types::EffectTexture,
    noise_texture: crate::types::EffectTexture,
}
