pub struct NeighbourConnectable {
    affected_by_direction: bool,
    connections: Vec<NeighbourConnectableConnectionDefinition>,
    neighbour_search_distance: f32,
}
