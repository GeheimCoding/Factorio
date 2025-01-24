#[derive(serde::Deserialize)]
pub struct ImageStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    graphical_set: Option<crate::types::ElementImageSet>,
    invert_colors_of_picture_when_hovered_or_toggled: Option<bool>,
    stretch_image_to_widget_size: Option<bool>,
    #[serde(rename = "type")]
    type_: String,
}
