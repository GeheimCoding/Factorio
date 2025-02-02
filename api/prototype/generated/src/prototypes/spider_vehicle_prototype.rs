#[derive(Debug, serde::Deserialize)]
pub struct SpiderVehiclePrototype {
    #[serde(flatten)]
    base_: crate::prototypes::VehiclePrototype,
    automatic_weapon_cycling: bool,
    chain_shooting_cooldown_modifier: f32,
    energy_source: SpiderVehiclePrototypeEnergySource,
    graphics_set: Option<crate::types::SpiderVehicleGraphicsSet>,
    guns: Option<crate::vec::Vec<crate::types::ItemID>>,
    height: f32,
    inventory_size: crate::types::ItemStackIndex,
    movement_energy_consumption: crate::types::Energy,
    spider_engine: crate::types::SpiderEngineSpecification,
    #[serde(default = "default_torso_bob_speed")]
    torso_bob_speed: f32,
    #[serde(default = "default_torso_rotation_speed")]
    torso_rotation_speed: f32,
    #[serde(default = "default_trash_inventory_size")]
    trash_inventory_size: crate::types::ItemStackIndex,
}
#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum SpiderVehiclePrototypeEnergySource {
    #[serde(rename = "burner")]
    BurnerEnergySource(Box<crate::types::BurnerEnergySource>),
    #[serde(rename = "void")]
    VoidEnergySource(Box<crate::types::VoidEnergySource>),
}
fn default_torso_bob_speed() -> f32 {
    1.0
}
fn default_torso_rotation_speed() -> f32 {
    1.0
}
fn default_trash_inventory_size() -> crate::types::ItemStackIndex {
    0
}
