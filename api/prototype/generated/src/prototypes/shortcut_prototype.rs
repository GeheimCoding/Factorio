#[derive(serde::Deserialize)]
pub struct ShortcutPrototype {
    base_: crate::prototypes::Prototype,
    action: ShortcutPrototypeAction,
    #[serde(default = "default_associated_control_input")]
    associated_control_input: String,
    icon: crate::types::FileName,
    #[serde(default = "default_icon_size")]
    icon_size: crate::types::SpriteSizeType,
    icons: Vec<crate::types::IconData>,
    item_to_spawn: crate::types::ItemID,
    #[serde(default = "default_order")]
    order: crate::types::Order,
    small_icon: crate::types::FileName,
    #[serde(default = "default_small_icon_size")]
    small_icon_size: crate::types::SpriteSizeType,
    small_icons: Vec<crate::types::IconData>,
    #[serde(default = "default_style")]
    style: ShortcutPrototypeStyle,
    technology_to_unlock: crate::types::TechnologyID,
    #[serde(default = "default_toggleable")]
    toggleable: bool,
    #[serde(default = "default_unavailable_until_unlocked")]
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
fn default_associated_control_input() -> String {
    String::from("")
}
fn default_icon_size() -> crate::types::SpriteSizeType {
    64
}
fn default_order() -> crate::types::Order {
    String::from("")
}
fn default_small_icon_size() -> crate::types::SpriteSizeType {
    64
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
fn default_style() -> ShortcutPrototypeStyle {
    ShortcutPrototypeStyle::Default
}
fn default_toggleable() -> bool {
    false
}
fn default_unavailable_until_unlocked() -> bool {
    false
}
