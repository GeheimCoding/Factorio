#[derive(serde::Deserialize)]
pub struct InfinityPipePrototype {
    base_: crate::prototypes::PipePrototype,
    #[serde(default = "default_gui_mode")]
    gui_mode: InfinityPipePrototypeGuiMode,
}
#[derive(serde::Deserialize)]
pub enum InfinityPipePrototypeGuiMode {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "admins")]
    Admins,
}
fn default_gui_mode() -> InfinityPipePrototypeGuiMode {
    InfinityPipePrototypeGuiMode::All
}
