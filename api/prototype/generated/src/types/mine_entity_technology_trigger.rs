#[derive(Debug, serde::Deserialize)]
pub struct MineEntityTechnologyTrigger {
    entity: crate::types::EntityID,
    #[serde(rename = "type")]
    type_: String,
}
