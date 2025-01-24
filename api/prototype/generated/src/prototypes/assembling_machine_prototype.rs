#[derive(serde::Deserialize)]
pub struct AssemblingMachinePrototype {
    base_: crate::prototypes::CraftingMachinePrototype,
    circuit_connector: Option<(
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
    )>,
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    default_recipe_finished_signal: Option<crate::types::SignalIDConnector>,
    default_working_signal: Option<crate::types::SignalIDConnector>,
    disabled_when_recipe_not_researched: Option<bool>,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    #[serde(default = "default_enable_logistic_control_behavior")]
    enable_logistic_control_behavior: bool,
    fixed_quality: Option<crate::types::QualityID>,
    #[serde(default = "default_fixed_recipe")]
    fixed_recipe: crate::types::RecipeID,
    #[serde(default = "default_fluid_boxes_off_when_no_fluid_recipe")]
    fluid_boxes_off_when_no_fluid_recipe: bool,
    #[serde(default = "default_gui_title_key")]
    gui_title_key: String,
    #[serde(default = "default_ingredient_count")]
    ingredient_count: u8,
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
fn default_enable_logistic_control_behavior() -> bool {
    true
}
fn default_fixed_recipe() -> crate::types::RecipeID {
    String::from("")
}
fn default_fluid_boxes_off_when_no_fluid_recipe() -> bool {
    false
}
fn default_gui_title_key() -> String {
    String::from("")
}
fn default_ingredient_count() -> u8 {
    255
}
