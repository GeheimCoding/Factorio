#[derive(Debug, serde::Deserialize)]
pub struct GateOverRailBuildTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(rename = "type")]
    type_: String,
}
