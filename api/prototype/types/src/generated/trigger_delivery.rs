pub enum TriggerDelivery {
    InstantTriggerDelivery(Box<InstantTriggerDelivery>),
    ProjectileTriggerDelivery(Box<ProjectileTriggerDelivery>),
    BeamTriggerDelivery(Box<BeamTriggerDelivery>),
    StreamTriggerDelivery(Box<StreamTriggerDelivery>),
    ArtilleryTriggerDelivery(Box<ArtilleryTriggerDelivery>),
    ChainTriggerDelivery(Box<ChainTriggerDelivery>),
    DelayedTriggerDelivery(Box<DelayedTriggerDelivery>),
}
