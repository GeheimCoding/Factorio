#[derive(Debug, serde::Deserialize)]
pub enum MapGenSize {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "very-low")]
    VeryLow,
    #[serde(rename = "very-small")]
    VerySmall,
    #[serde(rename = "very-poor")]
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
    #[serde(rename = "very-high")]
    VeryHigh,
    #[serde(rename = "very-big")]
    VeryBig,
    #[serde(rename = "very-good")]
    VeryGood,
    #[serde(untagged)]
    F32(f32),
}
