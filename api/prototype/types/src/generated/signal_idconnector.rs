pub struct SignalIDConnector {
    name: SignalIDConnectorName,
    type_: SignalIDConnectorType,
}
pub enum SignalIDConnectorName {
    VirtualSignalID(VirtualSignalID),
    ItemID(ItemID),
    FluidID(FluidID),
    RecipeID(RecipeID),
    EntityID(EntityID),
    SpaceLocationID(SpaceLocationID),
    AsteroidChunkID(AsteroidChunkID),
    QualityID(QualityID),
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
