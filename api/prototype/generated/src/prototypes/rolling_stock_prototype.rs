#[derive(Debug, serde::Deserialize)]
pub struct RollingStockPrototype {
    base_: crate::prototypes::VehiclePrototype,
    air_resistance: f64,
    #[serde(default = "default_allow_manual_color")]
    allow_manual_color: bool,
    #[serde(default = "default_allow_robot_dispatch_in_automatic_mode")]
    allow_robot_dispatch_in_automatic_mode: bool,
    back_light: Option<crate::types::LightDefinition>,
    color: Option<crate::types::Color>,
    connection_distance: f64,
    #[serde(default = "default_default_copy_color_from_train_stop")]
    default_copy_color_from_train_stop: bool,
    door_closing_sound: Option<crate::types::InterruptibleSound>,
    door_opening_sound: Option<crate::types::InterruptibleSound>,
    drive_over_elevated_tie_trigger: Option<crate::types::TriggerEffect>,
    drive_over_tie_trigger: Option<crate::types::TriggerEffect>,
    #[serde(default = "default_drive_over_tie_trigger_minimal_speed")]
    drive_over_tie_trigger_minimal_speed: f64,
    elevated_collision_mask: Option<crate::types::CollisionMaskConnector>,
    elevated_rail_sound: Option<crate::types::MainSound>,
    #[serde(default = "default_elevated_selection_priority")]
    elevated_selection_priority: u8,
    horizontal_doors: Option<crate::types::Animation>,
    joint_distance: f64,
    max_speed: f64,
    pictures: Option<crate::types::RollingStockRotatedSlopedGraphics>,
    stand_by_light: Option<crate::types::LightDefinition>,
    #[serde(default = "default_tie_distance")]
    tie_distance: f64,
    transition_collision_mask: Option<crate::types::CollisionMaskConnector>,
    vertical_doors: Option<crate::types::Animation>,
    vertical_selection_shift: f64,
    wheels: Option<crate::types::RollingStockRotatedSlopedGraphics>,
}
fn default_allow_manual_color() -> bool {
    true
}
fn default_allow_robot_dispatch_in_automatic_mode() -> bool {
    false
}
fn default_default_copy_color_from_train_stop() -> bool {
    false
}
fn default_drive_over_tie_trigger_minimal_speed() -> f64 {
    0.0
}
fn default_elevated_selection_priority() -> u8 {
    56
}
fn default_tie_distance() -> f64 {
    10.0
}
