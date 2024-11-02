use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Image {
    filename: String,
    caption: Option<String>,
}
