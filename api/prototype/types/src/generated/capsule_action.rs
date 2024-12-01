pub enum CapsuleAction {
    ThrowCapsuleAction(ThrowCapsuleAction),
    ActivateEquipmentCapsuleAction(ActivateEquipmentCapsuleAction),
    UseOnSelfCapsuleAction(UseOnSelfCapsuleAction),
    DestroyCliffsCapsuleAction(DestroyCliffsCapsuleAction),
    ArtilleryRemoteCapsuleAction(ArtilleryRemoteCapsuleAction),
}
