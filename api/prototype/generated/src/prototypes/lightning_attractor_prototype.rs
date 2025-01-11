pub struct LightningAttractorPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    chargable_graphics: crate::types::ChargableGraphics,
    efficiency: f64,
    energy_source: crate::types::ElectricEnergySource,
    lightning_strike_offset: crate::types::MapPosition,
    range_elongation: f64,
}
