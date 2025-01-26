#[derive(Debug, serde::Deserialize)]
pub struct StatelessVisualisation {
    acceleration_x: Option<f32>,
    acceleration_y: Option<f32>,
    acceleration_z: Option<f32>,
    #[serde(default = "default_adjust_animation_speed_by_base_scale")]
    adjust_animation_speed_by_base_scale: bool,
    #[serde(default = "default_affected_by_wind")]
    affected_by_wind: bool,
    animation: Option<crate::types::AnimationVariations>,
    #[serde(default = "default_begin_scale")]
    begin_scale: f32,
    #[serde(default = "default_can_lay_on_the_ground")]
    can_lay_on_the_ground: bool,
    count: Option<u16>,
    #[serde(default = "default_end_scale")]
    end_scale: f32,
    #[serde(default = "default_fade_in_progress_duration")]
    fade_in_progress_duration: f32,
    #[serde(default = "default_fade_out_progress_duration")]
    fade_out_progress_duration: f32,
    light: Option<crate::types::LightDefinition>,
    // default: Value of `min_count`
    max_count: Option<u16>,
    #[serde(default = "default_min_count")]
    min_count: u16,
    movement_slowdown_factor_x: Option<f32>,
    movement_slowdown_factor_y: Option<f32>,
    movement_slowdown_factor_z: Option<f32>,
    nested_visualisations: Option<StatelessVisualisationNestedVisualisations>,
    offset_x: Option<crate::types::RangedValue>,
    offset_y: Option<crate::types::RangedValue>,
    offset_z: Option<crate::types::RangedValue>,
    #[serde(default = "default_particle_tick_offset")]
    particle_tick_offset: f32,
    period: Option<u16>,
    positions: Option<Vec<crate::types::Vector>>,
    #[serde(default = "default_probability")]
    probability: f32,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    scale: Option<crate::types::RangedValue>,
    shadow: Option<crate::types::AnimationVariations>,
    speed_x: Option<crate::types::RangedValue>,
    speed_y: Option<crate::types::RangedValue>,
    speed_z: Option<crate::types::RangedValue>,
    #[serde(default = "default_spread_progress_duration")]
    spread_progress_duration: f32,
}
fn default_adjust_animation_speed_by_base_scale() -> bool {
    false
}
fn default_affected_by_wind() -> bool {
    false
}
fn default_begin_scale() -> f32 {
    1.0
}
fn default_can_lay_on_the_ground() -> bool {
    true
}
fn default_end_scale() -> f32 {
    1.0
}
fn default_fade_in_progress_duration() -> f32 {
    0.0
}
fn default_fade_out_progress_duration() -> f32 {
    0.0
}
fn default_min_count() -> u16 {
    1
}
#[derive(Debug, serde::Deserialize)]
pub enum StatelessVisualisationNestedVisualisations {
    #[serde(untagged)]
    StatelessVisualisation(Box<crate::types::StatelessVisualisation>),
    #[serde(untagged)]
    VecStatelessVisualisation(Vec<crate::types::StatelessVisualisation>),
}
fn default_particle_tick_offset() -> f32 {
    0.0
}
fn default_probability() -> f32 {
    1.0
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_spread_progress_duration() -> f32 {
    1.0
}
