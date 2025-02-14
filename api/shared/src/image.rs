use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Image {
    pub filename: String,
    pub caption: Option<String>,
}
