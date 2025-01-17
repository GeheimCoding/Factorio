#[derive(serde::Deserialize)]
pub struct TrivialSmokePrototype {
    base_: crate::prototypes::Prototype,
    affected_by_wind: bool,
    animation: crate::types::Animation,
    color: crate::types::Color,
    cyclic: bool,
    duration: u32,
    end_scale: f32,
    fade_away_duration: u32,
    fade_in_duration: u32,
    glow_animation: crate::types::Animation,
    glow_fade_away_duration: u32,
    movement_slow_down_factor: f64,
    render_layer: crate::types::RenderLayer,
    show_when_smoke_off: bool,
    spread_duration: u32,
    start_scale: f32,
}
