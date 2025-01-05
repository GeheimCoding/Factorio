pub struct NeighbourConnectableConnectionDefinition {
    category: crate::types::NeighbourConnectableConnectionCategory,
    location: MapLocation,
    neighbour_category: Vec<crate::types::NeighbourConnectableConnectionCategory>,
}
pub struct MapLocation {
    direction: crate::types::MapPosition,
    position: crate::types::MapPosition,
}
