#[derive(Debug, serde::Deserialize)]
pub struct VerticalScrollBarStyleSpecification {
    #[serde(flatten)]
    base_: crate::types::ScrollBarStyleSpecification,
}
