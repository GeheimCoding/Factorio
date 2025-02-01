#[derive(Debug, serde::Deserialize)]
pub struct LampPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    #[serde(default = "default_always_on")]
    always_on: bool,
    circuit_connector: Option<crate::types::CircuitConnectorDefinition>,
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    #[serde(default = "default_darkness_for_all_lamps_off")]
    darkness_for_all_lamps_off: f32,
    #[serde(default = "default_darkness_for_all_lamps_on")]
    darkness_for_all_lamps_on: f32,
    default_blue_signal: Option<crate::types::SignalIDConnector>,
    default_green_signal: Option<crate::types::SignalIDConnector>,
    default_red_signal: Option<crate::types::SignalIDConnector>,
    default_rgb_signal: Option<crate::types::SignalIDConnector>,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    energy_source: LampPrototypeEnergySource,
    energy_usage_per_tick: crate::types::Energy,
    #[serde(default = "default_glow_color_intensity")]
    glow_color_intensity: f32,
    #[serde(default = "default_glow_render_mode")]
    glow_render_mode: LampPrototypeGlowRenderMode,
    #[serde(default = "default_glow_size")]
    glow_size: f32,
    light: Option<crate::types::LightDefinition>,
    light_when_colored: Option<crate::types::LightDefinition>,
    picture_off: Option<crate::types::Sprite>,
    picture_on: Option<crate::types::Sprite>,
    signal_to_color_mapping: Option<Vec<crate::types::SignalColorMapping>>,
}
fn default_always_on() -> bool {
    false
}
fn default_circuit_wire_max_distance() -> f64 {
    0.0
}
fn default_darkness_for_all_lamps_off() -> f32 {
    0.3
}
fn default_darkness_for_all_lamps_on() -> f32 {
    0.5
}
fn default_draw_circuit_wires() -> bool {
    true
}
fn default_draw_copper_wires() -> bool {
    true
}
#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum LampPrototypeEnergySource {
    #[serde(rename = "electric")]
    ElectricEnergySource(Box<crate::types::ElectricEnergySource>),
    #[serde(rename = "void")]
    VoidEnergySource(Box<crate::types::VoidEnergySource>),
}
fn default_glow_color_intensity() -> f32 {
    0.0
}
#[derive(Debug, serde::Deserialize)]
pub enum LampPrototypeGlowRenderMode {
    #[serde(rename = "additive")]
    Additive,
    #[serde(rename = "multiplicative")]
    Multiplicative,
}
fn default_glow_render_mode() -> LampPrototypeGlowRenderMode {
    LampPrototypeGlowRenderMode::Additive
}
fn default_glow_size() -> f32 {
    0.0
}
