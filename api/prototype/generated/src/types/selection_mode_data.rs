#[derive(Debug, serde::Deserialize)]
pub struct SelectionModeData {
    border_color: crate::types::Color,
    // default: Value of border_color
    chart_color: Option<crate::types::Color>,
    // default: Value of border_color
    count_button_color: Option<crate::types::Color>,
    cursor_box_type: crate::types::CursorBoxType,
    ended_sound: Option<crate::types::Sound>,
    #[serde(default = "default_entity_filter_mode")]
    entity_filter_mode: SelectionModeDataEntityFilterMode,
    entity_filters: Option<Vec<crate::types::EntityID>>,
    entity_type_filters: Option<Vec<String>>,
    mode: crate::types::SelectionModeFlags,
    #[serde(default = "default_play_ended_sound_when_nothing_selected")]
    play_ended_sound_when_nothing_selected: bool,
    started_sound: Option<crate::types::Sound>,
    #[serde(default = "default_tile_filter_mode")]
    tile_filter_mode: SelectionModeDataTileFilterMode,
    tile_filters: Option<Vec<crate::types::TileID>>,
}
#[derive(Debug, serde::Deserialize)]
pub enum SelectionModeDataEntityFilterMode {
    #[serde(rename = "whitelist")]
    Whitelist,
    #[serde(rename = "blacklist")]
    Blacklist,
}
fn default_entity_filter_mode() -> SelectionModeDataEntityFilterMode {
    SelectionModeDataEntityFilterMode::Whitelist
}
fn default_play_ended_sound_when_nothing_selected() -> bool {
    false
}
#[derive(Debug, serde::Deserialize)]
pub enum SelectionModeDataTileFilterMode {
    #[serde(rename = "whitelist")]
    Whitelist,
    #[serde(rename = "blacklist")]
    Blacklist,
}
fn default_tile_filter_mode() -> SelectionModeDataTileFilterMode {
    SelectionModeDataTileFilterMode::Whitelist
}
