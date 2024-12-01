pub struct WorkingVisualisations {
    always_draw_idle_animation: bool,
    animation: Animation4Way,
    default_recipe_tint: GlobalRecipeTints,
    idle_animation: Animation4Way,
    recipe_not_set_tint: GlobalRecipeTints,
    shift_animation_transition_duration: u16,
    shift_animation_waypoint_stop_duration: u16,
    shift_animation_waypoints: ShiftAnimationWaypoints,
    states: Vec<VisualState>,
    status_colors: StatusColors,
    working_visualisations: Vec<WorkingVisualisation>,
}
