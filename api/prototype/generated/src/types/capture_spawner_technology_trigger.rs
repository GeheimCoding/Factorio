#[derive(serde::Deserialize)]
pub struct CaptureSpawnerTechnologyTrigger {
    entity: crate::types::EntityID,
    #[serde(rename = "type")]
    type_: String,
}
