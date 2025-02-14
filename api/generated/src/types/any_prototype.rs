#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum AnyPrototype {
    #[serde(rename = "accumulator")]
    AccumulatorPrototype(Box<crate::prototypes::AccumulatorPrototype>),
    #[serde(rename = "achievement")]
    AchievementPrototype(Box<crate::prototypes::AchievementPrototype>),
    #[serde(rename = "active-defense-equipment")]
    ActiveDefenseEquipmentPrototype(Box<crate::prototypes::ActiveDefenseEquipmentPrototype>),
    #[serde(rename = "agricultural-tower")]
    AgriculturalTowerPrototype(Box<crate::prototypes::AgriculturalTowerPrototype>),
    #[serde(rename = "airborne-pollutant")]
    AirbornePollutantPrototype(Box<crate::prototypes::AirbornePollutantPrototype>),
    #[serde(rename = "ambient-sound")]
    AmbientSound(Box<crate::prototypes::AmbientSound>),
    #[serde(rename = "ammo-category")]
    AmmoCategory(Box<crate::prototypes::AmmoCategory>),
    #[serde(rename = "ammo")]
    AmmoItemPrototype(Box<crate::prototypes::AmmoItemPrototype>),
    #[serde(rename = "ammo-turret")]
    AmmoTurretPrototype(Box<crate::prototypes::AmmoTurretPrototype>),
    #[serde(rename = "animation")]
    AnimationPrototype(Box<crate::prototypes::AnimationPrototype>),
    #[serde(rename = "arithmetic-combinator")]
    ArithmeticCombinatorPrototype(Box<crate::prototypes::ArithmeticCombinatorPrototype>),
    #[serde(rename = "armor")]
    ArmorPrototype(Box<crate::prototypes::ArmorPrototype>),
    #[serde(rename = "arrow")]
    ArrowPrototype(Box<crate::prototypes::ArrowPrototype>),
    #[serde(rename = "artillery-flare")]
    ArtilleryFlarePrototype(Box<crate::prototypes::ArtilleryFlarePrototype>),
    #[serde(rename = "artillery-projectile")]
    ArtilleryProjectilePrototype(Box<crate::prototypes::ArtilleryProjectilePrototype>),
    #[serde(rename = "artillery-turret")]
    ArtilleryTurretPrototype(Box<crate::prototypes::ArtilleryTurretPrototype>),
    #[serde(rename = "artillery-wagon")]
    ArtilleryWagonPrototype(Box<crate::prototypes::ArtilleryWagonPrototype>),
    #[serde(rename = "assembling-machine")]
    AssemblingMachinePrototype(Box<crate::prototypes::AssemblingMachinePrototype>),
    #[serde(rename = "asteroid-chunk")]
    AsteroidChunkPrototype(Box<crate::prototypes::AsteroidChunkPrototype>),
    #[serde(rename = "asteroid-collector")]
    AsteroidCollectorPrototype(Box<crate::prototypes::AsteroidCollectorPrototype>),
    #[serde(rename = "asteroid")]
    AsteroidPrototype(Box<crate::prototypes::AsteroidPrototype>),
    #[serde(rename = "autoplace-control")]
    AutoplaceControl(Box<crate::prototypes::AutoplaceControl>),
    #[serde(rename = "battery-equipment")]
    BatteryEquipmentPrototype(Box<crate::prototypes::BatteryEquipmentPrototype>),
    #[serde(rename = "beacon")]
    BeaconPrototype(Box<crate::prototypes::BeaconPrototype>),
    #[serde(rename = "beam")]
    BeamPrototype(Box<crate::prototypes::BeamPrototype>),
    #[serde(rename = "belt-immunity-equipment")]
    BeltImmunityEquipmentPrototype(Box<crate::prototypes::BeltImmunityEquipmentPrototype>),
    #[serde(rename = "blueprint-book")]
    BlueprintBookPrototype(Box<crate::prototypes::BlueprintBookPrototype>),
    #[serde(rename = "blueprint")]
    BlueprintItemPrototype(Box<crate::prototypes::BlueprintItemPrototype>),
    #[serde(rename = "boiler")]
    BoilerPrototype(Box<crate::prototypes::BoilerPrototype>),
    #[serde(rename = "build-entity-achievement")]
    BuildEntityAchievementPrototype(Box<crate::prototypes::BuildEntityAchievementPrototype>),
    #[serde(rename = "burner-generator")]
    BurnerGeneratorPrototype(Box<crate::prototypes::BurnerGeneratorPrototype>),
    #[serde(rename = "burner-usage")]
    BurnerUsagePrototype(Box<crate::prototypes::BurnerUsagePrototype>),
    #[serde(rename = "capsule")]
    CapsulePrototype(Box<crate::prototypes::CapsulePrototype>),
    #[serde(rename = "capture-robot")]
    CaptureRobotPrototype(Box<crate::prototypes::CaptureRobotPrototype>),
    #[serde(rename = "car")]
    CarPrototype(Box<crate::prototypes::CarPrototype>),
    #[serde(rename = "cargo-bay")]
    CargoBayPrototype(Box<crate::prototypes::CargoBayPrototype>),
    #[serde(rename = "cargo-landing-pad")]
    CargoLandingPadPrototype(Box<crate::prototypes::CargoLandingPadPrototype>),
    #[serde(rename = "cargo-pod")]
    CargoPodPrototype(Box<crate::prototypes::CargoPodPrototype>),
    #[serde(rename = "cargo-wagon")]
    CargoWagonPrototype(Box<crate::prototypes::CargoWagonPrototype>),
    #[serde(rename = "chain-active-trigger")]
    ChainActiveTriggerPrototype(Box<crate::prototypes::ChainActiveTriggerPrototype>),
    #[serde(rename = "change-surface-achievement")]
    ChangedSurfaceAchievementPrototype(Box<crate::prototypes::ChangedSurfaceAchievementPrototype>),
    #[serde(rename = "character-corpse")]
    CharacterCorpsePrototype(Box<crate::prototypes::CharacterCorpsePrototype>),
    #[serde(rename = "character")]
    CharacterPrototype(Box<crate::prototypes::CharacterPrototype>),
    #[serde(rename = "cliff")]
    CliffPrototype(Box<crate::prototypes::CliffPrototype>),
    #[serde(rename = "collision-layer")]
    CollisionLayerPrototype(Box<crate::prototypes::CollisionLayerPrototype>),
    #[serde(rename = "combat-robot-count-achievement")]
    CombatRobotCountAchievementPrototype(
        Box<crate::prototypes::CombatRobotCountAchievementPrototype>,
    ),
    #[serde(rename = "combat-robot")]
    CombatRobotPrototype(Box<crate::prototypes::CombatRobotPrototype>),
    #[serde(rename = "complete-objective-achievement")]
    CompleteObjectiveAchievementPrototype(
        Box<crate::prototypes::CompleteObjectiveAchievementPrototype>,
    ),
    #[serde(rename = "constant-combinator")]
    ConstantCombinatorPrototype(Box<crate::prototypes::ConstantCombinatorPrototype>),
    #[serde(rename = "construct-with-robots-achievement")]
    ConstructWithRobotsAchievementPrototype(
        Box<crate::prototypes::ConstructWithRobotsAchievementPrototype>,
    ),
    #[serde(rename = "construction-robot")]
    ConstructionRobotPrototype(Box<crate::prototypes::ConstructionRobotPrototype>),
    #[serde(rename = "container")]
    ContainerPrototype(Box<crate::prototypes::ContainerPrototype>),
    #[serde(rename = "copy-paste-tool")]
    CopyPasteToolPrototype(Box<crate::prototypes::CopyPasteToolPrototype>),
    #[serde(rename = "corpse")]
    CorpsePrototype(Box<crate::prototypes::CorpsePrototype>),
    #[serde(rename = "create-platform-achievement")]
    CreatePlatformAchievementPrototype(Box<crate::prototypes::CreatePlatformAchievementPrototype>),
    #[serde(rename = "curved-rail-a")]
    CurvedRailAPrototype(Box<crate::prototypes::CurvedRailAPrototype>),
    #[serde(rename = "curved-rail-b")]
    CurvedRailBPrototype(Box<crate::prototypes::CurvedRailBPrototype>),
    #[serde(rename = "custom-event")]
    CustomEventPrototype(Box<crate::prototypes::CustomEventPrototype>),
    #[serde(rename = "custom-input")]
    CustomInputPrototype(Box<crate::prototypes::CustomInputPrototype>),
    #[serde(rename = "damage-type")]
    DamageType(Box<crate::prototypes::DamageType>),
    #[serde(rename = "decider-combinator")]
    DeciderCombinatorPrototype(Box<crate::prototypes::DeciderCombinatorPrototype>),
    #[serde(rename = "deconstruct-with-robots-achievement")]
    DeconstructWithRobotsAchievementPrototype(
        Box<crate::prototypes::DeconstructWithRobotsAchievementPrototype>,
    ),
    #[serde(rename = "deconstructible-tile-proxy")]
    DeconstructibleTileProxyPrototype(Box<crate::prototypes::DeconstructibleTileProxyPrototype>),
    #[serde(rename = "deconstruction-item")]
    DeconstructionItemPrototype(Box<crate::prototypes::DeconstructionItemPrototype>),
    #[serde(rename = "optimized-decorative")]
    DecorativePrototype(Box<crate::prototypes::DecorativePrototype>),
    #[serde(rename = "delayed-active-trigger")]
    DelayedActiveTriggerPrototype(Box<crate::prototypes::DelayedActiveTriggerPrototype>),
    #[serde(rename = "deliver-by-robots-achievement")]
    DeliverByRobotsAchievementPrototype(
        Box<crate::prototypes::DeliverByRobotsAchievementPrototype>,
    ),
    #[serde(rename = "deliver-category")]
    DeliverCategory(Box<crate::prototypes::DeliverCategory>),
    #[serde(rename = "deliver-impact-combination")]
    DeliverImpactCombination(Box<crate::prototypes::DeliverImpactCombination>),
    #[serde(rename = "deplete-resource-achievement")]
    DepleteResourceAchievementPrototype(
        Box<crate::prototypes::DepleteResourceAchievementPrototype>,
    ),
    #[serde(rename = "destroy-cliff-achievement")]
    DestroyCliffAchievementPrototype(Box<crate::prototypes::DestroyCliffAchievementPrototype>),
    #[serde(rename = "display-panel")]
    DisplayPanelPrototype(Box<crate::prototypes::DisplayPanelPrototype>),
    #[serde(rename = "dont-build-entity-achievement")]
    DontBuildEntityAchievementPrototype(
        Box<crate::prototypes::DontBuildEntityAchievementPrototype>,
    ),
    #[serde(rename = "dont-craft-manually-achievement")]
    DontCraftManuallyAchievementPrototype(
        Box<crate::prototypes::DontCraftManuallyAchievementPrototype>,
    ),
    #[serde(rename = "dont-kill-manually-achievement")]
    DontKillManuallyAchievementPrototype(
        Box<crate::prototypes::DontKillManuallyAchievementPrototype>,
    ),
    #[serde(rename = "dont-research-before-researching-achievement")]
    DontResearchBeforeResearchingAchievementPrototype(
        Box<crate::prototypes::DontResearchBeforeResearchingAchievementPrototype>,
    ),
    #[serde(rename = "dont-use-entity-in-energy-production-achievement")]
    DontUseEntityInEnergyProductionAchievementPrototype(
        Box<crate::prototypes::DontUseEntityInEnergyProductionAchievementPrototype>,
    ),
    #[serde(rename = "editor-controller")]
    EditorControllerPrototype(Box<crate::prototypes::EditorControllerPrototype>),
    #[serde(rename = "electric-energy-interface")]
    ElectricEnergyInterfacePrototype(Box<crate::prototypes::ElectricEnergyInterfacePrototype>),
    #[serde(rename = "electric-pole")]
    ElectricPolePrototype(Box<crate::prototypes::ElectricPolePrototype>),
    #[serde(rename = "electric-turret")]
    ElectricTurretPrototype(Box<crate::prototypes::ElectricTurretPrototype>),
    #[serde(rename = "elevated-curved-rail-a")]
    ElevatedCurvedRailAPrototype(Box<crate::prototypes::ElevatedCurvedRailAPrototype>),
    #[serde(rename = "elevated-curved-rail-b")]
    ElevatedCurvedRailBPrototype(Box<crate::prototypes::ElevatedCurvedRailBPrototype>),
    #[serde(rename = "elevated-half-diagonal-rail")]
    ElevatedHalfDiagonalRailPrototype(Box<crate::prototypes::ElevatedHalfDiagonalRailPrototype>),
    #[serde(rename = "elevated-straight-rail")]
    ElevatedStraightRailPrototype(Box<crate::prototypes::ElevatedStraightRailPrototype>),
    #[serde(rename = "unit-spawner")]
    EnemySpawnerPrototype(Box<crate::prototypes::EnemySpawnerPrototype>),
    #[serde(rename = "energy-shield-equipment")]
    EnergyShieldEquipmentPrototype(Box<crate::prototypes::EnergyShieldEquipmentPrototype>),
    #[serde(rename = "entity-ghost")]
    EntityGhostPrototype(Box<crate::prototypes::EntityGhostPrototype>),
    #[serde(rename = "equip-armor-achievement")]
    EquipArmorAchievementPrototype(Box<crate::prototypes::EquipArmorAchievementPrototype>),
    #[serde(rename = "equipment-category")]
    EquipmentCategory(Box<crate::prototypes::EquipmentCategory>),
    #[serde(rename = "equipment-ghost")]
    EquipmentGhostPrototype(Box<crate::prototypes::EquipmentGhostPrototype>),
    #[serde(rename = "equipment-grid")]
    EquipmentGridPrototype(Box<crate::prototypes::EquipmentGridPrototype>),
    #[serde(rename = "explosion")]
    ExplosionPrototype(Box<crate::prototypes::ExplosionPrototype>),
    #[serde(rename = "fire")]
    FireFlamePrototype(Box<crate::prototypes::FireFlamePrototype>),
    #[serde(rename = "fish")]
    FishPrototype(Box<crate::prototypes::FishPrototype>),
    #[serde(rename = "fluid")]
    FluidPrototype(Box<crate::prototypes::FluidPrototype>),
    #[serde(rename = "stream")]
    FluidStreamPrototype(Box<crate::prototypes::FluidStreamPrototype>),
    #[serde(rename = "fluid-turret")]
    FluidTurretPrototype(Box<crate::prototypes::FluidTurretPrototype>),
    #[serde(rename = "fluid-wagon")]
    FluidWagonPrototype(Box<crate::prototypes::FluidWagonPrototype>),
    #[serde(rename = "font")]
    FontPrototype(Box<crate::prototypes::FontPrototype>),
    #[serde(rename = "fuel-category")]
    FuelCategory(Box<crate::prototypes::FuelCategory>),
    #[serde(rename = "furnace")]
    FurnacePrototype(Box<crate::prototypes::FurnacePrototype>),
    #[serde(rename = "fusion-generator")]
    FusionGeneratorPrototype(Box<crate::prototypes::FusionGeneratorPrototype>),
    #[serde(rename = "fusion-reactor")]
    FusionReactorPrototype(Box<crate::prototypes::FusionReactorPrototype>),
    #[serde(rename = "gate")]
    GatePrototype(Box<crate::prototypes::GatePrototype>),
    #[serde(rename = "generator-equipment")]
    GeneratorEquipmentPrototype(Box<crate::prototypes::GeneratorEquipmentPrototype>),
    #[serde(rename = "generator")]
    GeneratorPrototype(Box<crate::prototypes::GeneratorPrototype>),
    #[serde(rename = "god-controller")]
    GodControllerPrototype(Box<crate::prototypes::GodControllerPrototype>),
    #[serde(rename = "group-attack-achievement")]
    GroupAttackAchievementPrototype(Box<crate::prototypes::GroupAttackAchievementPrototype>),
    #[serde(rename = "gui-style")]
    GuiStyle(Box<crate::prototypes::GuiStyle>),
    #[serde(rename = "gun")]
    GunPrototype(Box<crate::prototypes::GunPrototype>),
    #[serde(rename = "half-diagonal-rail")]
    HalfDiagonalRailPrototype(Box<crate::prototypes::HalfDiagonalRailPrototype>),
    #[serde(rename = "heat-interface")]
    HeatInterfacePrototype(Box<crate::prototypes::HeatInterfacePrototype>),
    #[serde(rename = "heat-pipe")]
    HeatPipePrototype(Box<crate::prototypes::HeatPipePrototype>),
    #[serde(rename = "highlight-box")]
    HighlightBoxEntityPrototype(Box<crate::prototypes::HighlightBoxEntityPrototype>),
    #[serde(rename = "impact-category")]
    ImpactCategory(Box<crate::prototypes::ImpactCategory>),
    #[serde(rename = "infinity-container")]
    InfinityContainerPrototype(Box<crate::prototypes::InfinityContainerPrototype>),
    #[serde(rename = "infinity-pipe")]
    InfinityPipePrototype(Box<crate::prototypes::InfinityPipePrototype>),
    #[serde(rename = "inserter")]
    InserterPrototype(Box<crate::prototypes::InserterPrototype>),
    #[serde(rename = "inventory-bonus-equipment")]
    InventoryBonusEquipmentPrototype(Box<crate::prototypes::InventoryBonusEquipmentPrototype>),
    #[serde(rename = "item-entity")]
    ItemEntityPrototype(Box<crate::prototypes::ItemEntityPrototype>),
    #[serde(rename = "item-group")]
    ItemGroup(Box<crate::prototypes::ItemGroup>),
    #[serde(rename = "item")]
    ItemPrototype(Box<crate::prototypes::ItemPrototype>),
    #[serde(rename = "item-request-proxy")]
    ItemRequestProxyPrototype(Box<crate::prototypes::ItemRequestProxyPrototype>),
    #[serde(rename = "item-subgroup")]
    ItemSubGroup(Box<crate::prototypes::ItemSubGroup>),
    #[serde(rename = "item-with-entity-data")]
    ItemWithEntityDataPrototype(Box<crate::prototypes::ItemWithEntityDataPrototype>),
    #[serde(rename = "item-with-inventory")]
    ItemWithInventoryPrototype(Box<crate::prototypes::ItemWithInventoryPrototype>),
    #[serde(rename = "item-with-label")]
    ItemWithLabelPrototype(Box<crate::prototypes::ItemWithLabelPrototype>),
    #[serde(rename = "item-with-tags")]
    ItemWithTagsPrototype(Box<crate::prototypes::ItemWithTagsPrototype>),
    #[serde(rename = "kill-achievement")]
    KillAchievementPrototype(Box<crate::prototypes::KillAchievementPrototype>),
    #[serde(rename = "lab")]
    LabPrototype(Box<crate::prototypes::LabPrototype>),
    #[serde(rename = "lamp")]
    LampPrototype(Box<crate::prototypes::LampPrototype>),
    #[serde(rename = "land-mine")]
    LandMinePrototype(Box<crate::prototypes::LandMinePrototype>),
    #[serde(rename = "lane-splitter")]
    LaneSplitterPrototype(Box<crate::prototypes::LaneSplitterPrototype>),
    #[serde(rename = "legacy-curved-rail")]
    LegacyCurvedRailPrototype(Box<crate::prototypes::LegacyCurvedRailPrototype>),
    #[serde(rename = "legacy-straight-rail")]
    LegacyStraightRailPrototype(Box<crate::prototypes::LegacyStraightRailPrototype>),
    #[serde(rename = "lightning-attractor")]
    LightningAttractorPrototype(Box<crate::prototypes::LightningAttractorPrototype>),
    #[serde(rename = "lightning")]
    LightningPrototype(Box<crate::prototypes::LightningPrototype>),
    #[serde(rename = "linked-belt")]
    LinkedBeltPrototype(Box<crate::prototypes::LinkedBeltPrototype>),
    #[serde(rename = "linked-container")]
    LinkedContainerPrototype(Box<crate::prototypes::LinkedContainerPrototype>),
    #[serde(rename = "loader-1x1")]
    Loader1x1Prototype(Box<crate::prototypes::Loader1x1Prototype>),
    #[serde(rename = "loader")]
    Loader1x2Prototype(Box<crate::prototypes::Loader1x2Prototype>),
    #[serde(rename = "locomotive")]
    LocomotivePrototype(Box<crate::prototypes::LocomotivePrototype>),
    #[serde(rename = "logistic-container")]
    LogisticContainerPrototype(Box<crate::prototypes::LogisticContainerPrototype>),
    #[serde(rename = "logistic-robot")]
    LogisticRobotPrototype(Box<crate::prototypes::LogisticRobotPrototype>),
    #[serde(rename = "map-gen-presets")]
    MapGenPresets(Box<crate::prototypes::MapGenPresets>),
    #[serde(rename = "map-settings")]
    MapSettings(Box<crate::prototypes::MapSettings>),
    #[serde(rename = "market")]
    MarketPrototype(Box<crate::prototypes::MarketPrototype>),
    #[serde(rename = "mining-drill")]
    MiningDrillPrototype(Box<crate::prototypes::MiningDrillPrototype>),
    #[serde(rename = "module-category")]
    ModuleCategory(Box<crate::prototypes::ModuleCategory>),
    #[serde(rename = "module")]
    ModulePrototype(Box<crate::prototypes::ModulePrototype>),
    #[serde(rename = "module-transfer-achievement")]
    ModuleTransferAchievementPrototype(Box<crate::prototypes::ModuleTransferAchievementPrototype>),
    #[serde(rename = "mouse-cursor")]
    MouseCursor(Box<crate::prototypes::MouseCursor>),
    #[serde(rename = "movement-bonus-equipment")]
    MovementBonusEquipmentPrototype(Box<crate::prototypes::MovementBonusEquipmentPrototype>),
    #[serde(rename = "noise-expression")]
    NamedNoiseExpression(Box<crate::prototypes::NamedNoiseExpression>),
    #[serde(rename = "noise-function")]
    NamedNoiseFunction(Box<crate::prototypes::NamedNoiseFunction>),
    #[serde(rename = "night-vision-equipment")]
    NightVisionEquipmentPrototype(Box<crate::prototypes::NightVisionEquipmentPrototype>),
    #[serde(rename = "offshore-pump")]
    OffshorePumpPrototype(Box<crate::prototypes::OffshorePumpPrototype>),
    #[serde(rename = "optimized-particle")]
    ParticlePrototype(Box<crate::prototypes::ParticlePrototype>),
    #[serde(rename = "particle-source")]
    ParticleSourcePrototype(Box<crate::prototypes::ParticleSourcePrototype>),
    #[serde(rename = "pipe")]
    PipePrototype(Box<crate::prototypes::PipePrototype>),
    #[serde(rename = "pipe-to-ground")]
    PipeToGroundPrototype(Box<crate::prototypes::PipeToGroundPrototype>),
    #[serde(rename = "place-equipment-achievement")]
    PlaceEquipmentAchievementPrototype(Box<crate::prototypes::PlaceEquipmentAchievementPrototype>),
    #[serde(rename = "planet")]
    PlanetPrototype(Box<crate::prototypes::PlanetPrototype>),
    #[serde(rename = "plant")]
    PlantPrototype(Box<crate::prototypes::PlantPrototype>),
    #[serde(rename = "player-damaged-achievement")]
    PlayerDamagedAchievementPrototype(Box<crate::prototypes::PlayerDamagedAchievementPrototype>),
    #[serde(rename = "player-port")]
    PlayerPortPrototype(Box<crate::prototypes::PlayerPortPrototype>),
    #[serde(rename = "power-switch")]
    PowerSwitchPrototype(Box<crate::prototypes::PowerSwitchPrototype>),
    #[serde(rename = "procession-layer-inheritance-group")]
    ProcessionLayerInheritanceGroup(Box<crate::prototypes::ProcessionLayerInheritanceGroup>),
    #[serde(rename = "procession")]
    ProcessionPrototype(Box<crate::prototypes::ProcessionPrototype>),
    #[serde(rename = "produce-achievement")]
    ProduceAchievementPrototype(Box<crate::prototypes::ProduceAchievementPrototype>),
    #[serde(rename = "produce-per-hour-achievement")]
    ProducePerHourAchievementPrototype(Box<crate::prototypes::ProducePerHourAchievementPrototype>),
    #[serde(rename = "programmable-speaker")]
    ProgrammableSpeakerPrototype(Box<crate::prototypes::ProgrammableSpeakerPrototype>),
    #[serde(rename = "projectile")]
    ProjectilePrototype(Box<crate::prototypes::ProjectilePrototype>),
    #[serde(rename = "pump")]
    PumpPrototype(Box<crate::prototypes::PumpPrototype>),
    #[serde(rename = "quality")]
    QualityPrototype(Box<crate::prototypes::QualityPrototype>),
    #[serde(rename = "radar")]
    RadarPrototype(Box<crate::prototypes::RadarPrototype>),
    #[serde(rename = "rail-chain-signal")]
    RailChainSignalPrototype(Box<crate::prototypes::RailChainSignalPrototype>),
    #[serde(rename = "rail-planner")]
    RailPlannerPrototype(Box<crate::prototypes::RailPlannerPrototype>),
    #[serde(rename = "rail-ramp")]
    RailRampPrototype(Box<crate::prototypes::RailRampPrototype>),
    #[serde(rename = "rail-remnants")]
    RailRemnantsPrototype(Box<crate::prototypes::RailRemnantsPrototype>),
    #[serde(rename = "rail-signal")]
    RailSignalPrototype(Box<crate::prototypes::RailSignalPrototype>),
    #[serde(rename = "rail-support")]
    RailSupportPrototype(Box<crate::prototypes::RailSupportPrototype>),
    #[serde(rename = "reactor")]
    ReactorPrototype(Box<crate::prototypes::ReactorPrototype>),
    #[serde(rename = "recipe-category")]
    RecipeCategory(Box<crate::prototypes::RecipeCategory>),
    #[serde(rename = "recipe")]
    RecipePrototype(Box<crate::prototypes::RecipePrototype>),
    #[serde(rename = "remote-controller")]
    RemoteControllerPrototype(Box<crate::prototypes::RemoteControllerPrototype>),
    #[serde(rename = "repair-tool")]
    RepairToolPrototype(Box<crate::prototypes::RepairToolPrototype>),
    #[serde(rename = "research-achievement")]
    ResearchAchievementPrototype(Box<crate::prototypes::ResearchAchievementPrototype>),
    #[serde(rename = "research-with-science-pack-achievement")]
    ResearchWithSciencePackAchievementPrototype(
        Box<crate::prototypes::ResearchWithSciencePackAchievementPrototype>,
    ),
    #[serde(rename = "resource-category")]
    ResourceCategory(Box<crate::prototypes::ResourceCategory>),
    #[serde(rename = "resource")]
    ResourceEntityPrototype(Box<crate::prototypes::ResourceEntityPrototype>),
    #[serde(rename = "roboport-equipment")]
    RoboportEquipmentPrototype(Box<crate::prototypes::RoboportEquipmentPrototype>),
    #[serde(rename = "roboport")]
    RoboportPrototype(Box<crate::prototypes::RoboportPrototype>),
    #[serde(rename = "rocket-silo")]
    RocketSiloPrototype(Box<crate::prototypes::RocketSiloPrototype>),
    #[serde(rename = "rocket-silo-rocket")]
    RocketSiloRocketPrototype(Box<crate::prototypes::RocketSiloRocketPrototype>),
    #[serde(rename = "rocket-silo-rocket-shadow")]
    RocketSiloRocketShadowPrototype(Box<crate::prototypes::RocketSiloRocketShadowPrototype>),
    #[serde(rename = "segment")]
    SegmentPrototype(Box<crate::prototypes::SegmentPrototype>),
    #[serde(rename = "segmented-unit")]
    SegmentedUnitPrototype(Box<crate::prototypes::SegmentedUnitPrototype>),
    #[serde(rename = "selection-tool")]
    SelectionToolPrototype(Box<crate::prototypes::SelectionToolPrototype>),
    #[serde(rename = "selector-combinator")]
    SelectorCombinatorPrototype(Box<crate::prototypes::SelectorCombinatorPrototype>),
    #[serde(rename = "shoot-achievement")]
    ShootAchievementPrototype(Box<crate::prototypes::ShootAchievementPrototype>),
    #[serde(rename = "shortcut")]
    ShortcutPrototype(Box<crate::prototypes::ShortcutPrototype>),
    #[serde(rename = "simple-entity")]
    SimpleEntityPrototype(Box<crate::prototypes::SimpleEntityPrototype>),
    #[serde(rename = "simple-entity-with-force")]
    SimpleEntityWithForcePrototype(Box<crate::prototypes::SimpleEntityWithForcePrototype>),
    #[serde(rename = "simple-entity-with-owner")]
    SimpleEntityWithOwnerPrototype(Box<crate::prototypes::SimpleEntityWithOwnerPrototype>),
    #[serde(rename = "smoke-with-trigger")]
    SmokeWithTriggerPrototype(Box<crate::prototypes::SmokeWithTriggerPrototype>),
    #[serde(rename = "solar-panel-equipment")]
    SolarPanelEquipmentPrototype(Box<crate::prototypes::SolarPanelEquipmentPrototype>),
    #[serde(rename = "solar-panel")]
    SolarPanelPrototype(Box<crate::prototypes::SolarPanelPrototype>),
    #[serde(rename = "sound")]
    SoundPrototype(Box<crate::prototypes::SoundPrototype>),
    #[serde(rename = "space-connection-distance-traveled-achievement")]
    SpaceConnectionDistanceTraveledAchievementPrototype(
        Box<crate::prototypes::SpaceConnectionDistanceTraveledAchievementPrototype>,
    ),
    #[serde(rename = "space-connection")]
    SpaceConnectionPrototype(Box<crate::prototypes::SpaceConnectionPrototype>),
    #[serde(rename = "space-location")]
    SpaceLocationPrototype(Box<crate::prototypes::SpaceLocationPrototype>),
    #[serde(rename = "space-platform-hub")]
    SpacePlatformHubPrototype(Box<crate::prototypes::SpacePlatformHubPrototype>),
    #[serde(rename = "space-platform-starter-pack")]
    SpacePlatformStarterPackPrototype(Box<crate::prototypes::SpacePlatformStarterPackPrototype>),
    #[serde(rename = "spectator-controller")]
    SpectatorControllerPrototype(Box<crate::prototypes::SpectatorControllerPrototype>),
    #[serde(rename = "speech-bubble")]
    SpeechBubblePrototype(Box<crate::prototypes::SpeechBubblePrototype>),
    #[serde(rename = "spider-leg")]
    SpiderLegPrototype(Box<crate::prototypes::SpiderLegPrototype>),
    #[serde(rename = "spider-unit")]
    SpiderUnitPrototype(Box<crate::prototypes::SpiderUnitPrototype>),
    #[serde(rename = "spider-vehicle")]
    SpiderVehiclePrototype(Box<crate::prototypes::SpiderVehiclePrototype>),
    #[serde(rename = "spidertron-remote")]
    SpidertronRemotePrototype(Box<crate::prototypes::SpidertronRemotePrototype>),
    #[serde(rename = "splitter")]
    SplitterPrototype(Box<crate::prototypes::SplitterPrototype>),
    #[serde(rename = "sprite")]
    SpritePrototype(Box<crate::prototypes::SpritePrototype>),
    #[serde(rename = "sticker")]
    StickerPrototype(Box<crate::prototypes::StickerPrototype>),
    #[serde(rename = "storage-tank")]
    StorageTankPrototype(Box<crate::prototypes::StorageTankPrototype>),
    #[serde(rename = "straight-rail")]
    StraightRailPrototype(Box<crate::prototypes::StraightRailPrototype>),
    #[serde(rename = "surface-property")]
    SurfacePropertyPrototype(Box<crate::prototypes::SurfacePropertyPrototype>),
    #[serde(rename = "surface")]
    SurfacePrototype(Box<crate::prototypes::SurfacePrototype>),
    #[serde(rename = "technology")]
    TechnologyPrototype(Box<crate::prototypes::TechnologyPrototype>),
    #[serde(rename = "temporary-container")]
    TemporaryContainerPrototype(Box<crate::prototypes::TemporaryContainerPrototype>),
    #[serde(rename = "thruster")]
    ThrusterPrototype(Box<crate::prototypes::ThrusterPrototype>),
    #[serde(rename = "tile-effect")]
    TileEffectDefinition(Box<crate::prototypes::TileEffectDefinition>),
    #[serde(rename = "tile-ghost")]
    TileGhostPrototype(Box<crate::prototypes::TileGhostPrototype>),
    #[serde(rename = "tile")]
    TilePrototype(Box<crate::prototypes::TilePrototype>),
    #[serde(rename = "tips-and-tricks-item")]
    TipsAndTricksItem(Box<crate::prototypes::TipsAndTricksItem>),
    #[serde(rename = "tips-and-tricks-item-category")]
    TipsAndTricksItemCategory(Box<crate::prototypes::TipsAndTricksItemCategory>),
    #[serde(rename = "tool")]
    ToolPrototype(Box<crate::prototypes::ToolPrototype>),
    #[serde(rename = "train-path-achievement")]
    TrainPathAchievementPrototype(Box<crate::prototypes::TrainPathAchievementPrototype>),
    #[serde(rename = "train-stop")]
    TrainStopPrototype(Box<crate::prototypes::TrainStopPrototype>),
    #[serde(rename = "transport-belt")]
    TransportBeltPrototype(Box<crate::prototypes::TransportBeltPrototype>),
    #[serde(rename = "tree")]
    TreePrototype(Box<crate::prototypes::TreePrototype>),
    #[serde(rename = "trigger-target-type")]
    TriggerTargetType(Box<crate::prototypes::TriggerTargetType>),
    #[serde(rename = "trivial-smoke")]
    TrivialSmokePrototype(Box<crate::prototypes::TrivialSmokePrototype>),
    #[serde(rename = "turret")]
    TurretPrototype(Box<crate::prototypes::TurretPrototype>),
    #[serde(rename = "tutorial")]
    TutorialDefinition(Box<crate::prototypes::TutorialDefinition>),
    #[serde(rename = "underground-belt")]
    UndergroundBeltPrototype(Box<crate::prototypes::UndergroundBeltPrototype>),
    #[serde(rename = "unit")]
    UnitPrototype(Box<crate::prototypes::UnitPrototype>),
    #[serde(rename = "upgrade-item")]
    UpgradeItemPrototype(Box<crate::prototypes::UpgradeItemPrototype>),
    #[serde(rename = "use-entity-in-energy-production-achievement")]
    UseEntityInEnergyProductionAchievementPrototype(
        Box<crate::prototypes::UseEntityInEnergyProductionAchievementPrototype>,
    ),
    #[serde(rename = "use-item-achievement")]
    UseItemAchievementPrototype(Box<crate::prototypes::UseItemAchievementPrototype>),
    #[serde(rename = "utility-constants")]
    UtilityConstants(Box<crate::prototypes::UtilityConstants>),
    #[serde(rename = "utility-sounds")]
    UtilitySounds(Box<crate::prototypes::UtilitySounds>),
    #[serde(rename = "utility-sprites")]
    UtilitySprites(Box<crate::prototypes::UtilitySprites>),
    #[serde(rename = "virtual-signal")]
    VirtualSignalPrototype(Box<crate::prototypes::VirtualSignalPrototype>),
    #[serde(rename = "wall")]
    WallPrototype(Box<crate::prototypes::WallPrototype>),
}
