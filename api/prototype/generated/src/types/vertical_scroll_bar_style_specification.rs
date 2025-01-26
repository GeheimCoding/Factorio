#[derive(Debug, serde::Deserialize)]
pub struct VerticalScrollBarStyleSpecification {
    base_: crate::types::ScrollBarStyleSpecification,
    #[serde(rename = "type")]
    type_: String,
}
