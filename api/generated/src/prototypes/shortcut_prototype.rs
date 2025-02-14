#[derive(Debug, serde::Deserialize)]
pub struct ShortcutPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::Prototype,
    action: ShortcutPrototypeAction,
    #[serde(default = "default_associated_control_input")]
    associated_control_input: String,
    icon: Option<crate::types::FileName>,
    #[serde(default = "default_icon_size")]
    icon_size: crate::types::SpriteSizeType,
    icons: Option<crate::vec::Vec<crate::types::IconData>>,
    item_to_spawn: Option<crate::types::ItemID>,
    #[serde(default = "default_order")]
    order: crate::types::Order,
    small_icon: Option<crate::types::FileName>,
    #[serde(default = "default_small_icon_size")]
    small_icon_size: crate::types::SpriteSizeType,
    small_icons: Option<crate::vec::Vec<crate::types::IconData>>,
    #[serde(default = "default_style")]
    style: ShortcutPrototypeStyle,
    technology_to_unlock: Option<crate::types::TechnologyID>,
    #[serde(default = "default_toggleable")]
    toggleable: bool,
    #[serde(default = "default_unavailable_until_unlocked")]
    unavailable_until_unlocked: bool,
}
#[derive(Debug, serde::Deserialize)]
pub enum ShortcutPrototypeAction {
    #[serde(rename = "toggle-alt-mode")]
    ToggleAltMode,
    #[serde(rename = "undo")]
    Undo,
    #[serde(rename = "copy")]
    Copy,
    #[serde(rename = "cut")]
    Cut,
    #[serde(rename = "paste")]
    Paste,
    #[serde(rename = "import-string")]
    ImportString,
    #[serde(rename = "toggle-personal-roboport")]
    TogglePersonalRoboport,
    #[serde(rename = "toggle-equipment-movement-bonus")]
    ToggleEquipmentMovementBonus,
    #[serde(rename = "spawn-item")]
    SpawnItem,
    #[serde(rename = "lua")]
    Lua,
    #[serde(rename = "redo")]
    Redo,
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
#[derive(Debug, serde::Deserialize)]
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
