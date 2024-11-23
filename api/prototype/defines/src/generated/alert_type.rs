#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum AlertType {
    CollectorPathBlocked = 13,
    Custom = 8,
    EntityDestroyed = 0,
    EntityUnderAttack = 1,
    NoMaterialForConstruction = 3,
    NoPlatformStorage = 12,
    NoRoboportStorage = 15,
    NoStorage = 9,
    NotEnoughConstructionRobots = 2,
    NotEnoughRepairPacks = 4,
    PipelineOverextended = 16,
    PlatformTileBuildingBlocked = 5,
    TrainNoPath = 11,
    TrainOutOfFuel = 10,
    TurretFire = 7,
    TurretOutOfAmmo = 6,
    UnclaimedCargo = 14,
}
