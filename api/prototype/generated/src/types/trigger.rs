#[derive(Debug, serde::Deserialize)]
pub enum Trigger {
    #[serde(untagged)]
    TriggerVariants(TriggerVariants),
    #[serde(untagged)]
    VecTriggerVariants(crate::vec::Vec<TriggerVariants>),
}
#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum TriggerVariants {
    #[serde(rename = "direct")]
    DirectTriggerItem(Box<crate::types::DirectTriggerItem>),
    #[serde(rename = "area")]
    AreaTriggerItem(Box<crate::types::AreaTriggerItem>),
    #[serde(rename = "line")]
    LineTriggerItem(Box<crate::types::LineTriggerItem>),
    #[serde(rename = "cluster")]
    ClusterTriggerItem(Box<crate::types::ClusterTriggerItem>),
}
