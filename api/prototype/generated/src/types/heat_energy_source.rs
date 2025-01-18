#[derive(serde::Deserialize)]
pub struct HeatEnergySource {
    base_: crate::types::BaseEnergySource,
    connections: Vec<crate::types::HeatConnection>,
    default_temperature: f64,
    emissions_per_minute: std::collections::HashMap<crate::types::AirbornePollutantID, f64>,
    heat_glow: crate::types::Sprite4Way,
    heat_picture: crate::types::Sprite4Way,
    heat_pipe_covers: crate::types::Sprite4Way,
    max_temperature: f64,
    max_transfer: crate::types::Energy,
    min_temperature_gradient: f64,
    min_working_temperature: f64,
    minimum_glow_temperature: f32,
    pipe_covers: crate::types::Sprite4Way,
    specific_heat: crate::types::Energy,
    #[serde(rename = "type")]
    type_: String,
}
