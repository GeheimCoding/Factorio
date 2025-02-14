#[derive(Debug, serde::Deserialize)]
pub struct TabbedPaneStyleSpecification {
    #[serde(flatten)]
    base_: crate::types::BaseStyleSpecification,
    tab_container: Option<crate::types::TableStyleSpecification>,
    tab_content_frame: Option<crate::types::FrameStyleSpecification>,
    vertical_spacing: Option<u32>,
}
