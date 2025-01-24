#[derive(serde::Deserialize)]
pub struct CarPrototype {
    base_: crate::prototypes::VehiclePrototype,
    animation: Option<crate::types::RotatedAnimation>,
    #[serde(default = "default_auto_sort_inventory")]
    auto_sort_inventory: bool,
    consumption: crate::types::Energy,
    #[serde(default = "default_darkness_to_render_light_animation")]
    darkness_to_render_light_animation: f32,
    effectivity: f64,
    energy_source: CarPrototypeEnergySource,
    guns: Option<Vec<crate::types::ItemID>>,
    #[serde(default = "default_has_belt_immunity")]
    has_belt_immunity: bool,
    #[serde(default = "default_immune_to_cliff_impacts")]
    immune_to_cliff_impacts: bool,
    #[serde(default = "default_immune_to_rock_impacts")]
    immune_to_rock_impacts: bool,
    #[serde(default = "default_immune_to_tree_impacts")]
    immune_to_tree_impacts: bool,
    inventory_size: crate::types::ItemStackIndex,
    light: Option<crate::types::LightDefinition>,
    light_animation: Option<crate::types::RotatedAnimation>,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    rotation_speed: f64,
    sound_no_fuel: Option<crate::types::Sound>,
    #[serde(default = "default_tank_driving")]
    tank_driving: bool,
    track_particle_triggers: Option<crate::types::FootstepTriggerEffectList>,
    #[serde(default = "default_trash_inventory_size")]
    trash_inventory_size: crate::types::ItemStackIndex,
    turret_animation: Option<crate::types::RotatedAnimation>,
    #[serde(default = "default_turret_return_timeout")]
    turret_return_timeout: u32,
    #[serde(default = "default_turret_rotation_speed")]
    turret_rotation_speed: f32,
}
fn default_auto_sort_inventory() -> bool {
    false
}
fn default_darkness_to_render_light_animation() -> f32 {
    0.3
}
#[derive(serde::Deserialize)]
pub enum CarPrototypeEnergySource {
    #[serde(untagged)]
    BurnerEnergySource(Box<crate::types::BurnerEnergySource>),
    #[serde(untagged)]
    VoidEnergySource(Box<crate::types::VoidEnergySource>),
}
fn default_has_belt_immunity() -> bool {
    false
}
fn default_immune_to_cliff_impacts() -> bool {
    true
}
fn default_immune_to_rock_impacts() -> bool {
    false
}
fn default_immune_to_tree_impacts() -> bool {
    false
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_tank_driving() -> bool {
    false
}
fn default_trash_inventory_size() -> crate::types::ItemStackIndex {
    0
}
fn default_turret_return_timeout() -> u32 {
    60
}
fn default_turret_rotation_speed() -> f32 {
    0.0
}
