pub struct SelectionModeData {
    border_color: crate::types::Color,
    chart_color: crate::types::Color,
    count_button_color: crate::types::Color,
    cursor_box_type: crate::types::CursorBoxType,
    ended_sound: crate::types::Sound,
    entity_filter_mode: SelectionModeDataEntityFilterMode,
    entity_filters: Vec<crate::types::EntityID>,
    entity_type_filters: Vec<String>,
    mode: crate::types::SelectionModeFlags,
    play_ended_sound_when_nothing_selected: bool,
    started_sound: crate::types::Sound,
    tile_filter_mode: SelectionModeDataTileFilterMode,
    tile_filters: Vec<crate::types::TileID>,
}
#[derive(serde::Deserialize)]
pub enum SelectionModeDataEntityFilterMode {
    #[serde(rename = "whitelist")]
    Whitelist,
    #[serde(rename = "blacklist")]
    Blacklist,
}
#[derive(serde::Deserialize)]
pub enum SelectionModeDataTileFilterMode {
    #[serde(rename = "whitelist")]
    Whitelist,
    #[serde(rename = "blacklist")]
    Blacklist,
}
