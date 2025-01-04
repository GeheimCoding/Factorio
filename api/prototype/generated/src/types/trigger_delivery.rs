pub enum TriggerDelivery {
    InstantTriggerDelivery(Box<crate::types::InstantTriggerDelivery>),
    ProjectileTriggerDelivery(Box<crate::types::ProjectileTriggerDelivery>),
    BeamTriggerDelivery(Box<crate::types::BeamTriggerDelivery>),
    StreamTriggerDelivery(Box<crate::types::StreamTriggerDelivery>),
    ArtilleryTriggerDelivery(Box<crate::types::ArtilleryTriggerDelivery>),
    ChainTriggerDelivery(Box<crate::types::ChainTriggerDelivery>),
    DelayedTriggerDelivery(Box<crate::types::DelayedTriggerDelivery>),
}
