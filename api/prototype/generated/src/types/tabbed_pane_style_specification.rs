#[derive(serde::Deserialize)]
pub struct TabbedPaneStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    tab_container: crate::types::TableStyleSpecification,
    tab_content_frame: crate::types::FrameStyleSpecification,
    #[serde(rename = "type")]
    type_: String,
    vertical_spacing: u32,
}
