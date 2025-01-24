#[derive(serde::Deserialize)]
pub struct TextBoxStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    active_background: Option<crate::types::ElementImageSet>,
    default_background: Option<crate::types::ElementImageSet>,
    disabled_background: Option<crate::types::ElementImageSet>,
    disabled_font_color: Option<crate::types::Color>,
    font: Option<String>,
    font_color: Option<crate::types::Color>,
    game_controller_hovered_background: Option<crate::types::ElementImageSet>,
    rich_text_highlight_error_color: Option<crate::types::Color>,
    rich_text_highlight_ok_color: Option<crate::types::Color>,
    rich_text_highlight_warning_color: Option<crate::types::Color>,
    rich_text_setting: Option<crate::types::RichTextSetting>,
    selected_rich_text_highlight_error_color: Option<crate::types::Color>,
    selected_rich_text_highlight_ok_color: Option<crate::types::Color>,
    selected_rich_text_highlight_warning_color: Option<crate::types::Color>,
    selection_background_color: Option<crate::types::Color>,
    #[serde(rename = "type")]
    type_: String,
}
