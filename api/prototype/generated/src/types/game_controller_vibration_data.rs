#[derive(serde::Deserialize)]
pub struct GameControllerVibrationData {
    duration: u32,
    high_frequency_vibration_intensity: f32,
    low_frequency_vibration_intensity: f32,
    play_for: PlayFor,
}
#[derive(serde::Deserialize)]
pub enum PlayFor {
    #[serde(rename = "character_actions")]
    CharacterActions,
    #[serde(rename = "everything")]
    Everything,
}
