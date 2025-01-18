#[derive(serde::Deserialize)]
pub struct CameraStyleSpecification {
    base_: crate::types::EmptyWidgetStyleSpecification,
    #[serde(rename = "type")]
    type_: String,
}
