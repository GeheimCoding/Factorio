#[derive(Debug, serde::Deserialize)]
pub struct NeighbourConnectable {
    #[serde(default = "default_affected_by_direction")]
    affected_by_direction: bool,
    connections: crate::vec::Vec<crate::types::NeighbourConnectableConnectionDefinition>,
    #[serde(default = "default_neighbour_search_distance")]
    neighbour_search_distance: f32,
}
fn default_affected_by_direction() -> bool {
    true
}
fn default_neighbour_search_distance() -> f32 {
    0.7
}
