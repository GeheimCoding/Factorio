#[derive(Debug, serde::Deserialize)]
pub struct FlowStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    horizontal_spacing: Option<i32>,
    max_on_row: Option<i32>,
    #[serde(rename = "type")]
    type_: String,
    vertical_spacing: Option<i32>,
}
