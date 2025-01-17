#[derive(serde::Deserialize)]
pub struct VirtualSignalPrototype {
    base_: crate::prototypes::Prototype,
    icon: crate::types::FileName,
    icon_size: crate::types::SpriteSizeType,
    icons: Vec<crate::types::IconData>,
}
