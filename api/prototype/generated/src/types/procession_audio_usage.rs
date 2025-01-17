#[derive(serde::Deserialize)]
pub enum ProcessionAudioUsage {
    #[serde(rename = "both")]
    Both,
    #[serde(rename = "passenger")]
    Passenger,
    #[serde(rename = "outside")]
    Outside,
}
