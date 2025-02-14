#[derive(Debug, serde::Deserialize)]
pub struct GateOverRailBuildTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
}
