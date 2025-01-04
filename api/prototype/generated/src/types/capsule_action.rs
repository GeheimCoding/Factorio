pub enum CapsuleAction {
    ThrowCapsuleAction(Box<ThrowCapsuleAction>),
    ActivateEquipmentCapsuleAction(Box<ActivateEquipmentCapsuleAction>),
    UseOnSelfCapsuleAction(Box<UseOnSelfCapsuleAction>),
    DestroyCliffsCapsuleAction(Box<DestroyCliffsCapsuleAction>),
    ArtilleryRemoteCapsuleAction(Box<ArtilleryRemoteCapsuleAction>),
}
