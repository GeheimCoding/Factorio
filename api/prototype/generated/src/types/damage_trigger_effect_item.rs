#[derive(serde::Deserialize)]
pub struct DamageTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    apply_damage_to_trees: bool,
    damage: crate::types::DamageParameters,
    lower_damage_modifier: f32,
    lower_distance_threshold: u16,
    #[serde(rename = "type")]
    type_: String,
    upper_damage_modifier: f32,
    upper_distance_threshold: u16,
    use_substitute: bool,
    vaporize: bool,
}
