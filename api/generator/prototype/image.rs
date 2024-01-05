#![allow(unused)]
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Image {
    /// The name of the image file to display. These files are placed into the `/static/images/` directory.
    filename: String,
    /// The explanatory text to show attached to the image.
    caption: Option<String>,
}
