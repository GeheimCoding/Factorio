pub struct ProgrammableSpeakerPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    audible_distance_modifier: f32,
    circuit_connector: crate::types::CircuitConnectorDefinition,
    circuit_wire_max_distance: f64,
    draw_circuit_wires: bool,
    draw_copper_wires: bool,
    energy_source: ProgrammableSpeakerPrototypeEnergySource,
    energy_usage_per_tick: crate::types::Energy,
    instruments: Vec<ProgrammableSpeakerInstrument>,
    maximum_polyphony: u32,
    sprite: crate::types::Sprite,
}
#[derive(serde::Deserialize)]
pub enum ProgrammableSpeakerPrototypeEnergySource {
    #[serde(untagged)]
    ElectricEnergySource(Box<crate::types::ElectricEnergySource>),
    #[serde(untagged)]
    VoidEnergySource(Box<crate::types::VoidEnergySource>),
}
pub struct ProgrammableSpeakerInstrument {
    name: String,
    notes: Vec<crate::types::ProgrammableSpeakerNote>,
}
