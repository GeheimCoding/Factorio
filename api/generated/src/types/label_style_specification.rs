#[derive(Debug, serde::Deserialize)]
pub struct LabelStyleSpecification {
    #[serde(flatten)]
    base_: crate::types::BaseStyleSpecification,
    clicked_font_color: Option<crate::types::Color>,
    disabled_font_color: Option<crate::types::Color>,
    font: Option<String>,
    font_color: Option<crate::types::Color>,
    game_controller_hovered_font_color: Option<crate::types::Color>,
    hovered_font_color: Option<crate::types::Color>,
    parent_hovered_font_color: Option<crate::types::Color>,
    rich_text_highlight_error_color: Option<crate::types::Color>,
    rich_text_highlight_ok_color: Option<crate::types::Color>,
    rich_text_highlight_warning_color: Option<crate::types::Color>,
    rich_text_setting: Option<crate::types::RichTextSetting>,
    single_line: Option<bool>,
    underlined: Option<bool>,
}
