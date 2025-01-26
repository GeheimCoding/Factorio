#[derive(Debug, serde::Deserialize)]
pub struct FlyingRobotPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    #[serde(default = "default_energy_per_move")]
    energy_per_move: crate::types::Energy,
    #[serde(default = "default_energy_per_tick")]
    energy_per_tick: crate::types::Energy,
    #[serde(default = "default_is_military_target")]
    is_military_target: bool,
    #[serde(default = "default_max_energy")]
    max_energy: crate::types::Energy,
    // default: max double
    max_speed: Option<f64>,
    #[serde(default = "default_max_to_charge")]
    max_to_charge: f32,
    #[serde(default = "default_min_to_charge")]
    min_to_charge: f32,
    speed: f64,
    #[serde(default = "default_speed_multiplier_when_out_of_energy")]
    speed_multiplier_when_out_of_energy: f32,
}
fn default_energy_per_move() -> crate::types::Energy {
    String::from("0")
}
fn default_energy_per_tick() -> crate::types::Energy {
    String::from("0")
}
fn default_is_military_target() -> bool {
    true
}
fn default_max_energy() -> crate::types::Energy {
    String::from("0")
}
fn default_max_to_charge() -> f32 {
    0.9
}
fn default_min_to_charge() -> f32 {
    0.2
}
fn default_speed_multiplier_when_out_of_energy() -> f32 {
    0.0
}
