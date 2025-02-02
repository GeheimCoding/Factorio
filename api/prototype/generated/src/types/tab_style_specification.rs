#[derive(Debug, serde::Deserialize)]
pub struct TabStyleSpecification {
    #[serde(flatten)]
    base_: crate::types::StyleWithClickableGraphicalSetSpecification,
    badge_font: Option<String>,
    badge_horizontal_spacing: Option<i16>,
    default_badge_font_color: Option<crate::types::Color>,
    default_badge_graphical_set: Option<crate::types::ElementImageSet>,
    default_font_color: Option<crate::types::Color>,
    disabled_badge_font_color: Option<crate::types::Color>,
    disabled_badge_graphical_set: Option<crate::types::ElementImageSet>,
    disabled_font_color: Option<crate::types::Color>,
    draw_grayscale_picture: Option<bool>,
    font: Option<String>,
    hover_badge_graphical_set: Option<crate::types::ElementImageSet>,
    increase_height_when_selected: Option<bool>,
    left_edge_selected_graphical_set: Option<crate::types::ElementImageSet>,
    override_graphics_on_edges: Option<bool>,
    press_badge_graphical_set: Option<crate::types::ElementImageSet>,
    right_edge_selected_graphical_set: Option<crate::types::ElementImageSet>,
    selected_badge_font_color: Option<crate::types::Color>,
    selected_badge_graphical_set: Option<crate::types::ElementImageSet>,
    selected_font_color: Option<crate::types::Color>,
}
