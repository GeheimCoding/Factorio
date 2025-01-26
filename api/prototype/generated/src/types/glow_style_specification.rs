#[derive(Debug, serde::Deserialize)]
pub struct GlowStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    image_set: Option<crate::types::ElementImageSet>,
    #[serde(rename = "type")]
    type_: String,
}
