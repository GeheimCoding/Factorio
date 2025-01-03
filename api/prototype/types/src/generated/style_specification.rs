pub enum StyleSpecification {
    ActivityBarStyleSpecification(Box<ActivityBarStyleSpecification>),
    ButtonStyleSpecification(Box<ButtonStyleSpecification>),
    CameraStyleSpecification(Box<CameraStyleSpecification>),
    CheckBoxStyleSpecification(Box<CheckBoxStyleSpecification>),
    DropDownStyleSpecification(Box<DropDownStyleSpecification>),
    FlowStyleSpecification(Box<FlowStyleSpecification>),
    FrameStyleSpecification(Box<FrameStyleSpecification>),
    GraphStyleSpecification(Box<GraphStyleSpecification>),
    HorizontalFlowStyleSpecification(Box<HorizontalFlowStyleSpecification>),
    LineStyleSpecification(Box<LineStyleSpecification>),
    ImageStyleSpecification(Box<ImageStyleSpecification>),
    LabelStyleSpecification(Box<LabelStyleSpecification>),
    ListBoxStyleSpecification(Box<ListBoxStyleSpecification>),
    ProgressBarStyleSpecification(Box<ProgressBarStyleSpecification>),
    RadioButtonStyleSpecification(Box<RadioButtonStyleSpecification>),
    HorizontalScrollBarStyleSpecification(Box<HorizontalScrollBarStyleSpecification>),
    VerticalScrollBarStyleSpecification(Box<VerticalScrollBarStyleSpecification>),
    ScrollPaneStyleSpecification(Box<ScrollPaneStyleSpecification>),
    SliderStyleSpecification(Box<SliderStyleSpecification>),
    SwitchStyleSpecification(Box<SwitchStyleSpecification>),
    TableStyleSpecification(Box<TableStyleSpecification>),
    TabStyleSpecification(Box<TabStyleSpecification>),
    TextBoxStyleSpecification(Box<TextBoxStyleSpecification>),
    VerticalFlowStyleSpecification(Box<VerticalFlowStyleSpecification>),
    TabbedPaneStyleSpecification(Box<TabbedPaneStyleSpecification>),
    EmptyWidgetStyleSpecification(Box<EmptyWidgetStyleSpecification>),
    MinimapStyleSpecification(Box<MinimapStyleSpecification>),
    TechnologySlotStyleSpecification(Box<TechnologySlotStyleSpecification>),
    GlowStyleSpecification(Box<GlowStyleSpecification>),
    SpeechBubbleStyleSpecification(Box<SpeechBubbleStyleSpecification>),
    DoubleSliderStyleSpecification(Box<DoubleSliderStyleSpecification>),
}
