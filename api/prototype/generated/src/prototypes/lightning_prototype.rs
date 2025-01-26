#[derive(Debug, serde::Deserialize)]
pub struct LightningPrototype {
    base_: crate::prototypes::EntityPrototype,
    #[serde(default = "default_attracted_volume_modifier")]
    attracted_volume_modifier: f32,
    #[serde(default = "default_damage")]
    damage: f64,
    effect_duration: u16,
    // default: Max double
    energy: Option<crate::types::Energy>,
    graphics_set: Option<crate::types::LightningGraphicsSet>,
    sound: Option<crate::types::Sound>,
    source_offset: Option<crate::types::Vector>,
    source_variance: Option<crate::types::Vector>,
    strike_effect: Option<crate::types::Trigger>,
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
