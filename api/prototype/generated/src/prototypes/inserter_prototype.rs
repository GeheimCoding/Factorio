#[derive(serde::Deserialize)]
pub struct InserterPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    #[serde(default = "default_allow_burner_leech")]
    allow_burner_leech: bool,
    #[serde(default = "default_allow_custom_vectors")]
    allow_custom_vectors: bool,
    #[serde(default = "default_bulk")]
    bulk: bool,
    #[serde(default = "default_chases_belt_items")]
    chases_belt_items: bool,
    circuit_connector: (
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
    ),
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    default_stack_control_input_signal: crate::types::SignalIDConnector,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    #[serde(default = "default_draw_held_item")]
    draw_held_item: bool,
    #[serde(default = "default_draw_inserter_arrow")]
    draw_inserter_arrow: bool,
    #[serde(default = "default_energy_per_movement")]
    energy_per_movement: crate::types::Energy,
    #[serde(default = "default_energy_per_rotation")]
    energy_per_rotation: crate::types::Energy,
    energy_source: crate::types::EnergySource,
    #[serde(default = "default_enter_drop_mode_if_held_stack_spoiled")]
    enter_drop_mode_if_held_stack_spoiled: bool,
    extension_speed: f64,
    #[serde(default = "default_filter_count")]
    filter_count: u8,
    #[serde(default = "default_grab_less_to_match_belt_stack")]
    grab_less_to_match_belt_stack: bool,
    hand_base_frozen: crate::types::Sprite,
    hand_base_picture: crate::types::Sprite,
    hand_base_shadow: crate::types::Sprite,
    hand_closed_frozen: crate::types::Sprite,
    hand_closed_picture: crate::types::Sprite,
    hand_closed_shadow: crate::types::Sprite,
    hand_open_frozen: crate::types::Sprite,
    hand_open_picture: crate::types::Sprite,
    hand_open_shadow: crate::types::Sprite,
    #[serde(default = "default_hand_size")]
    hand_size: f64,
    insert_position: crate::types::Vector,
    #[serde(default = "default_max_belt_stack_size")]
    max_belt_stack_size: u8,
    pickup_position: crate::types::Vector,
    platform_frozen: crate::types::Sprite4Way,
    platform_picture: crate::types::Sprite4Way,
    rotation_speed: f64,
    #[serde(default = "default_stack_size_bonus")]
    stack_size_bonus: u8,
    #[serde(default = "default_use_easter_egg")]
    use_easter_egg: bool,
    #[serde(default = "default_wait_for_full_hand")]
    wait_for_full_hand: bool,
}
fn default_allow_burner_leech() -> bool {
    false
}
fn default_allow_custom_vectors() -> bool {
    false
}
fn default_bulk() -> bool {
    false
}
fn default_chases_belt_items() -> bool {
    true
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
fn default_draw_held_item() -> bool {
    true
}
fn default_draw_inserter_arrow() -> bool {
    true
}
fn default_energy_per_movement() -> crate::types::Energy {
    String::from("0")
}
fn default_energy_per_rotation() -> crate::types::Energy {
    String::from("0")
}
fn default_enter_drop_mode_if_held_stack_spoiled() -> bool {
    false
}
fn default_filter_count() -> u8 {
    0
}
fn default_grab_less_to_match_belt_stack() -> bool {
    false
}
fn default_hand_size() -> f64 {
    0.8
}
fn default_max_belt_stack_size() -> u8 {
    1
}
fn default_stack_size_bonus() -> u8 {
    0
}
fn default_use_easter_egg() -> bool {
    true
}
fn default_wait_for_full_hand() -> bool {
    false
}
