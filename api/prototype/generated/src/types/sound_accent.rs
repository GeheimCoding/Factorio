#[derive(serde::Deserialize)]
pub struct SoundAccent {
    audible_distance_modifier: f32,
    frame: u16,
    play_for_working_visualisations: Vec<String>,
    sound: crate::types::Sound,
}
