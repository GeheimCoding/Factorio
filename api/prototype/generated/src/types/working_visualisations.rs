#[derive(serde::Deserialize)]
pub struct WorkingVisualisations {
    #[serde(default = "default_always_draw_idle_animation")]
    always_draw_idle_animation: bool,
    animation: crate::types::Animation4Way,
    default_recipe_tint: crate::types::GlobalRecipeTints,
    idle_animation: crate::types::Animation4Way,
    recipe_not_set_tint: crate::types::GlobalRecipeTints,
    #[serde(default = "default_shift_animation_transition_duration")]
    shift_animation_transition_duration: u16,
    #[serde(default = "default_shift_animation_waypoint_stop_duration")]
    shift_animation_waypoint_stop_duration: u16,
    shift_animation_waypoints: crate::types::ShiftAnimationWaypoints,
    states: Vec<crate::types::VisualState>,
    status_colors: crate::types::StatusColors,
    working_visualisations: Vec<crate::types::WorkingVisualisation>,
}
fn default_always_draw_idle_animation() -> bool {
    false
}
fn default_shift_animation_transition_duration() -> u16 {
    0
}
fn default_shift_animation_waypoint_stop_duration() -> u16 {
    0
}
