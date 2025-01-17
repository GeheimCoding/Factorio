#[derive(serde::Deserialize)]
pub struct GroupAttackTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    type_: String,
}
