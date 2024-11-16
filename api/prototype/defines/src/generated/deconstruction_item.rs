pub enum TileSelectionMode {
    Always,
    Never,
    Normal,
    Only,
}
pub enum TileFilterMode {
    Blacklist,
    Whitelist,
}
pub enum EntityFilterMode {
    Blacklist,
    Whitelist,
}
pub enum DeconstructionItem {
    EntityFilterMode(EntityFilterMode),
    TileFilterMode(TileFilterMode),
    TileSelectionMode(TileSelectionMode),
}
