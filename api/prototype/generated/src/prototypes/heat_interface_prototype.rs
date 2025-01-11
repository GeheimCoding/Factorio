pub struct HeatInterfacePrototype {
    gui_mode: HeatInterfacePrototypeGuiMode,
    heat_buffer: crate::types::HeatBuffer,
    picture: crate::types::Sprite,
}
pub enum HeatInterfacePrototypeGuiMode {
    All,
    None,
    Admins,
}
