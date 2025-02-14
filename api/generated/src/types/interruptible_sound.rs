#[derive(Debug, serde::Deserialize)]
pub struct InterruptibleSound {
    #[serde(default = "default_fade_ticks")]
    fade_ticks: u32,
    #[serde(default = "default_minimal_change_per_tick")]
    minimal_change_per_tick: f32,
    #[serde(default = "default_minimal_sound_duration_for_stopped_sound")]
    minimal_sound_duration_for_stopped_sound: u16,
    sound: Option<crate::types::Sound>,
    stopped_sound: Option<crate::types::Sound>,
}
fn default_fade_ticks() -> u32 {
    0
}
fn default_minimal_change_per_tick() -> f32 {
    0.0
}
fn default_minimal_sound_duration_for_stopped_sound() -> u16 {
    1
}
