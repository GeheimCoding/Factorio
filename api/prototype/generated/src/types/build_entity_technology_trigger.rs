#[derive(serde::Deserialize)]
pub struct BuildEntityTechnologyTrigger {
    entity: crate::types::EntityIDFilter,
    #[serde(rename = "type")]
    type_: String,
}
