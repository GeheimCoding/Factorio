#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum Modifier {
    #[serde(rename = "inserter-stack-size-bonus")]
    InserterStackSizeBonusModifier(Box<crate::types::InserterStackSizeBonusModifier>),
    #[serde(rename = "bulk-inserter-capacity-bonus")]
    BulkInserterCapacityBonusModifier(Box<crate::types::BulkInserterCapacityBonusModifier>),
    #[serde(rename = "laboratory-speed")]
    LaboratorySpeedModifier(Box<crate::types::LaboratorySpeedModifier>),
    #[serde(rename = "character-logistic-trash-slots")]
    CharacterLogisticTrashSlotsModifier(Box<crate::types::CharacterLogisticTrashSlotsModifier>),
    #[serde(rename = "maximum-following-robots-count")]
    MaximumFollowingRobotsCountModifier(Box<crate::types::MaximumFollowingRobotsCountModifier>),
    #[serde(rename = "worker-robot-speed")]
    WorkerRobotSpeedModifier(Box<crate::types::WorkerRobotSpeedModifier>),
    #[serde(rename = "worker-robot-storage")]
    WorkerRobotStorageModifier(Box<crate::types::WorkerRobotStorageModifier>),
    #[serde(rename = "turret-attack")]
    TurretAttackModifier(Box<crate::types::TurretAttackModifier>),
    #[serde(rename = "ammo-damage")]
    AmmoDamageModifier(Box<crate::types::AmmoDamageModifier>),
    #[serde(rename = "give-item")]
    GiveItemModifier(Box<crate::types::GiveItemModifier>),
    #[serde(rename = "gun-speed")]
    GunSpeedModifier(Box<crate::types::GunSpeedModifier>),
    #[serde(rename = "unlock-recipe")]
    UnlockRecipeModifier(Box<crate::types::UnlockRecipeModifier>),
    #[serde(rename = "character-crafting-speed")]
    CharacterCraftingSpeedModifier(Box<crate::types::CharacterCraftingSpeedModifier>),
    #[serde(rename = "character-mining-speed")]
    CharacterMiningSpeedModifier(Box<crate::types::CharacterMiningSpeedModifier>),
    #[serde(rename = "character-running-speed")]
    CharacterRunningSpeedModifier(Box<crate::types::CharacterRunningSpeedModifier>),
    #[serde(rename = "character-build-distance")]
    CharacterBuildDistanceModifier(Box<crate::types::CharacterBuildDistanceModifier>),
    #[serde(rename = "character-item-drop-distance")]
    CharacterItemDropDistanceModifier(Box<crate::types::CharacterItemDropDistanceModifier>),
    #[serde(rename = "character-reach-distance")]
    CharacterReachDistanceModifier(Box<crate::types::CharacterReachDistanceModifier>),
    #[serde(rename = "character-resource-reach-distance")]
    CharacterResourceReachDistanceModifier(
        Box<crate::types::CharacterResourceReachDistanceModifier>,
    ),
    #[serde(rename = "character-item-pickup-distance")]
    CharacterItemPickupDistanceModifier(Box<crate::types::CharacterItemPickupDistanceModifier>),
    #[serde(rename = "character-loot-pickup-distance")]
    CharacterLootPickupDistanceModifier(Box<crate::types::CharacterLootPickupDistanceModifier>),
    #[serde(rename = "character-inventory-slots-bonus")]
    CharacterInventorySlotsBonusModifier(Box<crate::types::CharacterInventorySlotsBonusModifier>),
    #[serde(rename = "deconstruction-time-to-live")]
    DeconstructionTimeToLiveModifier(Box<crate::types::DeconstructionTimeToLiveModifier>),
    #[serde(rename = "max-failed-attempts-per-tick-per-construction-queue")]
    MaxFailedAttemptsPerTickPerConstructionQueueModifier(
        Box<crate::types::MaxFailedAttemptsPerTickPerConstructionQueueModifier>,
    ),
    #[serde(rename = "max-successful-attempts-per-tick-per-construction-queue")]
    MaxSuccessfulAttemptsPerTickPerConstructionQueueModifier(
        Box<crate::types::MaxSuccessfulAttemptsPerTickPerConstructionQueueModifier>,
    ),
    #[serde(rename = "character-health-bonus")]
    CharacterHealthBonusModifier(Box<crate::types::CharacterHealthBonusModifier>),
    #[serde(rename = "mining-drill-productivity-bonus")]
    MiningDrillProductivityBonusModifier(Box<crate::types::MiningDrillProductivityBonusModifier>),
    #[serde(rename = "train-braking-force-bonus")]
    TrainBrakingForceBonusModifier(Box<crate::types::TrainBrakingForceBonusModifier>),
    #[serde(rename = "worker-robot-battery")]
    WorkerRobotBatteryModifier(Box<crate::types::WorkerRobotBatteryModifier>),
    #[serde(rename = "laboratory-productivity")]
    LaboratoryProductivityModifier(Box<crate::types::LaboratoryProductivityModifier>),
    #[serde(rename = "follower-robot-lifetime")]
    FollowerRobotLifetimeModifier(Box<crate::types::FollowerRobotLifetimeModifier>),
    #[serde(rename = "artillery-range")]
    ArtilleryRangeModifier(Box<crate::types::ArtilleryRangeModifier>),
    #[serde(rename = "nothing")]
    NothingModifier(Box<crate::types::NothingModifier>),
    #[serde(rename = "character-logistic-requests")]
    CharacterLogisticRequestsModifier(Box<crate::types::CharacterLogisticRequestsModifier>),
    #[serde(rename = "vehicle-logistics")]
    VehicleLogisticsModifier(Box<crate::types::VehicleLogisticsModifier>),
    #[serde(rename = "unlock-space-location")]
    UnlockSpaceLocationModifier(Box<crate::types::UnlockSpaceLocationModifier>),
    #[serde(rename = "unlock-quality")]
    UnlockQualityModifier(Box<crate::types::UnlockQualityModifier>),
    #[serde(rename = "unlock-space-platforms")]
    SpacePlatformsModifier(Box<crate::types::SpacePlatformsModifier>),
    #[serde(rename = "unlock-circuit-network")]
    CircuitNetworkModifier(Box<crate::types::CircuitNetworkModifier>),
    #[serde(rename = "cargo-landing-pad-count")]
    CargoLandingPadLimitModifier(Box<crate::types::CargoLandingPadLimitModifier>),
    #[serde(rename = "change-recipe-productivity")]
    ChangeRecipeProductivityModifier(Box<crate::types::ChangeRecipeProductivityModifier>),
    #[serde(rename = "cliff-deconstruction-enabled")]
    CliffDeconstructionEnabledModifier(Box<crate::types::CliffDeconstructionEnabledModifier>),
    #[serde(rename = "mining-with-fluid")]
    MiningWithFluidModifier(Box<crate::types::MiningWithFluidModifier>),
    #[serde(rename = "rail-support-on-deep-oil-ocean")]
    RailSupportOnDeepOilOceanModifier(Box<crate::types::RailSupportOnDeepOilOceanModifier>),
    #[serde(rename = "rail-planner-allow-elevated-rails")]
    RailPlannerAllowElevatedRailsModifier(Box<crate::types::RailPlannerAllowElevatedRailsModifier>),
    #[serde(rename = "beacon-distribution")]
    BeaconDistributionModifier(Box<crate::types::BeaconDistributionModifier>),
    #[serde(rename = "create-ghost-on-entity-death")]
    CreateGhostOnEntityDeathModifier(Box<crate::types::CreateGhostOnEntityDeathModifier>),
    #[serde(rename = "belt-stack-size-bonus")]
    BeltStackSizeBonusModifier(Box<crate::types::BeltStackSizeBonusModifier>),
}
