#[derive(Debug, serde::Deserialize)]
pub enum BlendMode {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "additive")]
    Additive,
    #[serde(rename = "additive_soft")]
    AdditiveSoft,
    #[serde(rename = "multiplicative")]
    Multiplicative,
    #[serde(rename = "multiplicative_with_alpha")]
    MultiplicativeWithAlpha,
    #[serde(rename = "overwrite")]
    Overwrite,
}
