#[derive(serde::Deserialize)]
pub struct ItemProductPrototype {
    amount: u16,
    amount_max: u16,
    amount_min: u16,
    extra_count_fraction: f32,
    ignored_by_productivity: u16,
    ignored_by_stats: u16,
    name: crate::types::ItemID,
    percent_spoiled: f32,
    probability: f64,
    show_details_in_recipe_tooltip: bool,
    type_: String,
}
