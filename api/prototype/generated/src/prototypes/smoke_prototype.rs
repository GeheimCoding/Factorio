#[derive(serde::Deserialize)]
pub struct SmokePrototype {
    base_: crate::prototypes::EntityPrototype,
    #[serde(default = "default_affected_by_wind")]
    affected_by_wind: bool,
    animation: crate::types::Animation,
    // default: Empty = `{{0, 0}, {0, 0}}`
    collision_box: crate::types::BoundingBox,
    // default: `{r=0.375, g=0.375, b=0.375, a=0.375}`
    color: crate::types::Color,
    #[serde(default = "default_cyclic")]
    cyclic: bool,
    #[serde(default = "default_duration")]
    duration: u32,
    #[serde(default = "default_end_scale")]
    end_scale: f64,
    #[serde(default = "default_fade_away_duration")]
    fade_away_duration: u32,
    #[serde(default = "default_fade_in_duration")]
    fade_in_duration: u32,
    glow_animation: crate::types::Animation,
    // default: The value of `fade_away_duration`
    glow_fade_away_duration: u32,
    #[serde(default = "default_movement_slow_down_factor")]
    movement_slow_down_factor: f64,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    #[serde(default = "default_show_when_smoke_off")]
    show_when_smoke_off: bool,
    #[serde(default = "default_spread_duration")]
    spread_duration: u32,
    #[serde(default = "default_start_scale")]
    start_scale: f64,
}
fn default_affected_by_wind() -> bool {
    true
}
fn default_cyclic() -> bool {
    false
}
fn default_duration() -> u32 {
    0
}
fn default_end_scale() -> f64 {
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
fn default_start_scale() -> f64 {
    1.0
}
