#[derive(serde::Deserialize)]
pub struct LabelStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    clicked_font_color: crate::types::Color,
    disabled_font_color: crate::types::Color,
    font: String,
    font_color: crate::types::Color,
    game_controller_hovered_font_color: crate::types::Color,
    hovered_font_color: crate::types::Color,
    parent_hovered_font_color: crate::types::Color,
    rich_text_highlight_error_color: crate::types::Color,
    rich_text_highlight_ok_color: crate::types::Color,
    rich_text_highlight_warning_color: crate::types::Color,
    rich_text_setting: crate::types::RichTextSetting,
    single_line: bool,
    #[serde(rename = "type")]
    type_: String,
    underlined: bool,
}
