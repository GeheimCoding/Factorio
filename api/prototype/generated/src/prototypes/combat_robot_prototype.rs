pub struct CombatRobotPrototype {
    base_: crate::prototypes::FlyingRobotPrototype,
    attack_parameters: crate::types::AttackParameters,
    destroy_action: crate::types::Trigger,
    follows_player: bool,
    friction: f64,
    idle: crate::types::RotatedAnimation,
    in_motion: crate::types::RotatedAnimation,
    light: crate::types::LightDefinition,
    range_from_player: f64,
    shadow_idle: crate::types::RotatedAnimation,
    shadow_in_motion: crate::types::RotatedAnimation,
    time_to_live: u32,
}
