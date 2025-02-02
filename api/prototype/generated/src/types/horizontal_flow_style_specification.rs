#[derive(Debug, serde::Deserialize)]
pub struct HorizontalFlowStyleSpecification {
    #[serde(flatten)]
    base_: crate::types::BaseStyleSpecification,
    horizontal_spacing: Option<i32>,
}
