use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Image {
    pub filename: String,
    pub caption: Option<String>,
}
