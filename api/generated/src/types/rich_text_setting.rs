#[derive(Debug, serde::Deserialize)]
pub enum RichTextSetting {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "highlight")]
    Highlight,
}
