#[derive(Debug, serde::Deserialize)]
pub struct ReactorPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    circuit_connector: Option<crate::types::CircuitConnectorDefinition>,
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    connection_patches_connected: Option<crate::types::SpriteVariations>,
    connection_patches_disconnected: Option<crate::types::SpriteVariations>,
    consumption: crate::types::Energy,
    // default: `{1, 1, 1, 1} (white)`
    default_fuel_glow_color: Option<crate::types::Color>,
    default_temperature_signal: Option<crate::types::SignalIDConnector>,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    energy_source: crate::types::EnergySource,
    heat_buffer: crate::types::HeatBuffer,
    heat_connection_patches_connected: Option<crate::types::SpriteVariations>,
    heat_connection_patches_disconnected: Option<crate::types::SpriteVariations>,
    heat_lower_layer_picture: Option<crate::types::Sprite>,
    light: Option<crate::types::LightDefinition>,
    lower_layer_picture: Option<crate::types::Sprite>,
    meltdown_action: Option<crate::types::Trigger>,
    #[serde(default = "default_neighbour_bonus")]
    neighbour_bonus: f64,
    picture: Option<crate::types::Sprite>,
    #[serde(default = "default_scale_energy_usage")]
    scale_energy_usage: bool,
    #[serde(default = "default_use_fuel_glow_color")]
    use_fuel_glow_color: bool,
    working_light_picture: Option<crate::types::Animation>,
}
fn default_circuit_wire_max_distance() -> f64 {
    0.0
}
fn default_draw_circuit_wires() -> bool {
    true
}
fn default_draw_copper_wires() -> bool {
    true
}
fn default_neighbour_bonus() -> f64 {
    1.0
}
fn default_scale_energy_usage() -> bool {
    false
}
fn default_use_fuel_glow_color() -> bool {
    false
}
