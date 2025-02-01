#[derive(Debug, serde::Deserialize)]
pub struct CustomInputPrototype {
    base_: crate::prototypes::Prototype,
    #[serde(default = "default_action")]
    action: CustomInputPrototypeAction,
    alternative_key_sequence: Option<String>,
    #[serde(default = "default_block_modifiers")]
    block_modifiers: bool,
    #[serde(default = "default_consuming")]
    consuming: crate::types::ConsumingType,
    controller_alternative_key_sequence: Option<String>,
    controller_key_sequence: Option<String>,
    #[serde(default = "default_enabled")]
    enabled: bool,
    #[serde(default = "default_enabled_while_in_cutscene")]
    enabled_while_in_cutscene: bool,
    #[serde(default = "default_enabled_while_spectating")]
    enabled_while_spectating: bool,
    #[serde(default = "default_include_selected_prototype")]
    include_selected_prototype: bool,
    item_to_spawn: Option<crate::types::ItemID>,
    key_sequence: String,
    linked_game_control: Option<crate::types::LinkedGameControl>,
    name: String,
}
#[derive(Debug, serde::Deserialize)]
pub enum CustomInputPrototypeAction {
    #[serde(rename = "lua")]
    Lua,
    #[serde(rename = "spawn-item")]
    SpawnItem,
    #[serde(rename = "toggle-personal-roboport")]
    TogglePersonalRoboport,
    #[serde(rename = "toggle-personal-logistic-requests")]
    TogglePersonalLogisticRequests,
    #[serde(rename = "toggle-equipment-movement-bonus")]
    ToggleEquipmentMovementBonus,
}
fn default_action() -> CustomInputPrototypeAction {
    CustomInputPrototypeAction::Lua
}
fn default_block_modifiers() -> bool {
    false
}
fn default_consuming() -> crate::types::ConsumingType {
    crate::types::ConsumingType::None
}
fn default_enabled() -> bool {
    true
}
fn default_enabled_while_in_cutscene() -> bool {
    false
}
fn default_enabled_while_spectating() -> bool {
    false
}
fn default_include_selected_prototype() -> bool {
    false
}
