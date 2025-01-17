#[derive(serde::Deserialize)]
pub enum StretchRule {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "stretch_and_expand")]
    StretchAndExpand,
}
