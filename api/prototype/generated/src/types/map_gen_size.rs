#[derive(Debug, serde::Deserialize)]
pub enum MapGenSize {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "very_low")]
    VeryLow,
    #[serde(rename = "very_small")]
    VerySmall,
    #[serde(rename = "very_poor")]
    VeryPoor,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "poor")]
    Poor,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "big")]
    Big,
    #[serde(rename = "good")]
    Good,
    #[serde(rename = "very_high")]
    VeryHigh,
    #[serde(rename = "very_big")]
    VeryBig,
    #[serde(rename = "very_good")]
    VeryGood,
    #[serde(untagged)]
    F32(f32),
}
