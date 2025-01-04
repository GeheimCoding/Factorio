pub struct FluidIngredientPrototype {
    amount: crate::types::FluidAmount,
    fluidbox_index: u32,
    fluidbox_multiplier: u8,
    ignored_by_stats: crate::types::FluidAmount,
    maximum_temperature: f32,
    minimum_temperature: f32,
    name: crate::types::FluidID,
    temperature: f32,
    type_: Fluid,
}
