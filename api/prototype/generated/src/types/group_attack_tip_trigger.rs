#[derive(Debug, serde::Deserialize)]
pub struct GroupAttackTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(rename = "type")]
    type_: String,
}
