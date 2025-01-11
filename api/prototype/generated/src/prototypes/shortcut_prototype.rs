pub struct ShortcutPrototype {
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
pub enum ShortcutPrototypeAction {
    ToggleAltMode,
    Undo,
    Copy,
    Cut,
    Paste,
    ImportString,
    TogglePersonalRoboport,
    ToggleEquipmentMovementBonus,
    SpawnItem,
    Lua,
}
pub enum ShortcutPrototypeStyle {
    Default,
    Blue,
    Red,
    Green,
}
