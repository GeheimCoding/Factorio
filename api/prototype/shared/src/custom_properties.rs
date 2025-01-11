use crate::image::Image;
use crate::type_::Type;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CustomProperties {
    pub description: String,
    pub lists: Option<Vec<String>>,
    pub examples: Option<Vec<String>>,
    pub images: Option<Vec<Image>>,
    pub key_type: Type,
    pub value_type: Type,
}
