#[derive(Debug, serde::Deserialize)]
pub enum ProcessionAudioType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "sound")]
    Sound,
    #[serde(rename = "looped_sound")]
    LoopedSound,
    #[serde(rename = "pod_catalogue")]
    PodCatalogue,
    #[serde(rename = "location_catalogue")]
    LocationCatalogue,
}
