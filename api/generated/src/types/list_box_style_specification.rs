#[derive(Debug, serde::Deserialize)]
pub struct ListBoxStyleSpecification {
    #[serde(flatten)]
    base_: crate::types::BaseStyleSpecification,
    item_style: Option<crate::types::ButtonStyleSpecification>,
    scroll_pane_style: Option<crate::types::ScrollPaneStyleSpecification>,
}
