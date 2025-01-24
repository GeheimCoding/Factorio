#[derive(serde::Deserialize)]
pub struct TabbedPaneStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    tab_container: Option<crate::types::TableStyleSpecification>,
    tab_content_frame: Option<crate::types::FrameStyleSpecification>,
    #[serde(rename = "type")]
    type_: String,
    vertical_spacing: Option<u32>,
}
