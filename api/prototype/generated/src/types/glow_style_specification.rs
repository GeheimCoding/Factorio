#[derive(Debug, serde::Deserialize)]
pub struct GlowStyleSpecification {
    #[serde(flatten)]
    base_: crate::types::BaseStyleSpecification,
    image_set: Option<crate::types::ElementImageSet>,
}
