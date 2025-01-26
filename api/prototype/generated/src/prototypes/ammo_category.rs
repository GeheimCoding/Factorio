#[derive(Debug, serde::Deserialize)]
pub struct AmmoCategory {
    base_: crate::prototypes::Prototype,
    #[serde(default = "default_bonus_gui_order")]
    bonus_gui_order: crate::types::Order,
    icon: Option<crate::types::FileName>,
    #[serde(default = "default_icon_size")]
    icon_size: crate::types::SpriteSizeType,
    icons: Option<Vec<crate::types::IconData>>,
}
fn default_bonus_gui_order() -> crate::types::Order {
    String::from("")
}
fn default_icon_size() -> crate::types::SpriteSizeType {
    64
}
