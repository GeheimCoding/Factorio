pub struct HeatBuffer {
    connections: Vec<HeatConnection>,
    default_temperature: f64,
    heat_glow: Sprite4Way,
    heat_picture: Sprite4Way,
    heat_pipe_covers: Sprite4Way,
    max_temperature: f64,
    max_transfer: Energy,
    min_temperature_gradient: f64,
    min_working_temperature: f64,
    minimum_glow_temperature: f32,
    pipe_covers: Sprite4Way,
    specific_heat: Energy,
}
