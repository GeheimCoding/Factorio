#[derive(serde::Deserialize)]
pub struct ThrusterGraphicsSet {
    base_: crate::types::WorkingVisualisations,
    flame: crate::types::Sprite,
    flame_effect: crate::types::EffectTexture,
    #[serde(default = "default_flame_effect_height")]
    flame_effect_height: f32,
    #[serde(default = "default_flame_effect_offset")]
    flame_effect_offset: f32,
    #[serde(default = "default_flame_effect_width")]
    flame_effect_width: f32,
    #[serde(default = "default_flame_half_height")]
    flame_half_height: f32,
    flame_position: crate::types::Vector,
}
fn default_flame_effect_height() -> f32 {
    31.2
}
fn default_flame_effect_offset() -> f32 {
    1.6
}
fn default_flame_effect_width() -> f32 {
    6.0
}
fn default_flame_half_height() -> f32 {
    0.0
}
