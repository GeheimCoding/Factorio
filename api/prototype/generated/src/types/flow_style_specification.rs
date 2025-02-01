#[derive(Debug, serde::Deserialize)]
pub struct FlowStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    horizontal_spacing: Option<i32>,
    max_on_row: Option<i32>,
    vertical_spacing: Option<i32>,
}
