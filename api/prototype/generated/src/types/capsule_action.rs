#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum CapsuleAction {
    #[serde(rename = "throw")]
    ThrowCapsuleAction(Box<crate::types::ThrowCapsuleAction>),
    #[serde(rename = "equipment-remote")]
    ActivateEquipmentCapsuleAction(Box<crate::types::ActivateEquipmentCapsuleAction>),
    #[serde(rename = "use-on-self")]
    UseOnSelfCapsuleAction(Box<crate::types::UseOnSelfCapsuleAction>),
    #[serde(rename = "destroy-cliffs")]
    DestroyCliffsCapsuleAction(Box<crate::types::DestroyCliffsCapsuleAction>),
    #[serde(rename = "artillery-remote")]
    ArtilleryRemoteCapsuleAction(Box<crate::types::ArtilleryRemoteCapsuleAction>),
}
