#[derive(serde::Deserialize)]
pub struct RobotWithLogisticInterfacePrototype {
    base_: crate::prototypes::FlyingRobotPrototype,
    charging_sound: Option<crate::types::InterruptibleSound>,
    destroy_action: Option<crate::types::Trigger>,
    #[serde(default = "default_draw_cargo")]
    draw_cargo: bool,
    idle: Option<crate::types::RotatedAnimation>,
    in_motion: Option<crate::types::RotatedAnimation>,
    max_payload_size: crate::types::ItemCountType,
    shadow_idle: Option<crate::types::RotatedAnimation>,
    shadow_in_motion: Option<crate::types::RotatedAnimation>,
}
fn default_draw_cargo() -> bool {
    true
}
