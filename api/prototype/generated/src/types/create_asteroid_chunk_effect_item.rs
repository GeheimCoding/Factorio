pub struct CreateAsteroidChunkEffectItem {
    asteroid_name: crate::types::AsteroidChunkID,
    offset_deviation: crate::types::BoundingBox,
    offsets: Vec<crate::types::Vector>,
    type_: CreateAsteroidChunk,
}
