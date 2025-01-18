#[derive(serde::Deserialize)]
pub struct HorizontalScrollBarStyleSpecification {
    base_: crate::types::ScrollBarStyleSpecification,
    #[serde(rename = "type")]
    type_: String,
}
