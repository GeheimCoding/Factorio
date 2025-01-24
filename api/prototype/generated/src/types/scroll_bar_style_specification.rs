#[derive(serde::Deserialize)]
pub struct ScrollBarStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    background_graphical_set: Option<crate::types::ElementImageSet>,
    thumb_button_style: Option<crate::types::ButtonStyleSpecification>,
}
