pub struct FlyingRobotPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    energy_per_move: crate::types::Energy,
    energy_per_tick: crate::types::Energy,
    is_military_target: bool,
    max_energy: crate::types::Energy,
    max_speed: f64,
    max_to_charge: f32,
    min_to_charge: f32,
    speed: f64,
    speed_multiplier_when_out_of_energy: f32,
}
