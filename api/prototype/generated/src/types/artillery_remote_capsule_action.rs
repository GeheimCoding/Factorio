#[derive(serde::Deserialize)]
pub struct ArtilleryRemoteCapsuleAction {
    flare: crate::types::EntityID,
    play_sound_on_failure: bool,
    type_: String,
}
