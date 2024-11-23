#[derive(Debug, Clone)]
pub enum Prototypes {
    Achievement(Achievement),
    ActiveTrigger(ActiveTrigger),
    AirbornePollutant(AirbornePollutant),
    AmbientSound(AmbientSound),
    AmmoCategory(AmmoCategory),
    Animation(Animation),
    AsteroidChunk(AsteroidChunk),
    AutoplaceControl(AutoplaceControl),
    BurnerUsage(BurnerUsage),
    CollisionLayer(CollisionLayer),
    CustomEvent(CustomEvent),
    CustomInput(CustomInput),
    DamageType(DamageType),
    Decorative(Decorative),
    DeliverCategory(DeliverCategory),
    DeliverImpactCombination(DeliverImpactCombination),
    EditorController(EditorController),
    Entity(Entity),
    Equipment(Equipment),
    EquipmentCategory(EquipmentCategory),
    EquipmentGrid(EquipmentGrid),
    Fluid(Fluid),
    Font(Font),
    FuelCategory(FuelCategory),
    GodController(GodController),
    GuiStyle(GuiStyle),
    ImpactCategory(ImpactCategory),
    Item(Item),
    ItemGroup(ItemGroup),
    ItemSubgroup(ItemSubgroup),
    MapGenPresets(MapGenPresets),
    MapSettings(MapSettings),
    ModuleCategory(ModuleCategory),
    MouseCursor(MouseCursor),
    NoiseExpression(NoiseExpression),
    NoiseFunction(NoiseFunction),
    Particle(Particle),
    Procession(Procession),
    ProcessionLayerInheritanceGroup(ProcessionLayerInheritanceGroup),
    Quality(Quality),
    Recipe(Recipe),
    RecipeCategory(RecipeCategory),
    RemoteController(RemoteController),
    ResourceCategory(ResourceCategory),
    Shortcut(Shortcut),
    Sound(Sound),
    SpaceConnection(SpaceConnection),
    SpaceLocation(SpaceLocation),
    SpectatorController(SpectatorController),
    Sprite(Sprite),
    Surface(Surface),
    SurfaceProperty(SurfaceProperty),
    Technology(Technology),
    Tile(Tile),
    TileEffect(TileEffect),
    TipsAndTricksItem(TipsAndTricksItem),
    TipsAndTricksItemCategory(TipsAndTricksItemCategory),
    TriggerTargetType(TriggerTargetType),
    TrivialSmoke(TrivialSmoke),
    Tutorial(Tutorial),
    UtilityConstants(UtilityConstants),
    UtilitySounds(UtilitySounds),
    UtilitySprites(UtilitySprites),
    VirtualSignal(VirtualSignal),
}
#[derive(Debug, Clone)]
pub enum Achievement {
    Achievement,
    BuildEntityAchievement,
    ChangeSurfaceAchievement,
    CombatRobotCountAchievement,
    CompleteObjectiveAchievement,
    ConstructWithRobotsAchievement,
    CreatePlatformAchievement,
    DeconstructWithRobotsAchievement,
    DeliverByRobotsAchievement,
    DepleteResourceAchievement,
    DestroyCliffAchievement,
    DontBuildEntityAchievement,
    DontCraftManuallyAchievement,
    DontKillManuallyAchievement,
    DontResearchBeforeResearchingAchievement,
    DontUseEntityInEnergyProductionAchievement,
    EquipArmorAchievement,
    GroupAttackAchievement,
    KillAchievement,
    ModuleTransferAchievement,
    PlaceEquipmentAchievement,
    PlayerDamagedAchievement,
    ProduceAchievement,
    ProducePerHourAchievement,
    ResearchAchievement,
    ResearchWithSciencePackAchievement,
    ShootAchievement,
    SpaceConnectionDistanceTraveledAchievement,
    TrainPathAchievement,
    UseEntityInEnergyProductionAchievement,
    UseItemAchievement,
}
#[derive(Debug, Clone)]
pub enum ActiveTrigger {
    ChainActiveTrigger,
    DelayedActiveTrigger,
}
#[derive(Debug, Clone)]
pub enum AirbornePollutant {
    AirbornePollutant,
}
#[derive(Debug, Clone)]
pub enum AmbientSound {
    AmbientSound,
}
#[derive(Debug, Clone)]
pub enum AmmoCategory {
    AmmoCategory,
}
#[derive(Debug, Clone)]
pub enum Animation {
    Animation,
}
#[derive(Debug, Clone)]
pub enum AsteroidChunk {
    AsteroidChunk,
}
#[derive(Debug, Clone)]
pub enum AutoplaceControl {
    AutoplaceControl,
}
#[derive(Debug, Clone)]
pub enum BurnerUsage {
    BurnerUsage,
}
#[derive(Debug, Clone)]
pub enum CollisionLayer {
    CollisionLayer,
}
#[derive(Debug, Clone)]
pub enum CustomEvent {
    CustomEvent,
}
#[derive(Debug, Clone)]
pub enum CustomInput {
    CustomInput,
}
#[derive(Debug, Clone)]
pub enum DamageType {
    DamageType,
}
#[derive(Debug, Clone)]
pub enum Decorative {
    OptimizedDecorative,
}
#[derive(Debug, Clone)]
pub enum DeliverCategory {
    DeliverCategory,
}
#[derive(Debug, Clone)]
pub enum DeliverImpactCombination {
    DeliverImpactCombination,
}
#[derive(Debug, Clone)]
pub enum EditorController {
    EditorController,
}
#[derive(Debug, Clone)]
pub enum Entity {
    Accumulator,
    AgriculturalTower,
    AmmoTurret,
    ArithmeticCombinator,
    Arrow,
    ArtilleryFlare,
    ArtilleryProjectile,
    ArtilleryTurret,
    ArtilleryWagon,
    AssemblingMachine,
    Asteroid,
    AsteroidCollector,
    Beacon,
    Beam,
    Boiler,
    BurnerGenerator,
    CaptureRobot,
    Car,
    CargoBay,
    CargoLandingPad,
    CargoPod,
    CargoWagon,
    Character,
    CharacterCorpse,
    Cliff,
    CombatRobot,
    ConstantCombinator,
    ConstructionRobot,
    Container,
    Corpse,
    CurvedRailA,
    CurvedRailB,
    DeciderCombinator,
    DeconstructibleTileProxy,
    DisplayPanel,
    ElectricEnergyInterface,
    ElectricPole,
    ElectricTurret,
    ElevatedCurvedRailA,
    ElevatedCurvedRailB,
    ElevatedHalfDiagonalRail,
    ElevatedStraightRail,
    EntityGhost,
    Explosion,
    Fire,
    Fish,
    FluidTurret,
    FluidWagon,
    Furnace,
    FusionGenerator,
    FusionReactor,
    Gate,
    Generator,
    HalfDiagonalRail,
    HeatInterface,
    HeatPipe,
    HighlightBox,
    InfinityContainer,
    InfinityPipe,
    Inserter,
    ItemEntity,
    ItemRequestProxy,
    Lab,
    Lamp,
    LandMine,
    LaneSplitter,
    LegacyCurvedRail,
    LegacyStraightRail,
    Lightning,
    LightningAttractor,
    LinkedBelt,
    LinkedContainer,
    Loader,
    Loader1x1,
    Locomotive,
    LogisticContainer,
    LogisticRobot,
    Market,
    MiningDrill,
    OffshorePump,
    ParticleSource,
    Pipe,
    PipeToGround,
    Plant,
    PlayerPort,
    PowerSwitch,
    ProgrammableSpeaker,
    Projectile,
    Pump,
    Radar,
    RailChainSignal,
    RailRamp,
    RailRemnants,
    RailSignal,
    RailSupport,
    Reactor,
    Resource,
    Roboport,
    RocketSilo,
    RocketSiloRocket,
    RocketSiloRocketShadow,
    Segment,
    SegmentedUnit,
    SelectorCombinator,
    SimpleEntity,
    SimpleEntityWithForce,
    SimpleEntityWithOwner,
    SmokeWithTrigger,
    SolarPanel,
    SpacePlatformHub,
    SpeechBubble,
    SpiderLeg,
    SpiderUnit,
    SpiderVehicle,
    Splitter,
    Sticker,
    StorageTank,
    StraightRail,
    Stream,
    TemporaryContainer,
    Thruster,
    TileGhost,
    TrainStop,
    TransportBelt,
    Tree,
    Turret,
    UndergroundBelt,
    Unit,
    UnitSpawner,
    Wall,
}
#[derive(Debug, Clone)]
pub enum Equipment {
    ActiveDefenseEquipment,
    BatteryEquipment,
    BeltImmunityEquipment,
    EnergyShieldEquipment,
    EquipmentGhost,
    GeneratorEquipment,
    InventoryBonusEquipment,
    MovementBonusEquipment,
    NightVisionEquipment,
    RoboportEquipment,
    SolarPanelEquipment,
}
#[derive(Debug, Clone)]
pub enum EquipmentCategory {
    EquipmentCategory,
}
#[derive(Debug, Clone)]
pub enum EquipmentGrid {
    EquipmentGrid,
}
#[derive(Debug, Clone)]
pub enum Fluid {
    Fluid,
}
#[derive(Debug, Clone)]
pub enum Font {
    Font,
}
#[derive(Debug, Clone)]
pub enum FuelCategory {
    FuelCategory,
}
#[derive(Debug, Clone)]
pub enum GodController {
    GodController,
}
#[derive(Debug, Clone)]
pub enum GuiStyle {
    GuiStyle,
}
#[derive(Debug, Clone)]
pub enum ImpactCategory {
    ImpactCategory,
}
#[derive(Debug, Clone)]
pub enum Item {
    Ammo,
    Armor,
    Blueprint,
    BlueprintBook,
    Capsule,
    CopyPasteTool,
    DeconstructionItem,
    Gun,
    Item,
    ItemWithEntityData,
    ItemWithInventory,
    ItemWithLabel,
    ItemWithTags,
    Module,
    RailPlanner,
    RepairTool,
    SelectionTool,
    SpacePlatformStarterPack,
    SpidertronRemote,
    Tool,
    UpgradeItem,
}
#[derive(Debug, Clone)]
pub enum ItemGroup {
    ItemGroup,
}
#[derive(Debug, Clone)]
pub enum ItemSubgroup {
    ItemSubgroup,
}
#[derive(Debug, Clone)]
pub enum MapGenPresets {
    MapGenPresets,
}
#[derive(Debug, Clone)]
pub enum MapSettings {
    MapSettings,
}
#[derive(Debug, Clone)]
pub enum ModuleCategory {
    ModuleCategory,
}
#[derive(Debug, Clone)]
pub enum MouseCursor {
    MouseCursor,
}
#[derive(Debug, Clone)]
pub enum NoiseExpression {
    NoiseExpression,
}
#[derive(Debug, Clone)]
pub enum NoiseFunction {
    NoiseFunction,
}
#[derive(Debug, Clone)]
pub enum Particle {
    OptimizedParticle,
}
#[derive(Debug, Clone)]
pub enum Procession {
    Procession,
}
#[derive(Debug, Clone)]
pub enum ProcessionLayerInheritanceGroup {
    ProcessionLayerInheritanceGroup,
}
#[derive(Debug, Clone)]
pub enum Quality {
    Quality,
}
#[derive(Debug, Clone)]
pub enum Recipe {
    Recipe,
}
#[derive(Debug, Clone)]
pub enum RecipeCategory {
    RecipeCategory,
}
#[derive(Debug, Clone)]
pub enum RemoteController {
    RemoteController,
}
#[derive(Debug, Clone)]
pub enum ResourceCategory {
    ResourceCategory,
}
#[derive(Debug, Clone)]
pub enum Shortcut {
    Shortcut,
}
#[derive(Debug, Clone)]
pub enum Sound {
    Sound,
}
#[derive(Debug, Clone)]
pub enum SpaceConnection {
    SpaceConnection,
}
#[derive(Debug, Clone)]
pub enum SpaceLocation {
    Planet,
    SpaceLocation,
}
#[derive(Debug, Clone)]
pub enum SpectatorController {
    SpectatorController,
}
#[derive(Debug, Clone)]
pub enum Sprite {
    Sprite,
}
#[derive(Debug, Clone)]
pub enum Surface {
    Surface,
}
#[derive(Debug, Clone)]
pub enum SurfaceProperty {
    SurfaceProperty,
}
#[derive(Debug, Clone)]
pub enum Technology {
    Technology,
}
#[derive(Debug, Clone)]
pub enum Tile {
    Tile,
}
#[derive(Debug, Clone)]
pub enum TileEffect {
    TileEffect,
}
#[derive(Debug, Clone)]
pub enum TipsAndTricksItem {
    TipsAndTricksItem,
}
#[derive(Debug, Clone)]
pub enum TipsAndTricksItemCategory {
    TipsAndTricksItemCategory,
}
#[derive(Debug, Clone)]
pub enum TriggerTargetType {
    TriggerTargetType,
}
#[derive(Debug, Clone)]
pub enum TrivialSmoke {
    TrivialSmoke,
}
#[derive(Debug, Clone)]
pub enum Tutorial {
    Tutorial,
}
#[derive(Debug, Clone)]
pub enum UtilityConstants {
    UtilityConstants,
}
#[derive(Debug, Clone)]
pub enum UtilitySounds {
    UtilitySounds,
}
#[derive(Debug, Clone)]
pub enum UtilitySprites {
    UtilitySprites,
}
#[derive(Debug, Clone)]
pub enum VirtualSignal {
    VirtualSignal,
}
