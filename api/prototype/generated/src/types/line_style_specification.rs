#[derive(Debug, serde::Deserialize)]
pub struct LineStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    border: Option<crate::types::BorderImageSet>,
}
