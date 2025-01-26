#[derive(Debug, serde::Deserialize)]
pub struct MineItemByRobotTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(rename = "type")]
    type_: String,
}
