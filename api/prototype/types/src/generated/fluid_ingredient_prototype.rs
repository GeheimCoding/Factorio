pub struct FluidIngredientPrototype {
    amount: FluidAmount,
    fluidbox_index: u32,
    fluidbox_multiplier: u8,
    ignored_by_stats: FluidAmount,
    maximum_temperature: f32,
    minimum_temperature: f32,
    name: FluidID,
    temperature: f32,
    type_: Fluid,
}
