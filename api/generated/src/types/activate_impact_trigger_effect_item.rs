#[derive(Debug, serde::Deserialize)]
pub struct ActivateImpactTriggerEffectItem {
    #[serde(flatten)]
    base_: crate::types::TriggerEffectItem,
    #[serde(default = "default_deliver_category")]
    deliver_category: String,
}
fn default_deliver_category() -> String {
    String::from("")
}
