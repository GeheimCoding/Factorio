#[derive(serde::Deserialize)]
pub struct ProcessionAudio {
    catalogue_id: u32,
    looped_sound: crate::types::InterruptibleSound,
    sound: crate::types::Sound,
    #[serde(rename = "type")]
    type_: crate::types::ProcessionAudioType,
}
