#[derive(serde::Deserialize)]
pub struct EnterVehicleTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    match_type_only: bool,
    #[serde(rename = "type")]
    type_: String,
    vehicle: crate::types::EntityID,
}
