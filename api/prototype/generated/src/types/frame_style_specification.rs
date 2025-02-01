#[derive(Debug, serde::Deserialize)]
pub struct FrameStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    background_graphical_set: Option<crate::types::ElementImageSet>,
    border: Option<crate::types::BorderImageSet>,
    drag_by_title: Option<bool>,
    graphical_set: Option<crate::types::ElementImageSet>,
    header_background: Option<crate::types::ElementImageSet>,
    header_filler_style: Option<crate::types::EmptyWidgetStyleSpecification>,
    header_flow_style: Option<crate::types::HorizontalFlowStyleSpecification>,
    horizontal_flow_style: Option<crate::types::HorizontalFlowStyleSpecification>,
    title_style: Option<crate::types::LabelStyleSpecification>,
    use_header_filler: Option<bool>,
    vertical_flow_style: Option<crate::types::VerticalFlowStyleSpecification>,
}
