#[derive(Debug, serde::Deserialize)]
pub struct ShootTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
    target: Option<ShootTipTriggerTarget>,
}
#[derive(Debug, serde::Deserialize)]
pub enum ShootTipTriggerTarget {
    #[serde(rename = "enemy")]
    Enemy,
    #[serde(rename = "entity")]
    Entity,
}
