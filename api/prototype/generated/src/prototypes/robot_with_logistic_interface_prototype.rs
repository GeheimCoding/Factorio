#[derive(serde::Deserialize)]
pub struct RobotWithLogisticInterfacePrototype {
    base_: crate::prototypes::FlyingRobotPrototype,
    charging_sound: crate::types::InterruptibleSound,
    destroy_action: crate::types::Trigger,
    draw_cargo: bool,
    idle: crate::types::RotatedAnimation,
    in_motion: crate::types::RotatedAnimation,
    max_payload_size: crate::types::ItemCountType,
    shadow_idle: crate::types::RotatedAnimation,
    shadow_in_motion: crate::types::RotatedAnimation,
}
