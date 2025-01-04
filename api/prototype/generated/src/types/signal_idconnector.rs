pub struct SignalIDConnector {
    name: SignalIDConnectorName,
    type_: SignalIDConnectorType,
}
pub enum SignalIDConnectorName {
    VirtualSignalID(crate::types::VirtualSignalID),
    ItemID(crate::types::ItemID),
    FluidID(crate::types::FluidID),
    RecipeID(crate::types::RecipeID),
    EntityID(crate::types::EntityID),
    SpaceLocationID(crate::types::SpaceLocationID),
    AsteroidChunkID(crate::types::AsteroidChunkID),
    QualityID(crate::types::QualityID),
}
pub enum SignalIDConnectorType {
    Virtual,
    Item,
    Fluid,
    Recipe,
    Entity,
    SpaceLocation,
    AsteroidChunk,
    Quality,
}
