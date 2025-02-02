#[derive(Debug, serde::Deserialize)]
pub struct ButtonStyleSpecification {
    #[serde(flatten)]
    base_: crate::types::StyleWithClickableGraphicalSetSpecification,
    clicked_font_color: Option<crate::types::Color>,
    clicked_vertical_offset: Option<u32>,
    default_font_color: Option<crate::types::Color>,
    disabled_font_color: Option<crate::types::Color>,
    draw_grayscale_picture: Option<bool>,
    draw_shadow_under_picture: Option<bool>,
    font: Option<String>,
    hovered_font_color: Option<crate::types::Color>,
    icon_horizontal_align: Option<crate::types::HorizontalAlign>,
    invert_colors_of_picture_when_disabled: Option<bool>,
    invert_colors_of_picture_when_hovered_or_toggled: Option<bool>,
    pie_progress_color: Option<crate::types::Color>,
    selected_clicked_font_color: Option<crate::types::Color>,
    selected_font_color: Option<crate::types::Color>,
    selected_hovered_font_color: Option<crate::types::Color>,
    strikethrough_color: Option<crate::types::Color>,
}
