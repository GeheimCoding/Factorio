pub struct CreateAsteroidChunkEffectItem {
    asteroid_name: AsteroidChunkID,
    offset_deviation: BoundingBox,
    offsets: Vec<Vector>,
    type_: CreateAsteroidChunk,
}
