pub struct WorkingVisualisation {
    align_to_waypoint: bool,
    always_draw: bool,
    animated_shift: bool,
    animation: crate::types::Animation,
    apply_recipe_tint: WorkingVisualisationApplyRecipeTint,
    apply_tint: WorkingVisualisationApplyTint,
    constant_speed: bool,
    draw_in_states: Vec<String>,
    draw_when_state_filter_matches: bool,
    east_animation: crate::types::Animation,
    east_position: crate::types::Vector,
    east_secondary_draw_order: i8,
    effect: WorkingVisualisationEffect,
    enabled_by_name: bool,
    enabled_in_animated_shift_during_transition: bool,
    enabled_in_animated_shift_during_waypoint_stop: bool,
    fadeout: bool,
    frame_based_on_shift_animation_progress: bool,
    light: crate::types::LightDefinition,
    mining_drill_scorch_mark: bool,
    name: String,
    north_animation: crate::types::Animation,
    north_position: crate::types::Vector,
    north_secondary_draw_order: i8,
    render_layer: crate::types::RenderLayer,
    scorch_mark_fade_in_frames: u8,
    scorch_mark_fade_out_duration: u16,
    scorch_mark_lifetime: u16,
    secondary_draw_order: i8,
    south_animation: crate::types::Animation,
    south_position: crate::types::Vector,
    south_secondary_draw_order: i8,
    synced_fadeout: bool,
    west_animation: crate::types::Animation,
    west_position: crate::types::Vector,
    west_secondary_draw_order: i8,
}
#[derive(serde::Deserialize)]
pub enum WorkingVisualisationApplyRecipeTint {
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "secondary")]
    Secondary,
    #[serde(rename = "tertiary")]
    Tertiary,
    #[serde(rename = "quaternary")]
    Quaternary,
    #[serde(rename = "none")]
    None,
}
#[derive(serde::Deserialize)]
pub enum WorkingVisualisationApplyTint {
    #[serde(rename = "resource_color")]
    ResourceColor,
    #[serde(rename = "input_fluid_base_color")]
    InputFluidBaseColor,
    #[serde(rename = "input_fluid_flow_color")]
    InputFluidFlowColor,
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "visual_state_color")]
    VisualStateColor,
}
#[derive(serde::Deserialize)]
pub enum WorkingVisualisationEffect {
    #[serde(rename = "flicker")]
    Flicker,
    #[serde(rename = "uranium_glow")]
    UraniumGlow,
    #[serde(rename = "none")]
    None,
}
