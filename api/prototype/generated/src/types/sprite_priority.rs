#[derive(Debug, serde::Deserialize)]
pub enum SpritePriority {
    #[serde(rename = "extra_high_no_scale")]
    ExtraHighNoScale,
    #[serde(rename = "extra_high")]
    ExtraHigh,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "very_low")]
    VeryLow,
    #[serde(rename = "no_atlas")]
    NoAtlas,
}
