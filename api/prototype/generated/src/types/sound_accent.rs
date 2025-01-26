#[derive(Debug, serde::Deserialize)]
pub struct SoundAccent {
    #[serde(default = "default_audible_distance_modifier")]
    audible_distance_modifier: f32,
    #[serde(default = "default_frame")]
    frame: u16,
    play_for_working_visualisations: Option<Vec<String>>,
    sound: Option<crate::types::Sound>,
}
fn default_audible_distance_modifier() -> f32 {
    1.0
}
fn default_frame() -> u16 {
    0
}
