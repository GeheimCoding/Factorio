#[derive(Debug, serde::Deserialize)]
pub struct UseRailPlannerTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    build_mode: crate::types::BuildMode,
    #[serde(rename = "type")]
    type_: String,
}
