#[derive(serde::Deserialize)]
pub struct GameControllerVibrationData {
    #[serde(default = "default_duration")]
    duration: u32,
    #[serde(default = "default_high_frequency_vibration_intensity")]
    high_frequency_vibration_intensity: f32,
    #[serde(default = "default_low_frequency_vibration_intensity")]
    low_frequency_vibration_intensity: f32,
    #[serde(default = "default_play_for")]
    play_for: PlayFor,
}
fn default_duration() -> u32 {
    0
}
fn default_high_frequency_vibration_intensity() -> f32 {
    0.0
}
fn default_low_frequency_vibration_intensity() -> f32 {
    0.0
}
#[derive(serde::Deserialize)]
pub enum PlayFor {
    #[serde(rename = "character_actions")]
    CharacterActions,
    #[serde(rename = "everything")]
    Everything,
}
fn default_play_for() -> PlayFor {
    PlayFor::CharacterActions
}
