#[derive(Debug, serde::Deserialize)]
pub enum ProcessionAudioType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "sound")]
    Sound,
    #[serde(rename = "looped-sound")]
    LoopedSound,
    #[serde(rename = "pod-catalogue")]
    PodCatalogue,
    #[serde(rename = "location-catalogue")]
    LocationCatalogue,
}
