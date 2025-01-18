#[derive(serde::Deserialize)]
pub struct ScrollPaneStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    always_draw_borders: bool,
    background_graphical_set: crate::types::ElementImageSet,
    dont_force_clipping_rect_for_contents: bool,
    extra_bottom_margin_when_activated: i32,
    extra_bottom_padding_when_activated: i32,
    extra_left_margin_when_activated: i32,
    extra_left_padding_when_activated: i32,
    extra_margin_when_activated: i32,
    extra_padding_when_activated: i32,
    extra_right_margin_when_activated: i32,
    extra_right_padding_when_activated: i32,
    extra_top_margin_when_activated: i32,
    extra_top_padding_when_activated: i32,
    graphical_set: crate::types::ElementImageSet,
    horizontal_scrollbar_style: crate::types::HorizontalScrollBarStyleSpecification,
    scrollbars_go_outside: bool,
    #[serde(rename = "type")]
    type_: String,
    vertical_flow_style: crate::types::VerticalFlowStyleSpecification,
    vertical_scrollbar_style: crate::types::VerticalScrollBarStyleSpecification,
}
