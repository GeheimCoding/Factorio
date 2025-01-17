#[derive(serde::Deserialize)]
pub struct VehiclePrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    allow_passengers: bool,
    allow_remote_driving: bool,
    braking_power: VehiclePrototypeBrakingPower,
    crash_trigger: crate::types::TriggerEffect,
    deliver_category: String,
    energy_per_hit_point: f64,
    equipment_grid: crate::types::EquipmentGridID,
    friction: f64,
    impact_speed_to_volume_ratio: f64,
    minimap_representation: crate::types::Sprite,
    selected_minimap_representation: crate::types::Sprite,
    stop_trigger: crate::types::TriggerEffect,
    stop_trigger_speed: f64,
    terrain_friction_modifier: f32,
    weight: f64,
}
#[derive(serde::Deserialize)]
pub enum VehiclePrototypeBrakingPower {
    #[serde(untagged)]
    Energy(crate::types::Energy),
    #[serde(untagged)]
    F64(f64),
}
