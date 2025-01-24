#[derive(serde::Deserialize)]
pub struct CaptureSpawnerTechnologyTrigger {
    entity: Option<crate::types::EntityID>,
    #[serde(rename = "type")]
    type_: String,
}
