#[derive(serde::Deserialize)]
pub struct EmptyWidgetStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    graphical_set: crate::types::ElementImageSet,
    type_: String,
}
