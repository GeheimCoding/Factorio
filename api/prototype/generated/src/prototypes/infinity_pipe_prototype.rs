#[derive(serde::Deserialize)]
pub struct InfinityPipePrototype {
    base_: crate::prototypes::PipePrototype,
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
