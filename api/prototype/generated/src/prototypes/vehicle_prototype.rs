#[derive(serde::Deserialize)]
pub struct VehiclePrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    #[serde(default = "default_allow_passengers")]
    allow_passengers: bool,
    #[serde(default = "default_allow_remote_driving")]
    allow_remote_driving: bool,
    #[serde(alias = "braking_force")]
    braking_power: VehiclePrototypeBrakingPower,
    crash_trigger: crate::types::TriggerEffect,
    #[serde(default = "default_deliver_category")]
    deliver_category: String,
    energy_per_hit_point: f64,
    equipment_grid: crate::types::EquipmentGridID,
    #[serde(alias = "friction_force")]
    friction: f64,
    #[serde(default = "default_impact_speed_to_volume_ratio")]
    impact_speed_to_volume_ratio: f64,
    minimap_representation: crate::types::Sprite,
    selected_minimap_representation: crate::types::Sprite,
    stop_trigger: crate::types::TriggerEffect,
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
#[derive(serde::Deserialize)]
pub enum VehiclePrototypeBrakingPower {
    #[serde(untagged)]
    Energy(crate::types::Energy),
    #[serde(untagged)]
    F64(f64),
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
