pub enum Trigger {
    TriggerVariants(TriggerVariants),
    VecTriggerVariants(Vec<TriggerVariants>),
}
pub enum TriggerVariants {
    DirectTriggerItem(Box<DirectTriggerItem>),
    AreaTriggerItem(Box<AreaTriggerItem>),
    LineTriggerItem(Box<LineTriggerItem>),
    ClusterTriggerItem(Box<ClusterTriggerItem>),
}
