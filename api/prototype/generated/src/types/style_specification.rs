#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum StyleSpecification {
    #[serde(rename = "activity_bar_style")]
    ActivityBarStyleSpecification(Box<crate::types::ActivityBarStyleSpecification>),
    #[serde(rename = "button_style")]
    ButtonStyleSpecification(Box<crate::types::ButtonStyleSpecification>),
    #[serde(rename = "camera_style")]
    CameraStyleSpecification(Box<crate::types::CameraStyleSpecification>),
    #[serde(rename = "checkbox_style")]
    CheckBoxStyleSpecification(Box<crate::types::CheckBoxStyleSpecification>),
    #[serde(rename = "dropdown_style")]
    DropDownStyleSpecification(Box<crate::types::DropDownStyleSpecification>),
    #[serde(rename = "flow_style")]
    FlowStyleSpecification(Box<crate::types::FlowStyleSpecification>),
    #[serde(rename = "frame_style")]
    FrameStyleSpecification(Box<crate::types::FrameStyleSpecification>),
    #[serde(rename = "graph_style")]
    GraphStyleSpecification(Box<crate::types::GraphStyleSpecification>),
    #[serde(rename = "horizontal_flow_style")]
    HorizontalFlowStyleSpecification(Box<crate::types::HorizontalFlowStyleSpecification>),
    #[serde(rename = "line_style")]
    LineStyleSpecification(Box<crate::types::LineStyleSpecification>),
    #[serde(rename = "image_style")]
    ImageStyleSpecification(Box<crate::types::ImageStyleSpecification>),
    #[serde(rename = "label_style")]
    LabelStyleSpecification(Box<crate::types::LabelStyleSpecification>),
    #[serde(rename = "list_box_style")]
    ListBoxStyleSpecification(Box<crate::types::ListBoxStyleSpecification>),
    #[serde(rename = "progressbar_style")]
    ProgressBarStyleSpecification(Box<crate::types::ProgressBarStyleSpecification>),
    #[serde(rename = "radiobutton_style")]
    RadioButtonStyleSpecification(Box<crate::types::RadioButtonStyleSpecification>),
    #[serde(rename = "horizontal_scrollbar_style")]
    HorizontalScrollBarStyleSpecification(Box<crate::types::HorizontalScrollBarStyleSpecification>),
    #[serde(rename = "vertical_scrollbar_style")]
    VerticalScrollBarStyleSpecification(Box<crate::types::VerticalScrollBarStyleSpecification>),
    #[serde(rename = "scroll_pane_style")]
    ScrollPaneStyleSpecification(Box<crate::types::ScrollPaneStyleSpecification>),
    #[serde(rename = "slider_style")]
    SliderStyleSpecification(Box<crate::types::SliderStyleSpecification>),
    #[serde(rename = "switch_style")]
    SwitchStyleSpecification(Box<crate::types::SwitchStyleSpecification>),
    #[serde(rename = "table_style")]
    TableStyleSpecification(Box<crate::types::TableStyleSpecification>),
    #[serde(rename = "tab_style")]
    TabStyleSpecification(Box<crate::types::TabStyleSpecification>),
    #[serde(rename = "textbox_style")]
    TextBoxStyleSpecification(Box<crate::types::TextBoxStyleSpecification>),
    #[serde(rename = "vertical_flow_style")]
    VerticalFlowStyleSpecification(Box<crate::types::VerticalFlowStyleSpecification>),
    #[serde(rename = "tabbed_pane_style")]
    TabbedPaneStyleSpecification(Box<crate::types::TabbedPaneStyleSpecification>),
    #[serde(rename = "empty_widget_style")]
    EmptyWidgetStyleSpecification(Box<crate::types::EmptyWidgetStyleSpecification>),
    #[serde(rename = "minimap_style")]
    MinimapStyleSpecification(Box<crate::types::MinimapStyleSpecification>),
    #[serde(rename = "technology_slot_style")]
    TechnologySlotStyleSpecification(Box<crate::types::TechnologySlotStyleSpecification>),
    #[serde(rename = "glow_style")]
    GlowStyleSpecification(Box<crate::types::GlowStyleSpecification>),
    #[serde(rename = "speech_bubble_style")]
    SpeechBubbleStyleSpecification(Box<crate::types::SpeechBubbleStyleSpecification>),
    #[serde(rename = "double_slider_style")]
    DoubleSliderStyleSpecification(Box<crate::types::DoubleSliderStyleSpecification>),
}
