pub struct ShortcutPrototype {
    base_: crate::prototypes::Prototype,
    action: ShortcutPrototypeAction,
    associated_control_input: String,
    icon: crate::types::FileName,
    icon_size: crate::types::SpriteSizeType,
    icons: Vec<crate::types::IconData>,
    item_to_spawn: crate::types::ItemID,
    order: crate::types::Order,
    small_icon: crate::types::FileName,
    small_icon_size: crate::types::SpriteSizeType,
    small_icons: Vec<crate::types::IconData>,
    style: ShortcutPrototypeStyle,
    technology_to_unlock: crate::types::TechnologyID,
    toggleable: bool,
    unavailable_until_unlocked: bool,
}
#[derive(serde::Deserialize)]
pub enum ShortcutPrototypeAction {
    #[serde(rename = "toggle_alt_mode")]
    ToggleAltMode,
    #[serde(rename = "undo")]
    Undo,
    #[serde(rename = "copy")]
    Copy,
    #[serde(rename = "cut")]
    Cut,
    #[serde(rename = "paste")]
    Paste,
    #[serde(rename = "import_string")]
    ImportString,
    #[serde(rename = "toggle_personal_roboport")]
    TogglePersonalRoboport,
    #[serde(rename = "toggle_equipment_movement_bonus")]
    ToggleEquipmentMovementBonus,
    #[serde(rename = "spawn_item")]
    SpawnItem,
    #[serde(rename = "lua")]
    Lua,
}
#[derive(serde::Deserialize)]
pub enum ShortcutPrototypeStyle {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "green")]
    Green,
}
