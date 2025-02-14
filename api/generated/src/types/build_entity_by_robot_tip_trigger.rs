#[derive(Debug, serde::Deserialize)]
pub struct BuildEntityByRobotTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
}
