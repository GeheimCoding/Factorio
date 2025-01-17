#[derive(serde::Deserialize)]
pub struct StatelessVisualisation {
    acceleration_x: f32,
    acceleration_y: f32,
    acceleration_z: f32,
    adjust_animation_speed_by_base_scale: bool,
    affected_by_wind: bool,
    animation: crate::types::AnimationVariations,
    begin_scale: f32,
    can_lay_on_the_ground: bool,
    count: u16,
    end_scale: f32,
    fade_in_progress_duration: f32,
    fade_out_progress_duration: f32,
    light: crate::types::LightDefinition,
    max_count: u16,
    min_count: u16,
    movement_slowdown_factor_x: f32,
    movement_slowdown_factor_y: f32,
    movement_slowdown_factor_z: f32,
    nested_visualisations: StatelessVisualisationNestedVisualisations,
    offset_x: crate::types::RangedValue,
    offset_y: crate::types::RangedValue,
    offset_z: crate::types::RangedValue,
    particle_tick_offset: f32,
    period: u16,
    positions: Vec<crate::types::Vector>,
    probability: f32,
    render_layer: crate::types::RenderLayer,
    scale: crate::types::RangedValue,
    shadow: crate::types::AnimationVariations,
    speed_x: crate::types::RangedValue,
    speed_y: crate::types::RangedValue,
    speed_z: crate::types::RangedValue,
    spread_progress_duration: f32,
}
#[derive(serde::Deserialize)]
pub enum StatelessVisualisationNestedVisualisations {
    #[serde(untagged)]
    StatelessVisualisation(Box<crate::types::StatelessVisualisation>),
    #[serde(untagged)]
    VecStatelessVisualisation(Vec<crate::types::StatelessVisualisation>),
}
