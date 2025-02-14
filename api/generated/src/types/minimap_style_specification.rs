#[derive(Debug, serde::Deserialize)]
pub struct MinimapStyleSpecification {
    #[serde(flatten)]
    base_: crate::types::EmptyWidgetStyleSpecification,
}
