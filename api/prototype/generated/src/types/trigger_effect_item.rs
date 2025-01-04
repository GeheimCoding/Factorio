pub struct TriggerEffectItem {
    affects_target: bool,
    damage_type_filters: crate::types::DamageTypeFilters,
    probability: f32,
    repeat_count: u16,
    repeat_count_deviation: u16,
    show_in_tooltip: bool,
}
