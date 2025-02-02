#[derive(Debug, serde::Deserialize)]
pub struct GroupAttackTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
}
