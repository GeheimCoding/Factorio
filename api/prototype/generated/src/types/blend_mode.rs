#[derive(Debug, serde::Deserialize)]
pub enum BlendMode {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "additive")]
    Additive,
    #[serde(rename = "additive-soft")]
    AdditiveSoft,
    #[serde(rename = "multiplicative")]
    Multiplicative,
    #[serde(rename = "multiplicative-with-alpha")]
    MultiplicativeWithAlpha,
    #[serde(rename = "overwrite")]
    Overwrite,
}
