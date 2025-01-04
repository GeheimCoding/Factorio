pub struct DamageTriggerEffectItem {
    apply_damage_to_trees: bool,
    damage: crate::types::DamageParameters,
    lower_damage_modifier: f32,
    lower_distance_threshold: u16,
    type_: Damage,
    upper_damage_modifier: f32,
    upper_distance_threshold: u16,
    use_substitute: bool,
    vaporize: bool,
}
