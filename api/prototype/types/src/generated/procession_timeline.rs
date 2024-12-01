pub struct ProcessionTimeline {
    audio_events: Vec<ProcessionAudioEvent>,
    draw_switch_tick: MapTick,
    duration: MapTick,
    intermezzo_max_duration: MapTick,
    intermezzo_min_duration: MapTick,
    layers: Vec<ProcessionLayer>,
    special_action_tick: MapTick,
}
