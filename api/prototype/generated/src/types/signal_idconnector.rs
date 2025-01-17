pub struct SignalIDConnector {
    name: SignalIDConnectorName,
    type_: SignalIDConnectorType,
}
#[derive(serde::Deserialize)]
pub enum SignalIDConnectorName {
    #[serde(untagged)]
    VirtualSignalID(crate::types::VirtualSignalID),
    #[serde(untagged)]
    ItemID(crate::types::ItemID),
    #[serde(untagged)]
    FluidID(crate::types::FluidID),
    #[serde(untagged)]
    RecipeID(crate::types::RecipeID),
    #[serde(untagged)]
    EntityID(crate::types::EntityID),
    #[serde(untagged)]
    SpaceLocationID(crate::types::SpaceLocationID),
    #[serde(untagged)]
    AsteroidChunkID(crate::types::AsteroidChunkID),
    #[serde(untagged)]
    QualityID(crate::types::QualityID),
}
#[derive(serde::Deserialize)]
pub enum SignalIDConnectorType {
    #[serde(rename = "virtual")]
    Virtual,
    #[serde(rename = "item")]
    Item,
    #[serde(rename = "fluid")]
    Fluid,
    #[serde(rename = "recipe")]
    Recipe,
    #[serde(rename = "entity")]
    Entity,
    #[serde(rename = "space_location")]
    SpaceLocation,
    #[serde(rename = "asteroid_chunk")]
    AsteroidChunk,
    #[serde(rename = "quality")]
    Quality,
}
