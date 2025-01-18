#[derive(serde::Deserialize)]
pub struct GlowStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    image_set: crate::types::ElementImageSet,
    #[serde(rename = "type")]
    type_: String,
}
