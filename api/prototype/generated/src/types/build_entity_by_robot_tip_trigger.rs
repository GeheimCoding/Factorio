#[derive(serde::Deserialize)]
pub struct BuildEntityByRobotTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    type_: String,
}
