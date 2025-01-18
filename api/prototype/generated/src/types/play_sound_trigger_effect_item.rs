#[derive(serde::Deserialize)]
pub struct PlaySoundTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    #[serde(default = "default_audible_distance_modifier")]
    audible_distance_modifier: f32,
    // default: 1e21
    max_distance: f32,
    #[serde(default = "default_min_distance")]
    min_distance: f32,
    #[serde(default = "default_play_on_target_position")]
    play_on_target_position: bool,
    sound: crate::types::Sound,
    #[serde(rename = "type")]
    type_: String,
    #[serde(default = "default_volume_modifier")]
    volume_modifier: f32,
}
fn default_audible_distance_modifier() -> f32 {
    1.0
}
fn default_min_distance() -> f32 {
    0.0
}
fn default_play_on_target_position() -> bool {
    false
}
fn default_volume_modifier() -> f32 {
    1.0
}
