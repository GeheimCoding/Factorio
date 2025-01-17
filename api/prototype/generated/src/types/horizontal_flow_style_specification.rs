#[derive(serde::Deserialize)]
pub struct HorizontalFlowStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    horizontal_spacing: i32,
    type_: String,
}
