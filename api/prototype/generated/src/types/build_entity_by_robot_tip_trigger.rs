#[derive(Debug, serde::Deserialize)]
pub struct BuildEntityByRobotTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(rename = "type")]
    type_: String,
}
