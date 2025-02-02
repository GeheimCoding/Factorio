#[derive(Debug, serde::Deserialize)]
pub struct InfinityContainerPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::LogisticContainerPrototype,
    erase_contents_when_mined: bool,
    #[serde(default = "default_gui_mode")]
    gui_mode: InfinityContainerPrototypeGuiMode,
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
fn default_preserve_contents_when_created() -> bool {
    true
}
fn default_render_not_in_network_icon() -> bool {
    false
}
