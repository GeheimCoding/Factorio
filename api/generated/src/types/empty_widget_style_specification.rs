#[derive(Debug, serde::Deserialize)]
pub struct EmptyWidgetStyleSpecification {
    #[serde(flatten)]
    base_: crate::types::BaseStyleSpecification,
    graphical_set: Option<crate::types::ElementImageSet>,
}
