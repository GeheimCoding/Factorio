#[derive(Debug, serde::Deserialize)]
pub struct CameraStyleSpecification {
    #[serde(flatten)]
    base_: crate::types::EmptyWidgetStyleSpecification,
}
