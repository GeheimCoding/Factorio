#[derive(Debug, serde::Deserialize)]
pub struct PipeConnectionDefinition {
    #[serde(default = "default_connection_category")]
    connection_category: PipeConnectionDefinitionConnectionCategory,
    #[serde(default = "default_connection_type")]
    connection_type: PipeConnectionType,
    direction: Option<crate::types::Direction>,
    enable_working_visualisations: Option<crate::vec::Vec<String>>,
    #[serde(default = "default_flow_direction")]
    flow_direction: PipeConnectionDefinitionFlowDirection,
    linked_connection_id: Option<crate::types::FluidBoxLinkedConnectionID>,
    max_distance_tint: Option<crate::types::Color>,
    #[serde(default = "default_max_underground_distance")]
    max_underground_distance: u8,
    position: Option<crate::types::MapPosition>,
    positions: Option<(
        Box<crate::types::MapPosition>,
        Box<crate::types::MapPosition>,
        Box<crate::types::MapPosition>,
        Box<crate::types::MapPosition>,
    )>,
    underground_collision_mask: Option<crate::types::CollisionMaskConnector>,
}
#[derive(Debug, serde::Deserialize)]
pub enum PipeConnectionDefinitionConnectionCategory {
    #[serde(untagged)]
    String(String),
    #[serde(untagged)]
    VecString(crate::vec::Vec<String>),
}
fn default_connection_category() -> PipeConnectionDefinitionConnectionCategory {
    PipeConnectionDefinitionConnectionCategory::String(String::from("default"))
}
#[derive(Debug, serde::Deserialize)]
pub enum PipeConnectionType {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "underground")]
    Underground,
    #[serde(rename = "linked")]
    Linked,
}
fn default_connection_type() -> PipeConnectionType {
    PipeConnectionType::Normal
}
#[derive(Debug, serde::Deserialize)]
pub enum PipeConnectionDefinitionFlowDirection {
    #[serde(rename = "input-output")]
    InputOutput,
    #[serde(rename = "input")]
    Input,
    #[serde(rename = "output")]
    Output,
}
fn default_flow_direction() -> PipeConnectionDefinitionFlowDirection {
    PipeConnectionDefinitionFlowDirection::InputOutput
}
fn default_max_underground_distance() -> u8 {
    0
}
