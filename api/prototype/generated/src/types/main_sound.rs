#[derive(serde::Deserialize)]
pub struct MainSound {
    activity_to_speed_modifiers: crate::types::ActivityMatchingModifiers,
    activity_to_volume_modifiers: crate::types::ActivityMatchingModifiers,
    #[serde(default = "default_audible_distance_modifier")]
    audible_distance_modifier: f64,
    #[serde(default = "default_fade_in_ticks")]
    fade_in_ticks: u32,
    #[serde(default = "default_fade_out_ticks")]
    fade_out_ticks: u32,
    #[serde(default = "default_match_progress_to_activity")]
    match_progress_to_activity: bool,
    #[serde(default = "default_match_speed_to_activity")]
    match_speed_to_activity: bool,
    #[serde(default = "default_match_volume_to_activity")]
    match_volume_to_activity: bool,
    play_for_working_visualisations: Vec<String>,
    #[serde(default = "default_probability")]
    probability: f64,
    sound: crate::types::Sound,
    #[serde(default = "default_volume_smoothing_window_size")]
    volume_smoothing_window_size: u32,
}
fn default_audible_distance_modifier() -> f64 {
    1.0
}
fn default_fade_in_ticks() -> u32 {
    0
}
fn default_fade_out_ticks() -> u32 {
    0
}
fn default_match_progress_to_activity() -> bool {
    false
}
fn default_match_speed_to_activity() -> bool {
    false
}
fn default_match_volume_to_activity() -> bool {
    false
}
fn default_probability() -> f64 {
    1.0
}
fn default_volume_smoothing_window_size() -> u32 {
    0
}
