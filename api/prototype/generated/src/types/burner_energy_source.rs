pub struct BurnerEnergySource {
    burner_usage: BurnerUsageID,
    burnt_inventory_size: ItemStackIndex,
    effectivity: f64,
    fuel_categories: Vec<FuelCategoryID>,
    fuel_inventory_size: ItemStackIndex,
    light_flicker: LightFlickeringDefinition,
    smoke: Vec<SmokeSource>,
    type_: Burner,
}
