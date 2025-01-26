#[derive(Debug, serde::Deserialize)]
pub struct DeliverImpactCombination {
    deliver_category: String,
    impact_category: String,
    name: String,
    trigger_effect_item: crate::types::TriggerEffectItem,
    #[serde(rename = "type")]
    type_: String,
}
