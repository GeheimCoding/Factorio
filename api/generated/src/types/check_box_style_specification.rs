#[derive(Debug, serde::Deserialize)]
pub struct CheckBoxStyleSpecification {
    #[serde(flatten)]
    base_: crate::types::StyleWithClickableGraphicalSetSpecification,
    checkmark: Option<crate::types::Sprite>,
    disabled_checkmark: Option<crate::types::Sprite>,
    disabled_font_color: Option<crate::types::Color>,
    font: Option<String>,
    font_color: Option<crate::types::Color>,
    intermediate_mark: Option<crate::types::Sprite>,
    text_padding: Option<u32>,
}
