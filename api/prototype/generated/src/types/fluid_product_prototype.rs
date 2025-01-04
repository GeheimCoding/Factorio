pub struct FluidProductPrototype {
    amount: FluidAmount,
    amount_max: FluidAmount,
    amount_min: FluidAmount,
    fluidbox_index: u32,
    ignored_by_productivity: FluidAmount,
    ignored_by_stats: FluidAmount,
    name: FluidID,
    probability: f64,
    show_details_in_recipe_tooltip: bool,
    temperature: f32,
    type_: Fluid,
}
