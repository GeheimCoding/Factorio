#[derive(Debug, serde::Deserialize)]
pub struct ItemWithEntityDataPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::ItemPrototype,
    icon_tintable: Option<crate::types::FileName>,
    icon_tintable_mask: Option<crate::types::FileName>,
    #[serde(default = "default_icon_tintable_mask_size")]
    icon_tintable_mask_size: crate::types::SpriteSizeType,
    icon_tintable_masks: Option<crate::vec::Vec<crate::types::IconData>>,
    #[serde(default = "default_icon_tintable_size")]
    icon_tintable_size: crate::types::SpriteSizeType,
    icon_tintables: Option<crate::vec::Vec<crate::types::IconData>>,
}
fn default_icon_tintable_mask_size() -> crate::types::SpriteSizeType {
    64
}
fn default_icon_tintable_size() -> crate::types::SpriteSizeType {
    64
}
