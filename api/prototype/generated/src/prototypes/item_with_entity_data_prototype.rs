#[derive(serde::Deserialize)]
pub struct ItemWithEntityDataPrototype {
    base_: crate::prototypes::ItemPrototype,
    icon_tintable: crate::types::FileName,
    icon_tintable_mask: crate::types::FileName,
    #[serde(default = "default_icon_tintable_mask_size")]
    icon_tintable_mask_size: crate::types::SpriteSizeType,
    icon_tintable_masks: Vec<crate::types::IconData>,
    #[serde(default = "default_icon_tintable_size")]
    icon_tintable_size: crate::types::SpriteSizeType,
    icon_tintables: Vec<crate::types::IconData>,
}
fn default_icon_tintable_mask_size() -> crate::types::SpriteSizeType {
    64
}
fn default_icon_tintable_size() -> crate::types::SpriteSizeType {
    64
}
