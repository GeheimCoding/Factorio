#[derive(serde::Deserialize)]
pub struct CreateAsteroidChunkEffectItem {
    base_: crate::types::TriggerEffectItem,
    asteroid_name: crate::types::AsteroidChunkID,
    offset_deviation: Option<crate::types::BoundingBox>,
    offsets: Option<Vec<crate::types::Vector>>,
    #[serde(rename = "type")]
    type_: String,
}
