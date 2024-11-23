#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum ChunkGeneratedStatus {
    BasicTiles = 20,
    CorrectedTiles = 30,
    CustomTiles = 10,
    Entities = 50,
    Nothing = 0,
    Tiles = 40,
}
