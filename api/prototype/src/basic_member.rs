use crate::image::Image;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BasicMember {
    name: String,
    order: u16,
    description: String,
    lists: Option<Vec<String>>,
    examples: Option<Vec<String>>,
    images: Option<Vec<Image>>,
}
