#[derive(Debug, serde::Deserialize)]
pub struct AgriculturalTowerPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    arm_extending_sound: Option<crate::types::InterruptibleSound>,
    arm_extending_sound_source: Option<String>,
    central_orienting_sound: Option<crate::types::InterruptibleSound>,
    central_orienting_sound_source: Option<String>,
    circuit_connector: Option<crate::types::CircuitConnectorDefinition>,
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    crane: crate::types::AgriculturalCraneProperties,
    crane_energy_usage: crate::types::Energy,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    energy_source: crate::types::EnergySource,
    energy_usage: crate::types::Energy,
    graphics_set: Option<crate::types::CraftingMachineGraphicsSet>,
    grappler_extending_sound: Option<crate::types::InterruptibleSound>,
    grappler_extending_sound_source: Option<String>,
    grappler_orienting_sound: Option<crate::types::InterruptibleSound>,
    grappler_orienting_sound_source: Option<String>,
    #[serde(default = "default_growth_grid_tile_size")]
    growth_grid_tile_size: u32,
    harvesting_procedure_points: Option<Vec<crate::types::Vector3D>>,
    harvesting_sound: Option<crate::types::Sound>,
    input_inventory_size: crate::types::ItemStackIndex,
    #[serde(default = "default_output_inventory_size")]
    output_inventory_size: crate::types::ItemStackIndex,
    planting_procedure_points: Option<Vec<crate::types::Vector3D>>,
    planting_sound: Option<crate::types::Sound>,
    radius: f64,
    radius_visualisation_picture: Option<crate::types::Sprite>,
    #[serde(default = "default_random_growth_offset")]
    random_growth_offset: f64,
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
fn default_growth_grid_tile_size() -> u32 {
    3
}
fn default_output_inventory_size() -> crate::types::ItemStackIndex {
    0
}
fn default_random_growth_offset() -> f64 {
    0.2
}
