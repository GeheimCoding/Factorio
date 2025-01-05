pub struct PipeConnectionDefinition {
    connection_category: PipeConnectionDefinitionConnectionCategory,
    connection_type: PipeConnectionType,
    direction: crate::types::Direction,
    enable_working_visualisations: Vec<String>,
    flow_direction: PipeConnectionDefinitionFlowDirection,
    linked_connection_id: crate::types::FluidBoxLinkedConnectionID,
    max_distance_tint: crate::types::Color,
    max_underground_distance: u8,
    position: crate::types::MapPosition,
    positions: (
        crate::types::MapPosition,
        crate::types::MapPosition,
        crate::types::MapPosition,
        crate::types::MapPosition,
    ),
    underground_collision_mask: crate::types::CollisionMaskConnector,
}
pub enum PipeConnectionDefinitionConnectionCategory {
    String(String),
    VecString(Vec<String>),
}
pub enum PipeConnectionType {
    Normal,
    Underground,
    Linked,
}
pub enum PipeConnectionDefinitionFlowDirection {
    InputOutput,
    Input,
    Output,
}
