#[derive(Debug, serde::Deserialize)]
pub struct WorkingVisualisation {
    #[serde(default = "default_align_to_waypoint")]
    align_to_waypoint: bool,
    #[serde(default = "default_always_draw")]
    always_draw: bool,
    #[serde(default = "default_animated_shift")]
    animated_shift: bool,
    animation: Option<crate::types::Animation>,
    apply_recipe_tint: Option<WorkingVisualisationApplyRecipeTint>,
    apply_tint: Option<WorkingVisualisationApplyTint>,
    #[serde(default = "default_constant_speed")]
    constant_speed: bool,
    draw_in_states: Option<Vec<String>>,
    #[serde(default = "default_draw_when_state_filter_matches")]
    draw_when_state_filter_matches: bool,
    east_animation: Option<crate::types::Animation>,
    east_position: Option<crate::types::Vector>,
    // default: Value of `secondary_draw_order`
    east_secondary_draw_order: Option<i8>,
    effect: Option<WorkingVisualisationEffect>,
    #[serde(default = "default_enabled_by_name")]
    enabled_by_name: bool,
    #[serde(default = "default_enabled_in_animated_shift_during_transition")]
    enabled_in_animated_shift_during_transition: bool,
    #[serde(default = "default_enabled_in_animated_shift_during_waypoint_stop")]
    enabled_in_animated_shift_during_waypoint_stop: bool,
    #[serde(default = "default_fadeout")]
    fadeout: bool,
    #[serde(default = "default_frame_based_on_shift_animation_progress")]
    frame_based_on_shift_animation_progress: bool,
    light: Option<crate::types::LightDefinition>,
    #[serde(default = "default_mining_drill_scorch_mark")]
    mining_drill_scorch_mark: bool,
    #[serde(default = "default_name")]
    name: String,
    north_animation: Option<crate::types::Animation>,
    north_position: Option<crate::types::Vector>,
    // default: Value of `secondary_draw_order`
    north_secondary_draw_order: Option<i8>,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    scorch_mark_fade_in_frames: Option<u8>,
    scorch_mark_fade_out_duration: Option<u16>,
    scorch_mark_lifetime: Option<u16>,
    secondary_draw_order: Option<i8>,
    south_animation: Option<crate::types::Animation>,
    south_position: Option<crate::types::Vector>,
    // default: Value of `secondary_draw_order`
    south_secondary_draw_order: Option<i8>,
    #[serde(default = "default_synced_fadeout")]
    synced_fadeout: bool,
    west_animation: Option<crate::types::Animation>,
    west_position: Option<crate::types::Vector>,
    // default: Value of `secondary_draw_order`
    west_secondary_draw_order: Option<i8>,
}
fn default_align_to_waypoint() -> bool {
    false
}
fn default_always_draw() -> bool {
    false
}
fn default_animated_shift() -> bool {
    false
}
#[derive(Debug, serde::Deserialize)]
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
#[derive(Debug, serde::Deserialize)]
pub enum WorkingVisualisationApplyTint {
    #[serde(rename = "resource-color")]
    ResourceColor,
    #[serde(rename = "input-fluid-base-color")]
    InputFluidBaseColor,
    #[serde(rename = "input-fluid-flow-color")]
    InputFluidFlowColor,
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "visual-state-color")]
    VisualStateColor,
}
fn default_constant_speed() -> bool {
    false
}
fn default_draw_when_state_filter_matches() -> bool {
    true
}
#[derive(Debug, serde::Deserialize)]
pub enum WorkingVisualisationEffect {
    #[serde(rename = "flicker")]
    Flicker,
    #[serde(rename = "uranium-glow")]
    UraniumGlow,
    #[serde(rename = "none")]
    None,
}
fn default_enabled_by_name() -> bool {
    false
}
fn default_enabled_in_animated_shift_during_transition() -> bool {
    true
}
fn default_enabled_in_animated_shift_during_waypoint_stop() -> bool {
    true
}
fn default_fadeout() -> bool {
    false
}
fn default_frame_based_on_shift_animation_progress() -> bool {
    false
}
fn default_mining_drill_scorch_mark() -> bool {
    false
}
fn default_name() -> String {
    String::from("")
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_synced_fadeout() -> bool {
    false
}
