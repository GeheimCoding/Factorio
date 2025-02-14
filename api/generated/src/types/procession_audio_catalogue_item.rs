#[derive(Debug, serde::Deserialize)]
pub struct ProcessionAudioCatalogueItem {
    index: u32,
    looped_sound: Option<crate::types::InterruptibleSound>,
    sound: Option<crate::types::Sound>,
}
