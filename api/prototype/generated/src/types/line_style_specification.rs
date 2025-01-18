#[derive(serde::Deserialize)]
pub struct LineStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    border: crate::types::BorderImageSet,
    #[serde(rename = "type")]
    type_: String,
}
