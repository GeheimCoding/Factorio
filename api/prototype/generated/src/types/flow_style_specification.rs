#[derive(serde::Deserialize)]
pub struct FlowStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    horizontal_spacing: i32,
    max_on_row: i32,
    #[serde(rename = "type")]
    type_: String,
    vertical_spacing: i32,
}
