#[derive(serde::Deserialize)]
pub struct FluidProductPrototype {
    amount: crate::types::FluidAmount,
    amount_max: crate::types::FluidAmount,
    amount_min: crate::types::FluidAmount,
    fluidbox_index: u32,
    ignored_by_productivity: crate::types::FluidAmount,
    ignored_by_stats: crate::types::FluidAmount,
    name: crate::types::FluidID,
    probability: f64,
    show_details_in_recipe_tooltip: bool,
    temperature: f32,
    #[serde(rename = "type")]
    type_: String,
}
