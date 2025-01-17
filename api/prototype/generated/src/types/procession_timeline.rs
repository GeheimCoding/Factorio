#[derive(serde::Deserialize)]
pub struct ProcessionTimeline {
    audio_events: Vec<crate::types::ProcessionAudioEvent>,
    draw_switch_tick: crate::types::MapTick,
    duration: crate::types::MapTick,
    intermezzo_max_duration: crate::types::MapTick,
    intermezzo_min_duration: crate::types::MapTick,
    layers: Vec<crate::types::ProcessionLayer>,
    special_action_tick: crate::types::MapTick,
}
