#[derive(Debug, serde::Deserialize)]
pub struct MineItemByRobotTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
}
