pub enum Trigger {
    TriggerVariants(TriggerVariants),
    VecTriggerVariants(Vec<TriggerVariants>),
}
pub enum TriggerVariants {
    DirectTriggerItem(DirectTriggerItem),
    AreaTriggerItem(AreaTriggerItem),
    LineTriggerItem(LineTriggerItem),
    ClusterTriggerItem(ClusterTriggerItem),
}
