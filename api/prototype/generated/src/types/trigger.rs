pub enum Trigger {
    TriggerVariants(TriggerVariants),
    VecTriggerVariants(Vec<TriggerVariants>),
}
pub enum TriggerVariants {
    DirectTriggerItem(Box<crate::types::DirectTriggerItem>),
    AreaTriggerItem(Box<crate::types::AreaTriggerItem>),
    LineTriggerItem(Box<crate::types::LineTriggerItem>),
    ClusterTriggerItem(Box<crate::types::ClusterTriggerItem>),
}
