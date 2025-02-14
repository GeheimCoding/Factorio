#[derive(Debug, serde::Deserialize)]
pub struct ArtilleryRemoteCapsuleAction {
    flare: crate::types::EntityID,
    #[serde(default = "default_play_sound_on_failure")]
    play_sound_on_failure: bool,
}
fn default_play_sound_on_failure() -> bool {
    true
}
