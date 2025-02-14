#[derive(Debug, serde::Deserialize)]
pub struct ChainActiveTriggerPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::ActiveTriggerPrototype,
    action: Option<crate::types::Trigger>,
    #[serde(default = "default_fork_chance")]
    fork_chance: f64,
    #[serde(default = "default_fork_chance_increase_per_quality_level")]
    fork_chance_increase_per_quality_level: f64,
    #[serde(default = "default_jump_delay_ticks")]
    jump_delay_ticks: crate::types::MapTick,
    // default: max uint32
    max_forks: Option<u32>,
    #[serde(default = "default_max_forks_per_jump")]
    max_forks_per_jump: u32,
    #[serde(default = "default_max_jumps")]
    max_jumps: u32,
    // default: infinity
    max_range: Option<f64>,
    #[serde(default = "default_max_range_per_jump")]
    max_range_per_jump: f64,
}
fn default_fork_chance() -> f64 {
    0.0
}
fn default_fork_chance_increase_per_quality_level() -> f64 {
    0.1
}
fn default_jump_delay_ticks() -> crate::types::MapTick {
    0.0
}
fn default_max_forks_per_jump() -> u32 {
    1
}
fn default_max_jumps() -> u32 {
    5
}
fn default_max_range_per_jump() -> f64 {
    5.0
}
