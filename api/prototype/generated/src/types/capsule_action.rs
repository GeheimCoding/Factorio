pub enum CapsuleAction {
    ThrowCapsuleAction(Box<crate::types::ThrowCapsuleAction>),
    ActivateEquipmentCapsuleAction(Box<crate::types::ActivateEquipmentCapsuleAction>),
    UseOnSelfCapsuleAction(Box<crate::types::UseOnSelfCapsuleAction>),
    DestroyCliffsCapsuleAction(Box<crate::types::DestroyCliffsCapsuleAction>),
    ArtilleryRemoteCapsuleAction(Box<crate::types::ArtilleryRemoteCapsuleAction>),
}
