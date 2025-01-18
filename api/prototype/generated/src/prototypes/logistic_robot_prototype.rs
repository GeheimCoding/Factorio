#[derive(serde::Deserialize)]
pub struct LogisticRobotPrototype {
    base_: crate::prototypes::RobotWithLogisticInterfacePrototype,
    // default: Empty = `{{0, 0}, {0, 0}}`
    collision_box: crate::types::BoundingBox,
    idle_with_cargo: crate::types::RotatedAnimation,
    in_motion_with_cargo: crate::types::RotatedAnimation,
    shadow_idle_with_cargo: crate::types::RotatedAnimation,
    shadow_in_motion_with_cargo: crate::types::RotatedAnimation,
}
