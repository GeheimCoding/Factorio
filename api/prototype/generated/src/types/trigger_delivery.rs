#[derive(serde::Deserialize)]
pub enum TriggerDelivery {
    #[serde(untagged)]
    InstantTriggerDelivery(Box<crate::types::InstantTriggerDelivery>),
    #[serde(untagged)]
    ProjectileTriggerDelivery(Box<crate::types::ProjectileTriggerDelivery>),
    #[serde(untagged)]
    BeamTriggerDelivery(Box<crate::types::BeamTriggerDelivery>),
    #[serde(untagged)]
    StreamTriggerDelivery(Box<crate::types::StreamTriggerDelivery>),
    #[serde(untagged)]
    ArtilleryTriggerDelivery(Box<crate::types::ArtilleryTriggerDelivery>),
    #[serde(untagged)]
    ChainTriggerDelivery(Box<crate::types::ChainTriggerDelivery>),
    #[serde(untagged)]
    DelayedTriggerDelivery(Box<crate::types::DelayedTriggerDelivery>),
}
