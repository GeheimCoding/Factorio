#[derive(serde::Deserialize)]
pub struct AssemblingMachinePrototype {
    base_: crate::prototypes::CraftingMachinePrototype,
    circuit_connector: (
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
    ),
    circuit_wire_max_distance: f64,
    default_recipe_finished_signal: crate::types::SignalIDConnector,
    default_working_signal: crate::types::SignalIDConnector,
    disabled_when_recipe_not_researched: bool,
    draw_circuit_wires: bool,
    draw_copper_wires: bool,
    enable_logistic_control_behavior: bool,
    fixed_quality: crate::types::QualityID,
    fixed_recipe: crate::types::RecipeID,
    fluid_boxes_off_when_no_fluid_recipe: bool,
    gui_title_key: String,
    ingredient_count: u8,
}
