#[derive(serde::Deserialize)]
pub struct VerticalFlowStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    #[serde(rename = "type")]
    type_: String,
    vertical_spacing: Option<i32>,
}
