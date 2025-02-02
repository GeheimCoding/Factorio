#[derive(Debug, serde::Deserialize)]
pub struct GameViewSettings {
    default_show_value: Option<bool>,
    #[serde(default = "default_show_alert_gui")]
    show_alert_gui: bool,
    #[serde(default = "default_show_controller_gui")]
    show_controller_gui: bool,
    #[serde(default = "default_show_crafting_queue")]
    show_crafting_queue: bool,
    #[serde(default = "default_show_entity_info")]
    show_entity_info: bool,
    #[serde(default = "default_show_entity_tooltip")]
    show_entity_tooltip: bool,
    #[serde(default = "default_show_hotkey_suggestions")]
    show_hotkey_suggestions: bool,
    #[serde(default = "default_show_map_view_options")]
    show_map_view_options: bool,
    #[serde(default = "default_show_minimap")]
    show_minimap: bool,
    #[serde(default = "default_show_quickbar")]
    show_quickbar: bool,
    #[serde(default = "default_show_rail_block_visualisation")]
    show_rail_block_visualisation: bool,
    #[serde(default = "default_show_research_info")]
    show_research_info: bool,
    #[serde(default = "default_show_shortcut_bar")]
    show_shortcut_bar: bool,
    #[serde(default = "default_show_side_menu")]
    show_side_menu: bool,
    #[serde(default = "default_show_surface_list")]
    show_surface_list: bool,
    #[serde(default = "default_show_tool_bar")]
    show_tool_bar: bool,
    #[serde(default = "default_update_entity_selection")]
    update_entity_selection: bool,
}
fn default_show_alert_gui() -> bool {
    true
}
fn default_show_controller_gui() -> bool {
    true
}
fn default_show_crafting_queue() -> bool {
    true
}
fn default_show_entity_info() -> bool {
    false
}
fn default_show_entity_tooltip() -> bool {
    true
}
fn default_show_hotkey_suggestions() -> bool {
    true
}
fn default_show_map_view_options() -> bool {
    true
}
fn default_show_minimap() -> bool {
    true
}
fn default_show_quickbar() -> bool {
    true
}
fn default_show_rail_block_visualisation() -> bool {
    false
}
fn default_show_research_info() -> bool {
    true
}
fn default_show_shortcut_bar() -> bool {
    true
}
fn default_show_side_menu() -> bool {
    true
}
fn default_show_surface_list() -> bool {
    true
}
fn default_show_tool_bar() -> bool {
    true
}
fn default_update_entity_selection() -> bool {
    true
}
