#[derive(serde::Deserialize)]
pub struct PipeConnectionDefinition {
    #[serde(default = "default_connection_category")]
    connection_category: PipeConnectionDefinitionConnectionCategory,
    connection_type: PipeConnectionType,
    direction: crate::types::Direction,
    enable_working_visualisations: Vec<String>,
    flow_direction: PipeConnectionDefinitionFlowDirection,
    linked_connection_id: crate::types::FluidBoxLinkedConnectionID,
    max_distance_tint: crate::types::Color,
    #[serde(default = "default_max_underground_distance")]
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
#[derive(serde::Deserialize)]
pub enum PipeConnectionDefinitionConnectionCategory {
    #[serde(untagged)]
    String(String),
    #[serde(untagged)]
    VecString(Vec<String>),
}
fn default_connection_category() -> PipeConnectionDefinitionConnectionCategory {
    PipeConnectionDefinitionConnectionCategory::String(String::from("default"))
}
#[derive(serde::Deserialize)]
pub enum PipeConnectionType {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "underground")]
    Underground,
    #[serde(rename = "linked")]
    Linked,
}
#[derive(serde::Deserialize)]
pub enum PipeConnectionDefinitionFlowDirection {
    #[serde(rename = "input_output")]
    InputOutput,
    #[serde(rename = "input")]
    Input,
    #[serde(rename = "output")]
    Output,
}
fn default_max_underground_distance() -> u8 {
    0
}
