#[derive(Debug, serde::Deserialize)]
pub struct ScrollPaneStyleSpecification {
    #[serde(flatten)]
    base_: crate::types::BaseStyleSpecification,
    always_draw_borders: Option<bool>,
    background_graphical_set: Option<crate::types::ElementImageSet>,
    dont_force_clipping_rect_for_contents: Option<bool>,
    extra_bottom_margin_when_activated: Option<i32>,
    extra_bottom_padding_when_activated: Option<i32>,
    extra_left_margin_when_activated: Option<i32>,
    extra_left_padding_when_activated: Option<i32>,
    extra_margin_when_activated: Option<i32>,
    extra_padding_when_activated: Option<i32>,
    extra_right_margin_when_activated: Option<i32>,
    extra_right_padding_when_activated: Option<i32>,
    extra_top_margin_when_activated: Option<i32>,
    extra_top_padding_when_activated: Option<i32>,
    graphical_set: Option<crate::types::ElementImageSet>,
    horizontal_scrollbar_style: Option<crate::types::HorizontalScrollBarStyleSpecification>,
    scrollbars_go_outside: Option<bool>,
    vertical_flow_style: Option<crate::types::VerticalFlowStyleSpecification>,
    vertical_scrollbar_style: Option<crate::types::VerticalScrollBarStyleSpecification>,
}
