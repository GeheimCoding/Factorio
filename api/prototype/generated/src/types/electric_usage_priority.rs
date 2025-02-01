#[derive(Debug, serde::Deserialize)]
pub enum ElectricUsagePriority {
    #[serde(rename = "primary-input")]
    PrimaryInput,
    #[serde(rename = "primary-output")]
    PrimaryOutput,
    #[serde(rename = "secondary-input")]
    SecondaryInput,
    #[serde(rename = "secondary-output")]
    SecondaryOutput,
    #[serde(rename = "tertiary")]
    Tertiary,
    #[serde(rename = "solar")]
    Solar,
    #[serde(rename = "lamp")]
    Lamp,
}
