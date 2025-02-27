#[derive(Debug, serde::Deserialize)]
pub struct TrivialSmokePrototype {
    #[serde(flatten)]
    base_: crate::prototypes::Prototype,
    #[serde(default = "default_affected_by_wind")]
    affected_by_wind: bool,
    animation: crate::types::Animation,
    // default: `{r=0.375, g=0.375, b=0.375, a=0.375}`
    color: Option<crate::types::Color>,
    #[serde(default = "default_cyclic")]
    cyclic: bool,
    duration: u32,
    #[serde(default = "default_end_scale")]
    end_scale: f32,
    #[serde(default = "default_fade_away_duration")]
    fade_away_duration: u32,
    #[serde(default = "default_fade_in_duration")]
    fade_in_duration: u32,
    glow_animation: Option<crate::types::Animation>,
    // default: Value of `fade_away_duration`
    glow_fade_away_duration: Option<u32>,
    #[serde(default = "default_movement_slow_down_factor")]
    movement_slow_down_factor: f64,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    #[serde(default = "default_show_when_smoke_off")]
    show_when_smoke_off: bool,
    #[serde(default = "default_spread_duration")]
    spread_duration: u32,
    #[serde(default = "default_start_scale")]
    start_scale: f32,
}
fn default_affected_by_wind() -> bool {
    true
}
fn default_cyclic() -> bool {
    false
}
fn default_end_scale() -> f32 {
    1.0
}
fn default_fade_away_duration() -> u32 {
    0
}
fn default_fade_in_duration() -> u32 {
    0
}
fn default_movement_slow_down_factor() -> f64 {
    1.0
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Smoke
}
fn default_show_when_smoke_off() -> bool {
    false
}
fn default_spread_duration() -> u32 {
    0
}
fn default_start_scale() -> f32 {
    1.0
}
