#[derive(Debug, Clone)]
pub enum DeconstructionItem {
    EntityFilterMode(EntityFilterMode),
    TileFilterMode(TileFilterMode),
    TileSelectionMode(TileSelectionMode),
}
#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum EntityFilterMode {
    Blacklist = 1,
    Whitelist = 0,
}
#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum TileFilterMode {
    Blacklist = 1,
    Whitelist = 0,
}
#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum TileSelectionMode {
    Always = 1,
    Never = 2,
    Normal = 0,
    Only = 3,
}
