#[derive(Debug, serde::Deserialize)]
pub struct VerticalFlowStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    vertical_spacing: Option<i32>,
}
