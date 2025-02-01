#[derive(Debug, serde::Deserialize)]
pub struct EnterVehicleTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(default = "default_match_type_only")]
    match_type_only: bool,
    vehicle: Option<crate::types::EntityID>,
}
fn default_match_type_only() -> bool {
    false
}
