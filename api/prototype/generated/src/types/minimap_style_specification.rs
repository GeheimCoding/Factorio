#[derive(serde::Deserialize)]
pub struct MinimapStyleSpecification {
    base_: crate::types::EmptyWidgetStyleSpecification,
    #[serde(rename = "type")]
    type_: String,
}
