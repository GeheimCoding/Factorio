#[derive(serde::Deserialize)]
pub struct CreateExplosionTriggerEffectItem {
    base_: crate::types::CreateEntityTriggerEffectItem,
    #[serde(default = "default_cycle_while_moving")]
    cycle_while_moving: bool,
    #[serde(default = "default_inherit_movement_distance_from_projectile")]
    inherit_movement_distance_from_projectile: bool,
    #[serde(default = "default_max_movement_distance")]
    max_movement_distance: f32,
    #[serde(default = "default_max_movement_distance_deviation")]
    max_movement_distance_deviation: f32,
    #[serde(rename = "type")]
    type_: String,
}
fn default_cycle_while_moving() -> bool {
    false
}
fn default_inherit_movement_distance_from_projectile() -> bool {
    false
}
fn default_max_movement_distance() -> f32 {
    -1.0
}
fn default_max_movement_distance_deviation() -> f32 {
    0.0
}
