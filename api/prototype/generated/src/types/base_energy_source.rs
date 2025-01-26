#[derive(Debug, serde::Deserialize)]
pub struct BaseEnergySource {
    emissions_per_minute: Option<std::collections::HashMap<crate::types::AirbornePollutantID, f64>>,
    #[serde(default = "default_render_no_network_icon")]
    render_no_network_icon: bool,
    #[serde(default = "default_render_no_power_icon")]
    render_no_power_icon: bool,
}
fn default_render_no_network_icon() -> bool {
    true
}
fn default_render_no_power_icon() -> bool {
    true
}
