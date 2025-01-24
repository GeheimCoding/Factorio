#[derive(serde::Deserialize)]
pub struct CheckBoxStyleSpecification {
    base_: crate::types::StyleWithClickableGraphicalSetSpecification,
    checkmark: Option<crate::types::Sprite>,
    disabled_checkmark: Option<crate::types::Sprite>,
    disabled_font_color: Option<crate::types::Color>,
    font: Option<String>,
    font_color: Option<crate::types::Color>,
    intermediate_mark: Option<crate::types::Sprite>,
    text_padding: Option<u32>,
    #[serde(rename = "type")]
    type_: String,
}
