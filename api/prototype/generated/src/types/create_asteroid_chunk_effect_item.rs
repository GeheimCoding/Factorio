#[derive(serde::Deserialize)]
pub struct CreateAsteroidChunkEffectItem {
    base_: crate::types::TriggerEffectItem,
    asteroid_name: crate::types::AsteroidChunkID,
    offset_deviation: crate::types::BoundingBox,
    offsets: Vec<crate::types::Vector>,
    type_: String,
}
