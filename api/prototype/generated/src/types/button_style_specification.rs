#[derive(serde::Deserialize)]
pub struct ButtonStyleSpecification {
    base_: crate::types::StyleWithClickableGraphicalSetSpecification,
    clicked_font_color: crate::types::Color,
    clicked_vertical_offset: u32,
    default_font_color: crate::types::Color,
    disabled_font_color: crate::types::Color,
    draw_grayscale_picture: bool,
    draw_shadow_under_picture: bool,
    font: String,
    hovered_font_color: crate::types::Color,
    icon_horizontal_align: crate::types::HorizontalAlign,
    invert_colors_of_picture_when_disabled: bool,
    invert_colors_of_picture_when_hovered_or_toggled: bool,
    pie_progress_color: crate::types::Color,
    selected_clicked_font_color: crate::types::Color,
    selected_font_color: crate::types::Color,
    selected_hovered_font_color: crate::types::Color,
    strikethrough_color: crate::types::Color,
    #[serde(rename = "type")]
    type_: String,
}
