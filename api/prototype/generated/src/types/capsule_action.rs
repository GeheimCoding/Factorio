#[derive(serde::Deserialize)]
pub enum CapsuleAction {
    #[serde(untagged)]
    ThrowCapsuleAction(Box<crate::types::ThrowCapsuleAction>),
    #[serde(untagged)]
    ActivateEquipmentCapsuleAction(Box<crate::types::ActivateEquipmentCapsuleAction>),
    #[serde(untagged)]
    UseOnSelfCapsuleAction(Box<crate::types::UseOnSelfCapsuleAction>),
    #[serde(untagged)]
    DestroyCliffsCapsuleAction(Box<crate::types::DestroyCliffsCapsuleAction>),
    #[serde(untagged)]
    ArtilleryRemoteCapsuleAction(Box<crate::types::ArtilleryRemoteCapsuleAction>),
}
