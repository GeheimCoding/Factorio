#[derive(Debug, serde::Deserialize)]
pub struct NeighbourConnectableConnectionDefinition {
    category: crate::types::NeighbourConnectableConnectionCategory,
    location: MapLocation,
    neighbour_category: Option<Vec<crate::types::NeighbourConnectableConnectionCategory>>,
}
#[derive(Debug, serde::Deserialize)]
pub struct MapLocation {
    direction: crate::types::MapPosition,
    position: crate::types::MapPosition,
}
