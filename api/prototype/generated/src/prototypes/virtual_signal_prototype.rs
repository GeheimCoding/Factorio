#[derive(Debug, serde::Deserialize)]
pub struct VirtualSignalPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::Prototype,
    icon: Option<crate::types::FileName>,
    #[serde(default = "default_icon_size")]
    icon_size: crate::types::SpriteSizeType,
    icons: Option<Vec<crate::types::IconData>>,
}
fn default_icon_size() -> crate::types::SpriteSizeType {
    64
}
