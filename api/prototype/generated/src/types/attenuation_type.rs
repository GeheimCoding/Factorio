#[derive(Debug, serde::Deserialize)]
pub enum AttenuationType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "logarithmic")]
    Logarithmic,
    #[serde(rename = "exponential")]
    Exponential,
    #[serde(rename = "cosine")]
    Cosine,
    #[serde(rename = "s_curve")]
    SCurve,
}
