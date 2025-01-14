pub struct SegmentedUnitPrototype {
    base_: crate::prototypes::SegmentPrototype,
    acceleration_rate: f64,
    attack_parameters: crate::types::AttackParameters,
    attacking_speed: f64,
    enraged_duration: u32,
    enraged_speed: f64,
    hurt_roar: crate::types::Sound,
    hurt_thresholds: Vec<f32>,
    investigating_speed: f64,
    patrolling_speed: f64,
    patrolling_turn_radius: f64,
    revenge_attack_parameters: crate::types::AttackParameters,
    roar: crate::types::Sound,
    roar_probability: f32,
    segment_engine: crate::types::SegmentEngineSpecification,
    territory_radius: u32,
    ticks_per_scan: u32,
    turn_radius: f64,
    turn_smoothing: f64,
    vision_distance: f64,
}
