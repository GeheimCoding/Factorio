#[derive(serde::Deserialize)]
pub struct ShootTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    target: ShootTipTriggerTarget,
    #[serde(rename = "type")]
    type_: String,
}
#[derive(serde::Deserialize)]
pub enum ShootTipTriggerTarget {
    #[serde(rename = "enemy")]
    Enemy,
    #[serde(rename = "entity")]
    Entity,
}
