#[derive(Debug, serde::Deserialize)]
pub struct SegmentedUnitPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::SegmentPrototype,
    acceleration_rate: f64,
    attack_parameters: Option<crate::types::AttackParameters>,
    attacking_speed: f64,
    enraged_duration: u32,
    enraged_speed: f64,
    hurt_roar: Option<crate::types::Sound>,
    hurt_thresholds: Option<crate::vec::Vec<f32>>,
    investigating_speed: f64,
    patrolling_speed: f64,
    // default: Value of `turn_radius`
    patrolling_turn_radius: Option<f64>,
    revenge_attack_parameters: Option<crate::types::AttackParameters>,
    roar: Option<crate::types::Sound>,
    #[serde(default = "default_roar_probability")]
    roar_probability: f32,
    segment_engine: crate::types::SegmentEngineSpecification,
    territory_radius: u32,
    #[serde(default = "default_ticks_per_scan")]
    ticks_per_scan: u32,
    turn_radius: f64,
    #[serde(default = "default_turn_smoothing")]
    turn_smoothing: f64,
    vision_distance: f64,
}
fn default_roar_probability() -> f32 {
    0.0
}
fn default_ticks_per_scan() -> u32 {
    120
}
fn default_turn_smoothing() -> f64 {
    1.0
}
