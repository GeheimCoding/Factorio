#[derive(serde::Deserialize)]
pub struct ProcessionAudio {
    catalogue_id: Option<u32>,
    looped_sound: Option<crate::types::InterruptibleSound>,
    sound: Option<crate::types::Sound>,
    #[serde(rename = "type")]
    type_: crate::types::ProcessionAudioType,
}
