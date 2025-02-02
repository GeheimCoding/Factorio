#[derive(Debug, serde::Deserialize)]
pub struct LineStyleSpecification {
    #[serde(flatten)]
    base_: crate::types::BaseStyleSpecification,
    border: Option<crate::types::BorderImageSet>,
}
