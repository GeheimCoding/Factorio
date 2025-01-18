#[derive(serde::Deserialize)]
pub struct EntityWithOwnerPrototype {
    base_: crate::prototypes::EntityWithHealthPrototype,
    #[serde(default = "default_allow_run_time_change_of_is_military_target")]
    allow_run_time_change_of_is_military_target: bool,
    #[serde(default = "default_is_military_target")]
    is_military_target: bool,
    // default: Calculated based on bounding box
    quality_indicator_scale: f64,
}
fn default_allow_run_time_change_of_is_military_target() -> bool {
    false
}
fn default_is_military_target() -> bool {
    false
}
