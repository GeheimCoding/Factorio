#[derive(Debug, serde::Deserialize)]
pub struct HeatInterfacePrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    #[serde(default = "default_gui_mode")]
    gui_mode: HeatInterfacePrototypeGuiMode,
    heat_buffer: crate::types::HeatBuffer,
    picture: Option<crate::types::Sprite>,
}
#[derive(Debug, serde::Deserialize)]
pub enum HeatInterfacePrototypeGuiMode {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "admins")]
    Admins,
}
fn default_gui_mode() -> HeatInterfacePrototypeGuiMode {
    HeatInterfacePrototypeGuiMode::All
}
