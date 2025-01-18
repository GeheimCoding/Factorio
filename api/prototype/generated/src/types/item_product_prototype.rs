#[derive(serde::Deserialize)]
pub struct ItemProductPrototype {
    amount: u16,
    amount_max: u16,
    amount_min: u16,
    #[serde(default = "default_extra_count_fraction")]
    extra_count_fraction: f32,
    #[serde(default = "default_ignored_by_productivity")]
    ignored_by_productivity: u16,
    #[serde(default = "default_ignored_by_stats")]
    ignored_by_stats: u16,
    name: crate::types::ItemID,
    #[serde(default = "default_percent_spoiled")]
    percent_spoiled: f32,
    #[serde(default = "default_probability")]
    probability: f64,
    #[serde(default = "default_show_details_in_recipe_tooltip")]
    show_details_in_recipe_tooltip: bool,
    #[serde(rename = "type")]
    type_: String,
}
fn default_extra_count_fraction() -> f32 {
    0.0
}
fn default_ignored_by_productivity() -> u16 {
    0
}
fn default_ignored_by_stats() -> u16 {
    0
}
fn default_percent_spoiled() -> f32 {
    0.0
}
fn default_probability() -> f64 {
    1.0
}
fn default_show_details_in_recipe_tooltip() -> bool {
    true
}
