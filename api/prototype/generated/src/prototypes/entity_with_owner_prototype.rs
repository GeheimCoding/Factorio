#[derive(Debug, serde::Deserialize)]
pub struct EntityWithOwnerPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithHealthPrototype,
    #[serde(default = "default_allow_run_time_change_of_is_military_target")]
    allow_run_time_change_of_is_military_target: bool,
    #[serde(default = "default_is_military_target")]
    is_military_target: bool,
    // default: Calculated based on entity tile_width and height
    quality_indicator_scale: Option<f64>,
}
fn default_allow_run_time_change_of_is_military_target() -> bool {
    false
}
fn default_is_military_target() -> bool {
    false
}
