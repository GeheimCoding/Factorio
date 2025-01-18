#[derive(serde::Deserialize)]
pub struct ItemGroup {
    base_: crate::prototypes::Prototype,
    icon: crate::types::FileName,
    #[serde(default = "default_icon_size")]
    icon_size: crate::types::SpriteSizeType,
    icons: Vec<crate::types::IconData>,
    // default: The `order` of this item group.
    order_in_recipe: crate::types::Order,
}
fn default_icon_size() -> crate::types::SpriteSizeType {
    64
}
