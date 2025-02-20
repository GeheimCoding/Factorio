use crate::image::Image;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct BasicMember {
    pub name: String,
    pub order: u16,
    pub description: String,
    pub lists: Option<Vec<String>>,
    pub examples: Option<Vec<String>>,
    pub images: Option<Vec<Image>>,
}
