#[derive(serde::Deserialize)]
pub struct MineEntityTechnologyTrigger {
    entity: crate::types::EntityID,
    type_: String,
}
