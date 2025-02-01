#[derive(Debug, serde::Deserialize)]
pub struct RadioButtonStyleSpecification {
    base_: crate::types::StyleWithClickableGraphicalSetSpecification,
    disabled_font_color: Option<crate::types::Color>,
    font: Option<String>,
    font_color: Option<crate::types::Color>,
    text_padding: Option<u32>,
}
