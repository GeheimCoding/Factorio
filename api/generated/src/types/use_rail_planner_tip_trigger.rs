#[derive(Debug, serde::Deserialize)]
pub struct UseRailPlannerTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
    build_mode: crate::types::BuildMode,
}
