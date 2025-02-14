#[derive(Debug, serde::Deserialize)]
pub struct SoundAccent {
    #[serde(default = "default_frame")]
    frame: u16,
    play_for_working_visualisation: Option<String>,
    sound: Option<crate::types::Sound>,
}
fn default_frame() -> u16 {
    0
}
