#[derive(serde::Deserialize)]
pub struct MineItemByRobotTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    type_: String,
}
