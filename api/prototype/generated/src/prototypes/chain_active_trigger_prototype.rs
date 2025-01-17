#[derive(serde::Deserialize)]
pub struct ChainActiveTriggerPrototype {
    base_: crate::prototypes::ActiveTriggerPrototype,
    action: crate::types::Trigger,
    fork_chance: f64,
    fork_chance_increase_per_quality_level: f64,
    jump_delay_ticks: crate::types::MapTick,
    max_forks: u32,
    max_forks_per_jump: u32,
    max_jumps: u32,
    max_range: f64,
    max_range_per_jump: f64,
}
