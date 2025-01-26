#[derive(Debug, serde::Deserialize)]
pub enum Trigger {
    #[serde(untagged)]
    TriggerVariants(TriggerVariants),
    #[serde(untagged)]
    VecTriggerVariants(Vec<TriggerVariants>),
}
#[derive(Debug, serde::Deserialize)]
pub enum TriggerVariants {
    #[serde(untagged)]
    DirectTriggerItem(Box<crate::types::DirectTriggerItem>),
    #[serde(untagged)]
    AreaTriggerItem(Box<crate::types::AreaTriggerItem>),
    #[serde(untagged)]
    LineTriggerItem(Box<crate::types::LineTriggerItem>),
    #[serde(untagged)]
    ClusterTriggerItem(Box<crate::types::ClusterTriggerItem>),
}
