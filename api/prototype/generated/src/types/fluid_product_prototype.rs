#[derive(serde::Deserialize)]
pub struct FluidProductPrototype {
    amount: Option<crate::types::FluidAmount>,
    amount_max: Option<crate::types::FluidAmount>,
    amount_min: Option<crate::types::FluidAmount>,
    #[serde(default = "default_fluidbox_index")]
    fluidbox_index: u32,
    #[serde(default = "default_ignored_by_productivity")]
    ignored_by_productivity: crate::types::FluidAmount,
    #[serde(default = "default_ignored_by_stats")]
    ignored_by_stats: crate::types::FluidAmount,
    name: crate::types::FluidID,
    #[serde(default = "default_probability")]
    probability: f64,
    #[serde(default = "default_show_details_in_recipe_tooltip")]
    show_details_in_recipe_tooltip: bool,
    temperature: Option<f32>,
    #[serde(rename = "type")]
    type_: String,
}
fn default_fluidbox_index() -> u32 {
    0
}
fn default_ignored_by_productivity() -> crate::types::FluidAmount {
    0.0
}
fn default_ignored_by_stats() -> crate::types::FluidAmount {
    0.0
}
fn default_probability() -> f64 {
    1.0
}
fn default_show_details_in_recipe_tooltip() -> bool {
    true
}
