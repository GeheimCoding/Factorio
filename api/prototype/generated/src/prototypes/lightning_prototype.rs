#[derive(serde::Deserialize)]
pub struct LightningPrototype {
    base_: crate::prototypes::EntityPrototype,
    #[serde(default = "default_attracted_volume_modifier")]
    attracted_volume_modifier: f32,
    #[serde(default = "default_damage")]
    damage: f64,
    effect_duration: u16,
    // default: Max double
    energy: crate::types::Energy,
    graphics_set: crate::types::LightningGraphicsSet,
    sound: crate::types::Sound,
    source_offset: crate::types::Vector,
    source_variance: crate::types::Vector,
    strike_effect: crate::types::Trigger,
    #[serde(default = "default_time_to_damage")]
    time_to_damage: u16,
}
fn default_attracted_volume_modifier() -> f32 {
    1.0
}
fn default_damage() -> f64 {
    100.0
}
fn default_time_to_damage() -> u16 {
    0
}
