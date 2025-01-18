#[derive(serde::Deserialize)]
pub struct LootItem {
    #[serde(default = "default_count_max")]
    count_max: f64,
    #[serde(default = "default_count_min")]
    count_min: f64,
    item: crate::types::ItemID,
    #[serde(default = "default_probability")]
    probability: f64,
}
fn default_count_max() -> f64 {
    1.0
}
fn default_count_min() -> f64 {
    1.0
}
fn default_probability() -> f64 {
    1.0
}
