pub struct WorkingVisualisations {
    always_draw_idle_animation: bool,
    animation: crate::types::Animation4Way,
    default_recipe_tint: crate::types::GlobalRecipeTints,
    idle_animation: crate::types::Animation4Way,
    recipe_not_set_tint: crate::types::GlobalRecipeTints,
    shift_animation_transition_duration: u16,
    shift_animation_waypoint_stop_duration: u16,
    shift_animation_waypoints: crate::types::ShiftAnimationWaypoints,
    states: Vec<crate::types::VisualState>,
    status_colors: crate::types::StatusColors,
    working_visualisations: Vec<crate::types::WorkingVisualisation>,
}
