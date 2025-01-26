#[derive(Debug, serde::Deserialize)]
pub struct SpeechBubbleStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    arrow_graphical_set: Option<crate::types::ElementImageSet>,
    arrow_indent: Option<f64>,
    close_color: Option<crate::types::Color>,
    frame_style: Option<crate::types::FrameStyleSpecification>,
    label_style: Option<crate::types::LabelStyleSpecification>,
    pass_through_mouse: Option<bool>,
    #[serde(rename = "type")]
    type_: String,
}
