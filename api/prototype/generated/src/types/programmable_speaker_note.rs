#[derive(Debug, serde::Deserialize)]
pub struct ProgrammableSpeakerNote {
    name: String,
    sound: crate::types::Sound,
}
