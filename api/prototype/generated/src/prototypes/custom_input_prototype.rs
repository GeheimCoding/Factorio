#[derive(serde::Deserialize)]
pub struct CustomInputPrototype {
    base_: crate::prototypes::Prototype,
    action: CustomInputPrototypeAction,
    alternative_key_sequence: String,
    block_modifiers: bool,
    consuming: crate::types::ConsumingType,
    controller_alternative_key_sequence: String,
    controller_key_sequence: String,
    enabled: bool,
    enabled_while_in_cutscene: bool,
    enabled_while_spectating: bool,
    include_selected_prototype: bool,
    item_to_spawn: crate::types::ItemID,
    key_sequence: String,
    linked_game_control: crate::types::LinkedGameControl,
    name: String,
}
#[derive(serde::Deserialize)]
pub enum CustomInputPrototypeAction {
    #[serde(rename = "lua")]
    Lua,
    #[serde(rename = "spawn_item")]
    SpawnItem,
    #[serde(rename = "toggle_personal_roboport")]
    TogglePersonalRoboport,
    #[serde(rename = "toggle_personal_logistic_requests")]
    TogglePersonalLogisticRequests,
    #[serde(rename = "toggle_equipment_movement_bonus")]
    ToggleEquipmentMovementBonus,
}
