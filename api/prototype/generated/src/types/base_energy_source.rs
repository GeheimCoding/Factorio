pub struct BaseEnergySource {
    emissions_per_minute: std::collections::HashMap<AirbornePollutantID, f64>,
    render_no_network_icon: bool,
    render_no_power_icon: bool,
}
