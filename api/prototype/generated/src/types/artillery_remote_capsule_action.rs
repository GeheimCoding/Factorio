#[derive(serde::Deserialize)]
pub struct ArtilleryRemoteCapsuleAction {
    flare: crate::types::EntityID,
    #[serde(default = "default_play_sound_on_failure")]
    play_sound_on_failure: bool,
    #[serde(rename = "type")]
    type_: String,
}
fn default_play_sound_on_failure() -> bool {
    true
}
