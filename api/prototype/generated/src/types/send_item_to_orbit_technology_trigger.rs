#[derive(Debug, serde::Deserialize)]
pub struct SendItemToOrbitTechnologyTrigger {
    item: crate::types::ItemIDFilter,
    #[serde(rename = "type")]
    type_: String,
}
