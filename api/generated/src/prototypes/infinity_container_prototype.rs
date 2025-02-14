#[derive(Debug, serde::Deserialize)]
pub struct InfinityContainerPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::LogisticContainerPrototype,
    erase_contents_when_mined: bool,
    #[serde(default = "default_gui_mode")]
    gui_mode: InfinityContainerPrototypeGuiMode,
    inventory_size: crate::types::ItemStackIndex,
    logistic_mode: Option<InfinityContainerPrototypeLogisticMode>,
    #[serde(default = "default_preserve_contents_when_created")]
    preserve_contents_when_created: bool,
    #[serde(default = "default_render_not_in_network_icon")]
    render_not_in_network_icon: bool,
}
#[derive(Debug, serde::Deserialize)]
pub enum InfinityContainerPrototypeGuiMode {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "admins")]
    Admins,
}
fn default_gui_mode() -> InfinityContainerPrototypeGuiMode {
    InfinityContainerPrototypeGuiMode::All
}
#[derive(Debug, serde::Deserialize)]
pub enum InfinityContainerPrototypeLogisticMode {
    #[serde(rename = "active-provider")]
    ActiveProvider,
    #[serde(rename = "passive-provider")]
    PassiveProvider,
    #[serde(rename = "requester")]
    Requester,
    #[serde(rename = "storage")]
    Storage,
    #[serde(rename = "buffer")]
    Buffer,
}
fn default_preserve_contents_when_created() -> bool {
    true
}
fn default_render_not_in_network_icon() -> bool {
    false
}
