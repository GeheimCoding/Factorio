use crate::image::Image;
use crate::type_::Type;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CustomProperty {
    description: String,
    lists: Option<Vec<String>>,
    examples: Option<Vec<String>>,
    images: Option<Vec<Image>>,
    key_type: Type,
    value_type: Type,
}
