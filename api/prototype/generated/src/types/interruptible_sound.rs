pub struct InterruptibleSound {
    fade_ticks: u32,
    minimal_change_per_tick: f32,
    minimal_sound_duration_for_stopped_sound: u16,
    sound: crate::types::Sound,
    stopped_sound: crate::types::Sound,
}
