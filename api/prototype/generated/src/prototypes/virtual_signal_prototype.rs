#[derive(serde::Deserialize)]
pub struct VirtualSignalPrototype {
    base_: crate::prototypes::Prototype,
    icon: crate::types::FileName,
    #[serde(default = "default_icon_size")]
    icon_size: crate::types::SpriteSizeType,
    icons: Vec<crate::types::IconData>,
}
fn default_icon_size() -> crate::types::SpriteSizeType {
    64
}
