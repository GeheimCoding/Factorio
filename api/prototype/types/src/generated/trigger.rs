pub enum Trigger {
    Trigger(Trigger),
    VecTrigger(Vec<Trigger>),
}
pub enum Trigger {
    DirectTriggerItem(DirectTriggerItem),
    AreaTriggerItem(AreaTriggerItem),
    LineTriggerItem(LineTriggerItem),
    ClusterTriggerItem(ClusterTriggerItem),
}
