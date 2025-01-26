#[derive(Debug, serde::Deserialize)]
pub struct TriggerEffectItem {
    #[serde(default = "default_affects_target")]
    affects_target: bool,
    damage_type_filters: Option<crate::types::DamageTypeFilters>,
    #[serde(default = "default_probability")]
    probability: f32,
    #[serde(default = "default_repeat_count")]
    repeat_count: u16,
    #[serde(default = "default_repeat_count_deviation")]
    repeat_count_deviation: u16,
    #[serde(default = "default_show_in_tooltip")]
    show_in_tooltip: bool,
}
fn default_affects_target() -> bool {
    false
}
fn default_probability() -> f32 {
    1.0
}
fn default_repeat_count() -> u16 {
    1
}
fn default_repeat_count_deviation() -> u16 {
    0
}
fn default_show_in_tooltip() -> bool {
    true
}
