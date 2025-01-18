#[derive(serde::Deserialize)]
pub struct SpaceDustEffectProperties {
    #[serde(default = "default_animation_speed")]
    animation_speed: f32,
    asteroid_normal_texture: crate::types::EffectTexture,
    asteroid_texture: crate::types::EffectTexture,
    noise_texture: crate::types::EffectTexture,
}
fn default_animation_speed() -> f32 {
    1.0
}
