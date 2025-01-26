#[derive(Debug, serde::Deserialize)]
pub struct ProcessionTimeline {
    audio_events: Vec<crate::types::ProcessionAudioEvent>,
    // default: 1/2 of duration
    draw_switch_tick: Option<crate::types::MapTick>,
    duration: crate::types::MapTick,
    #[serde(default = "default_intermezzo_max_duration")]
    intermezzo_max_duration: crate::types::MapTick,
    #[serde(default = "default_intermezzo_min_duration")]
    intermezzo_min_duration: crate::types::MapTick,
    layers: Vec<crate::types::ProcessionLayer>,
    // default: 1/2 of duration
    special_action_tick: Option<crate::types::MapTick>,
}
fn default_intermezzo_max_duration() -> crate::types::MapTick {
    0
}
fn default_intermezzo_min_duration() -> crate::types::MapTick {
    0
}
