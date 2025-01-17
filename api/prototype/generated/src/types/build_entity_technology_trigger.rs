#[derive(serde::Deserialize)]
pub struct BuildEntityTechnologyTrigger {
    entity: crate::types::EntityIDFilter,
    type_: String,
}
