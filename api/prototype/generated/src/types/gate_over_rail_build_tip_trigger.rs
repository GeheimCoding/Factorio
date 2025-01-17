#[derive(serde::Deserialize)]
pub struct GateOverRailBuildTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    type_: String,
}
