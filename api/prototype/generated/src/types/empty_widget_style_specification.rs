#[derive(serde::Deserialize)]
pub struct EmptyWidgetStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    graphical_set: crate::types::ElementImageSet,
    #[serde(rename = "type")]
    type_: String,
}
