#[derive(serde::Deserialize)]
pub struct ItemGroup {
    base_: crate::prototypes::Prototype,
    icon: crate::types::FileName,
    icon_size: crate::types::SpriteSizeType,
    icons: Vec<crate::types::IconData>,
    order_in_recipe: crate::types::Order,
}
