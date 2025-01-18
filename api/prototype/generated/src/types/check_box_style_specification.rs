#[derive(serde::Deserialize)]
pub struct CheckBoxStyleSpecification {
    base_: crate::types::StyleWithClickableGraphicalSetSpecification,
    checkmark: crate::types::Sprite,
    disabled_checkmark: crate::types::Sprite,
    disabled_font_color: crate::types::Color,
    font: String,
    font_color: crate::types::Color,
    intermediate_mark: crate::types::Sprite,
    text_padding: u32,
    #[serde(rename = "type")]
    type_: String,
}
