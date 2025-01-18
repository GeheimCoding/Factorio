#[derive(serde::Deserialize)]
pub struct HorizontalFlowStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    horizontal_spacing: i32,
    #[serde(rename = "type")]
    type_: String,
}
