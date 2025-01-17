#[derive(serde::Deserialize)]
pub enum ModuleTint {
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "secondary")]
    Secondary,
    #[serde(rename = "tertiary")]
    Tertiary,
    #[serde(rename = "quaternary")]
    Quaternary,
    #[serde(rename = "none")]
    None,
}
