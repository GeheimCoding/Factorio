#[derive(Debug, serde::Deserialize)]
pub struct CreateAsteroidChunkEffectItem {
    #[serde(flatten)]
    base_: crate::types::TriggerEffectItem,
    asteroid_name: crate::types::AsteroidChunkID,
    offset_deviation: Option<crate::types::BoundingBox>,
    offsets: Option<crate::vec::Vec<crate::types::Vector>>,
}
