#[derive(serde::Deserialize)]
pub enum BuildMode {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "forced")]
    Forced,
    #[serde(rename = "superforced")]
    Superforced,
}
