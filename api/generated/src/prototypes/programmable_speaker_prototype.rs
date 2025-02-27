#[derive(Debug, serde::Deserialize)]
pub struct ProgrammableSpeakerPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    #[serde(default = "default_audible_distance_modifier")]
    audible_distance_modifier: f32,
    circuit_connector: Option<crate::types::CircuitConnectorDefinition>,
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    energy_source: ProgrammableSpeakerPrototypeEnergySource,
    energy_usage_per_tick: crate::types::Energy,
    instruments: crate::vec::Vec<ProgrammableSpeakerInstrument>,
    maximum_polyphony: u32,
    sprite: Option<crate::types::Sprite>,
}
fn default_audible_distance_modifier() -> f32 {
    1.0
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
#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum ProgrammableSpeakerPrototypeEnergySource {
    #[serde(rename = "electric")]
    ElectricEnergySource(Box<crate::types::ElectricEnergySource>),
    #[serde(rename = "void")]
    VoidEnergySource(Box<crate::types::VoidEnergySource>),
}
#[derive(Debug, serde::Deserialize)]
pub struct ProgrammableSpeakerInstrument {
    name: String,
    notes: crate::vec::Vec<crate::types::ProgrammableSpeakerNote>,
}
