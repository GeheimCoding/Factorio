#[derive(Debug, serde::Deserialize)]
pub struct PlaySoundTriggerEffectItem {
    #[serde(flatten)]
    base_: crate::types::TriggerEffectItem,
    // default: 1e21
    max_distance: Option<f32>,
    #[serde(default = "default_min_distance")]
    min_distance: f32,
    #[serde(default = "default_play_on_target_position")]
    play_on_target_position: bool,
    sound: crate::types::Sound,
}
fn default_min_distance() -> f32 {
    0.0
}
fn default_play_on_target_position() -> bool {
    false
}
