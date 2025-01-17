#[derive(serde::Deserialize)]
pub enum ElectricUsagePriority {
    #[serde(rename = "primary_input")]
    PrimaryInput,
    #[serde(rename = "primary_output")]
    PrimaryOutput,
    #[serde(rename = "secondary_input")]
    SecondaryInput,
    #[serde(rename = "secondary_output")]
    SecondaryOutput,
    #[serde(rename = "tertiary")]
    Tertiary,
    #[serde(rename = "solar")]
    Solar,
    #[serde(rename = "lamp")]
    Lamp,
}
