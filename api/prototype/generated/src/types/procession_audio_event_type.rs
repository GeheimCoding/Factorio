#[derive(Debug, serde::Deserialize)]
pub enum ProcessionAudioEventType {
    #[serde(rename = "play-sound")]
    PlaySound,
    #[serde(rename = "start-looped-sound")]
    StartLoopedSound,
    #[serde(rename = "stop-looped-sound")]
    StopLoopedSound,
}
