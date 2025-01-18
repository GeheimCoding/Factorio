#[derive(serde::Deserialize)]
pub struct DamageTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    #[serde(default = "default_apply_damage_to_trees")]
    apply_damage_to_trees: bool,
    damage: crate::types::DamageParameters,
    #[serde(default = "default_lower_damage_modifier")]
    lower_damage_modifier: f32,
    // default: MAX_UINT16
    lower_distance_threshold: u16,
    #[serde(rename = "type")]
    type_: String,
    #[serde(default = "default_upper_damage_modifier")]
    upper_damage_modifier: f32,
    // default: MAX_UINT16
    upper_distance_threshold: u16,
    #[serde(default = "default_use_substitute")]
    use_substitute: bool,
    #[serde(default = "default_vaporize")]
    vaporize: bool,
}
fn default_apply_damage_to_trees() -> bool {
    true
}
fn default_lower_damage_modifier() -> f32 {
    1.0
}
fn default_upper_damage_modifier() -> f32 {
    1.0
}
fn default_use_substitute() -> bool {
    true
}
fn default_vaporize() -> bool {
    false
}
