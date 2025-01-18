#[derive(serde::Deserialize)]
pub struct RailPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    #[serde(default = "default_build_grid_size")]
    build_grid_size: f64,
    deconstruction_marker_positions: Vec<crate::types::Vector>,
    ending_shifts: Vec<crate::types::Vector>,
    #[serde(default = "default_extra_planner_goal_penalty")]
    extra_planner_goal_penalty: f64,
    #[serde(default = "default_extra_planner_penalty")]
    extra_planner_penalty: f64,
    fence_pictures: crate::types::RailFenceGraphicsSet,
    #[serde(default = "default_forced_fence_segment_count")]
    forced_fence_segment_count: u8,
    pictures: crate::types::RailPictureSet,
    #[serde(default = "default_removes_soft_decoratives")]
    removes_soft_decoratives: bool,
    selection_box: crate::types::BoundingBox,
    walking_sound: crate::types::Sound,
}
fn default_build_grid_size() -> f64 {
    2.0
}
fn default_extra_planner_goal_penalty() -> f64 {
    0.0
}
fn default_extra_planner_penalty() -> f64 {
    0.0
}
fn default_forced_fence_segment_count() -> u8 {
    0
}
fn default_removes_soft_decoratives() -> bool {
    false
}
