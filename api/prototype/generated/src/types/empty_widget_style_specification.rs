#[derive(serde::Deserialize)]
pub struct EmptyWidgetStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    graphical_set: Option<crate::types::ElementImageSet>,
    #[serde(rename = "type")]
    type_: String,
}
