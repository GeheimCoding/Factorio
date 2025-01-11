pub struct InfinityPipePrototype {
    base_: crate::prototypes::PipePrototype,
    gui_mode: InfinityPipePrototypeGuiMode,
}
pub enum InfinityPipePrototypeGuiMode {
    All,
    None,
    Admins,
}
