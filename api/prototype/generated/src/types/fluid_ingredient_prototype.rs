#[derive(serde::Deserialize)]
pub struct FluidIngredientPrototype {
    amount: crate::types::FluidAmount,
    #[serde(default = "default_fluidbox_index")]
    fluidbox_index: u32,
    #[serde(default = "default_fluidbox_multiplier")]
    fluidbox_multiplier: u8,
    #[serde(default = "default_ignored_by_stats")]
    ignored_by_stats: crate::types::FluidAmount,
    maximum_temperature: Option<f32>,
    minimum_temperature: Option<f32>,
    name: crate::types::FluidID,
    temperature: Option<f32>,
    #[serde(rename = "type")]
    type_: String,
}
fn default_fluidbox_index() -> u32 {
    0
}
fn default_fluidbox_multiplier() -> u8 {
    2
}
fn default_ignored_by_stats() -> crate::types::FluidAmount {
    0.0
}
