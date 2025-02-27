#[derive(Debug, serde::Deserialize)]
pub struct VehiclePrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    #[serde(default = "default_allow_passengers")]
    allow_passengers: bool,
    #[serde(default = "default_allow_remote_driving")]
    allow_remote_driving: bool,
    #[serde(alias = "braking_force")]
    braking_power: VehiclePrototypeBrakingPower,
    #[serde(default = "default_chunk_exploration_radius")]
    chunk_exploration_radius: u32,
    crash_trigger: Option<crate::types::TriggerEffect>,
    #[serde(default = "default_deliver_category")]
    deliver_category: String,
    energy_per_hit_point: f64,
    equipment_grid: Option<crate::types::EquipmentGridID>,
    #[serde(alias = "friction_force")]
    friction: f64,
    #[serde(default = "default_impact_speed_to_volume_ratio")]
    impact_speed_to_volume_ratio: f64,
    minimap_representation: Option<crate::types::Sprite>,
    selected_minimap_representation: Option<crate::types::Sprite>,
    stop_trigger: Option<crate::types::TriggerEffect>,
    #[serde(default = "default_stop_trigger_speed")]
    stop_trigger_speed: f64,
    #[serde(default = "default_terrain_friction_modifier")]
    terrain_friction_modifier: f32,
    weight: f64,
}
fn default_allow_passengers() -> bool {
    true
}
fn default_allow_remote_driving() -> bool {
    false
}
#[derive(Debug, serde::Deserialize)]
pub enum VehiclePrototypeBrakingPower {
    #[serde(untagged)]
    Energy(crate::types::Energy),
    #[serde(untagged)]
    F64(f64),
}
fn default_chunk_exploration_radius() -> u32 {
    0
}
fn default_deliver_category() -> String {
    String::from("")
}
fn default_impact_speed_to_volume_ratio() -> f64 {
    5.0
}
fn default_stop_trigger_speed() -> f64 {
    0.0
}
fn default_terrain_friction_modifier() -> f32 {
    1.0
}
