#[derive(Debug, serde::Deserialize)]
pub struct VerticalFlowStyleSpecification {
    #[serde(flatten)]
    base_: crate::types::BaseStyleSpecification,
    vertical_spacing: Option<i32>,
}
