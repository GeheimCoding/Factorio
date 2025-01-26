#[derive(Debug, serde::Deserialize)]
pub struct WorkingVisualisations {
    #[serde(default = "default_always_draw_idle_animation")]
    always_draw_idle_animation: bool,
    animation: Option<crate::types::Animation4Way>,
    default_recipe_tint: Option<crate::types::GlobalRecipeTints>,
    idle_animation: Option<crate::types::Animation4Way>,
    recipe_not_set_tint: Option<crate::types::GlobalRecipeTints>,
    #[serde(default = "default_shift_animation_transition_duration")]
    shift_animation_transition_duration: u16,
    #[serde(default = "default_shift_animation_waypoint_stop_duration")]
    shift_animation_waypoint_stop_duration: u16,
    shift_animation_waypoints: Option<crate::types::ShiftAnimationWaypoints>,
    states: Option<Vec<crate::types::VisualState>>,
    status_colors: Option<crate::types::StatusColors>,
    working_visualisations: Option<Vec<crate::types::WorkingVisualisation>>,
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
