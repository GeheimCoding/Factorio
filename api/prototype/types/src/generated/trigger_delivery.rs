pub enum TriggerDelivery {
    InstantTriggerDelivery(InstantTriggerDelivery),
    ProjectileTriggerDelivery(ProjectileTriggerDelivery),
    BeamTriggerDelivery(BeamTriggerDelivery),
    StreamTriggerDelivery(StreamTriggerDelivery),
    ArtilleryTriggerDelivery(ArtilleryTriggerDelivery),
    ChainTriggerDelivery(ChainTriggerDelivery),
    DelayedTriggerDelivery(DelayedTriggerDelivery),
}
