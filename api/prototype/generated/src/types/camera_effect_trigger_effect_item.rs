#[derive(Debug, serde::Deserialize)]
pub struct CameraEffectTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    #[serde(default = "default_delay")]
    delay: u8,
    duration: u8,
    #[serde(default = "default_ease_in_duration")]
    ease_in_duration: u8,
    #[serde(default = "default_ease_out_duration")]
    ease_out_duration: u8,
    #[serde(default = "default_full_strength_max_distance")]
    full_strength_max_distance: u16,
    #[serde(default = "default_max_distance")]
    max_distance: u16,
    #[serde(default = "default_strength")]
    strength: f32,
}
fn default_delay() -> u8 {
    0
}
fn default_ease_in_duration() -> u8 {
    0
}
fn default_ease_out_duration() -> u8 {
    0
}
fn default_full_strength_max_distance() -> u16 {
    0
}
fn default_max_distance() -> u16 {
    0
}
fn default_strength() -> f32 {
    0.0
}
