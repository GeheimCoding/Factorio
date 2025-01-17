#[derive(serde::Deserialize)]
pub struct CarPrototype {
    base_: crate::prototypes::VehiclePrototype,
    animation: crate::types::RotatedAnimation,
    auto_sort_inventory: bool,
    consumption: crate::types::Energy,
    darkness_to_render_light_animation: f32,
    effectivity: f64,
    energy_source: CarPrototypeEnergySource,
    guns: Vec<crate::types::ItemID>,
    has_belt_immunity: bool,
    immune_to_cliff_impacts: bool,
    immune_to_rock_impacts: bool,
    immune_to_tree_impacts: bool,
    inventory_size: crate::types::ItemStackIndex,
    light: crate::types::LightDefinition,
    light_animation: crate::types::RotatedAnimation,
    render_layer: crate::types::RenderLayer,
    rotation_speed: f64,
    sound_no_fuel: crate::types::Sound,
    tank_driving: bool,
    track_particle_triggers: crate::types::FootstepTriggerEffectList,
    trash_inventory_size: crate::types::ItemStackIndex,
    turret_animation: crate::types::RotatedAnimation,
    turret_return_timeout: u32,
    turret_rotation_speed: f32,
}
#[derive(serde::Deserialize)]
pub enum CarPrototypeEnergySource {
    #[serde(untagged)]
    BurnerEnergySource(Box<crate::types::BurnerEnergySource>),
    #[serde(untagged)]
    VoidEnergySource(Box<crate::types::VoidEnergySource>),
}
