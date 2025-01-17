#[derive(serde::Deserialize)]
pub struct NeighbourConnectableConnectionDefinition {
    category: crate::types::NeighbourConnectableConnectionCategory,
    location: MapLocation,
    neighbour_category: Vec<crate::types::NeighbourConnectableConnectionCategory>,
}
#[derive(serde::Deserialize)]
pub struct MapLocation {
    direction: crate::types::MapPosition,
    position: crate::types::MapPosition,
}
