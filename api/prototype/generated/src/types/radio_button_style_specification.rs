#[derive(serde::Deserialize)]
pub struct RadioButtonStyleSpecification {
    base_: crate::types::StyleWithClickableGraphicalSetSpecification,
    disabled_font_color: crate::types::Color,
    font: String,
    font_color: crate::types::Color,
    text_padding: u32,
    #[serde(rename = "type")]
    type_: String,
}
