pub struct HeatInterfacePrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    gui_mode: HeatInterfacePrototypeGuiMode,
    heat_buffer: crate::types::HeatBuffer,
    picture: crate::types::Sprite,
}
#[derive(serde::Deserialize)]
pub enum HeatInterfacePrototypeGuiMode {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "admins")]
    Admins,
}
