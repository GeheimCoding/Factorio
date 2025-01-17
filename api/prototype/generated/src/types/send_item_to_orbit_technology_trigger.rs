#[derive(serde::Deserialize)]
pub struct SendItemToOrbitTechnologyTrigger {
    item: crate::types::ItemIDFilter,
    type_: String,
}
