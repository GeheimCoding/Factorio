#[derive(Debug, serde::Deserialize)]
pub struct SolarPanelPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    energy_source: crate::types::ElectricEnergySource,
    overlay: Option<crate::types::SpriteVariations>,
    picture: Option<crate::types::SpriteVariations>,
    production: crate::types::Energy,
}
