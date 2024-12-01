pub struct PipeConnectionDefinition {
    connection_category: PipeConnectionDefinitionConnectionCategory,
    connection_type: PipeConnectionType,
    direction: Direction,
    enable_working_visualisations: Vec<String>,
    flow_direction: PipeConnectionDefinitionFlowDirection,
    linked_connection_id: FluidBoxLinkedConnectionID,
    max_distance_tint: Color,
    max_underground_distance: u8,
    position: MapPosition,
    positions: (MapPosition, MapPosition, MapPosition, MapPosition),
    underground_collision_mask: CollisionMaskConnector,
}
pub enum PipeConnectionDefinitionConnectionCategory {
    String(String),
    VecString(Vec<String>),
}
pub enum PipeConnectionDefinitionFlowDirection {
    InputOutput,
    Input,
    Output,
}
