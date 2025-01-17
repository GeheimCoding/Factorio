#[derive(serde::Deserialize)]
pub enum ProcessionAudioEventType {
    #[serde(rename = "play_sound")]
    PlaySound,
    #[serde(rename = "start_looped_sound")]
    StartLoopedSound,
    #[serde(rename = "stop_looped_sound")]
    StopLoopedSound,
}
