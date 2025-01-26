#[derive(Debug, serde::Deserialize)]
pub struct ListBoxStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    item_style: Option<crate::types::ButtonStyleSpecification>,
    scroll_pane_style: Option<crate::types::ScrollPaneStyleSpecification>,
    #[serde(rename = "type")]
    type_: String,
}
