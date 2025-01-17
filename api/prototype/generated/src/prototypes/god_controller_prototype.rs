#[derive(serde::Deserialize)]
pub struct GodControllerPrototype {
    crafting_categories: Vec<crate::types::RecipeCategoryID>,
    inventory_size: crate::types::ItemStackIndex,
    item_pickup_distance: f64,
    loot_pickup_distance: f64,
    mining_categories: Vec<crate::types::ResourceCategoryID>,
    mining_speed: f64,
    movement_speed: f64,
    name: String,
    type_: String,
}
