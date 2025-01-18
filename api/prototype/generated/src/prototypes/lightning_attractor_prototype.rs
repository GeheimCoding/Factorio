#[derive(serde::Deserialize)]
pub struct LightningAttractorPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    chargable_graphics: crate::types::ChargableGraphics,
    #[serde(default = "default_efficiency")]
    efficiency: f64,
    energy_source: crate::types::ElectricEnergySource,
    lightning_strike_offset: crate::types::MapPosition,
    #[serde(default = "default_range_elongation")]
    range_elongation: f64,
}
fn default_efficiency() -> f64 {
    0.0
}
fn default_range_elongation() -> f64 {
    0.0
}
