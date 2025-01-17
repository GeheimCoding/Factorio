#[derive(serde::Deserialize)]
pub struct GlowStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    image_set: crate::types::ElementImageSet,
    type_: String,
}
