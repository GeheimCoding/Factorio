#[derive(Debug, serde::Deserialize)]
pub struct LightningAttractorPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    chargable_graphics: Option<crate::types::ChargableGraphics>,
    #[serde(default = "default_efficiency")]
    efficiency: f64,
    energy_source: Option<crate::types::ElectricEnergySource>,
    lightning_strike_offset: Option<crate::types::MapPosition>,
    #[serde(default = "default_range_elongation")]
    range_elongation: f64,
}
fn default_efficiency() -> f64 {
    0.0
}
fn default_range_elongation() -> f64 {
    0.0
}
