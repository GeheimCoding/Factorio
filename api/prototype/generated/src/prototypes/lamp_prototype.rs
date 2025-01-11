pub struct LampPrototype {
    always_on: bool,
    circuit_connector: crate::types::CircuitConnectorDefinition,
    circuit_wire_max_distance: f64,
    darkness_for_all_lamps_off: f32,
    darkness_for_all_lamps_on: f32,
    default_blue_signal: crate::types::SignalIDConnector,
    default_green_signal: crate::types::SignalIDConnector,
    default_red_signal: crate::types::SignalIDConnector,
    default_rgb_signal: crate::types::SignalIDConnector,
    draw_circuit_wires: bool,
    draw_copper_wires: bool,
    energy_source: LampPrototypeEnergySource,
    energy_usage_per_tick: crate::types::Energy,
    glow_color_intensity: f32,
    glow_render_mode: LampPrototypeGlowRenderMode,
    glow_size: f32,
    light: crate::types::LightDefinition,
    light_when_colored: crate::types::LightDefinition,
    picture_off: crate::types::Sprite,
    picture_on: crate::types::Sprite,
    signal_to_color_mapping: Vec<crate::types::SignalColorMapping>,
}
pub enum LampPrototypeEnergySource {
    ElectricEnergySource(Box<crate::types::ElectricEnergySource>),
    VoidEnergySource(Box<crate::types::VoidEnergySource>),
}
pub enum LampPrototypeGlowRenderMode {
    Additive,
    Multiplicative,
}
