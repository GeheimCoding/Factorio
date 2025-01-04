pub struct SelectionModeData {
    border_color: Color,
    chart_color: Color,
    count_button_color: Color,
    cursor_box_type: CursorBoxType,
    ended_sound: Sound,
    entity_filter_mode: SelectionModeDataEntityFilterMode,
    entity_filters: Vec<EntityID>,
    entity_type_filters: Vec<String>,
    mode: SelectionModeFlags,
    play_ended_sound_when_nothing_selected: bool,
    started_sound: Sound,
    tile_filter_mode: SelectionModeDataTileFilterMode,
    tile_filters: Vec<TileID>,
}
pub enum SelectionModeDataEntityFilterMode {
    Whitelist,
    Blacklist,
}
pub enum SelectionModeDataTileFilterMode {
    Whitelist,
    Blacklist,
}
