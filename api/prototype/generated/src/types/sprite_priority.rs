#[derive(Debug, serde::Deserialize)]
pub enum SpritePriority {
    #[serde(rename = "extra-high-no-scale")]
    ExtraHighNoScale,
    #[serde(rename = "extra-high")]
    ExtraHigh,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "very-low")]
    VeryLow,
    #[serde(rename = "no-atlas")]
    NoAtlas,
}
