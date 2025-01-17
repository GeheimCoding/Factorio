pub struct SpiderVehiclePrototype {
    base_: crate::prototypes::VehiclePrototype,
    automatic_weapon_cycling: bool,
    chain_shooting_cooldown_modifier: f32,
    chunk_exploration_radius: u32,
    energy_source: SpiderVehiclePrototypeEnergySource,
    graphics_set: crate::types::SpiderVehicleGraphicsSet,
    guns: Vec<crate::types::ItemID>,
    height: f32,
    inventory_size: crate::types::ItemStackIndex,
    movement_energy_consumption: crate::types::Energy,
    spider_engine: crate::types::SpiderEngineSpecification,
    torso_bob_speed: f32,
    torso_rotation_speed: f32,
    trash_inventory_size: crate::types::ItemStackIndex,
}
#[derive(serde::Deserialize)]
pub enum SpiderVehiclePrototypeEnergySource {
    #[serde(untagged)]
    BurnerEnergySource(Box<crate::types::BurnerEnergySource>),
    #[serde(untagged)]
    VoidEnergySource(Box<crate::types::VoidEnergySource>),
}
