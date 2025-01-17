#[derive(serde::Deserialize)]
pub struct BaseEnergySource {
    emissions_per_minute: std::collections::HashMap<crate::types::AirbornePollutantID, f64>,
    render_no_network_icon: bool,
    render_no_power_icon: bool,
}
