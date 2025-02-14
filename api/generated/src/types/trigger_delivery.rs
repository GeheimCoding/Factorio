#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum TriggerDelivery {
    #[serde(rename = "instant")]
    InstantTriggerDelivery(Box<crate::types::InstantTriggerDelivery>),
    #[serde(rename = "projectile")]
    ProjectileTriggerDelivery(Box<crate::types::ProjectileTriggerDelivery>),
    #[serde(rename = "beam")]
    BeamTriggerDelivery(Box<crate::types::BeamTriggerDelivery>),
    #[serde(rename = "stream")]
    StreamTriggerDelivery(Box<crate::types::StreamTriggerDelivery>),
    #[serde(rename = "artillery")]
    ArtilleryTriggerDelivery(Box<crate::types::ArtilleryTriggerDelivery>),
    #[serde(rename = "chain")]
    ChainTriggerDelivery(Box<crate::types::ChainTriggerDelivery>),
    #[serde(rename = "delayed")]
    DelayedTriggerDelivery(Box<crate::types::DelayedTriggerDelivery>),
}
