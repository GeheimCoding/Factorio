#[derive(Debug, serde::Deserialize)]
pub struct LogisticRobotPrototype {
    base_: crate::prototypes::RobotWithLogisticInterfacePrototype,
    // default: Empty = `{{0, 0}, {0, 0}}`
    collision_box: Option<crate::types::BoundingBox>,
    idle_with_cargo: Option<crate::types::RotatedAnimation>,
    in_motion_with_cargo: Option<crate::types::RotatedAnimation>,
    shadow_idle_with_cargo: Option<crate::types::RotatedAnimation>,
    shadow_in_motion_with_cargo: Option<crate::types::RotatedAnimation>,
}
