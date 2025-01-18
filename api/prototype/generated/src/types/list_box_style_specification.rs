#[derive(serde::Deserialize)]
pub struct ListBoxStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    item_style: crate::types::ButtonStyleSpecification,
    scroll_pane_style: crate::types::ScrollPaneStyleSpecification,
    #[serde(rename = "type")]
    type_: String,
}
