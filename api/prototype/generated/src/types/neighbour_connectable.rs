pub struct NeighbourConnectable {
    affected_by_direction: bool,
    connections: Vec<crate::types::NeighbourConnectableConnectionDefinition>,
    neighbour_search_distance: f32,
}
