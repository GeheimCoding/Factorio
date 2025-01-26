#[derive(Debug, serde::Deserialize)]
pub struct HeatBuffer {
    connections: Option<Vec<crate::types::HeatConnection>>,
    #[serde(default = "default_default_temperature")]
    default_temperature: f64,
    heat_glow: Option<crate::types::Sprite4Way>,
    heat_picture: Option<crate::types::Sprite4Way>,
    heat_pipe_covers: Option<crate::types::Sprite4Way>,
    max_temperature: f64,
    max_transfer: crate::types::Energy,
    #[serde(default = "default_min_temperature_gradient")]
    min_temperature_gradient: f64,
    #[serde(default = "default_min_working_temperature")]
    min_working_temperature: f64,
    #[serde(default = "default_minimum_glow_temperature")]
    minimum_glow_temperature: f32,
    pipe_covers: Option<crate::types::Sprite4Way>,
    specific_heat: crate::types::Energy,
}
fn default_default_temperature() -> f64 {
    15.0
}
fn default_min_temperature_gradient() -> f64 {
    1.0
}
fn default_min_working_temperature() -> f64 {
    15.0
}
fn default_minimum_glow_temperature() -> f32 {
    1.0
}
