#[derive(serde::Deserialize)]
pub struct GameViewSettings {
    default_show_value: bool,
    show_alert_gui: bool,
    show_controller_gui: bool,
    show_crafting_queue: bool,
    show_entity_info: bool,
    show_entity_tooltip: bool,
    show_hotkey_suggestions: bool,
    show_map_view_options: bool,
    show_minimap: bool,
    show_quickbar: bool,
    show_rail_block_visualisation: bool,
    show_research_info: bool,
    show_shortcut_bar: bool,
    show_side_menu: bool,
    show_tool_bar: bool,
    update_entity_selection: bool,
}
