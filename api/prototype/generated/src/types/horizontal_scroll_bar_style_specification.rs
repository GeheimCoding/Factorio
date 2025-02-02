#[derive(Debug, serde::Deserialize)]
pub struct HorizontalScrollBarStyleSpecification {
    #[serde(flatten)]
    base_: crate::types::ScrollBarStyleSpecification,
}
