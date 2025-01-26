#[derive(Debug, serde::Deserialize)]
pub struct HorizontalFlowStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    horizontal_spacing: Option<i32>,
    #[serde(rename = "type")]
    type_: String,
}
