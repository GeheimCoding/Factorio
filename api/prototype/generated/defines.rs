pub enum AlertType {
    CollectorPathBlocked,
    Custom,
    EntityDestroyed,
    EntityUnderAttack,
    NoMaterialForConstruction,
    NoPlatformStorage,
    NoRoboportStorage,
    NoStorage,
    NotEnoughConstructionRobots,
    NotEnoughRepairPacks,
    PipelineOverextended,
    PlatformTileBuildingBlocked,
    TrainNoPath,
    TrainOutOfFuel,
    TurretFire,
    TurretOutOfAmmo,
    UnclaimedCargo,
}
pub enum BehaviorResult {
    Deleted,
    Fail,
    InProgress,
    Success,
}
pub enum BuildCheckType {
    BlueprintGhost,
    GhostRevive,
    Manual,
    ManualGhost,
    Script,
    ScriptGhost,
}
pub enum BuildMode {
    Forced,
    Normal,
    Superforced,
}
pub enum ChainSignalState {
    AllOpen,
    None,
    NoneOpen,
    PartiallyOpen,
}
pub enum ChunkGeneratedStatus {
    BasicTiles,
    CorrectedTiles,
    CustomTiles,
    Entities,
    Nothing,
    Tiles,
}
pub enum Command {
    Attack,
    AttackArea,
    BuildBase,
    Compound,
    Flee,
    GoToLocation,
    Group,
    Stop,
    Wander,
}
pub enum CompoundCommand {
    LogicalAnd,
    LogicalOr,
    ReturnLast,
}
pub enum ControlBehaviorType {
    Accumulator,
    AgriculturalTower,
    ArithmeticCombinator,
    ArtilleryTurret,
    AssemblingMachine,
    AsteroidCollector,
    CargoLandingPad,
    ConstantCombinator,
    Container,
    DeciderCombinator,
    DisplayPanel,
    GenericOnOff,
    Inserter,
    Lamp,
    Loader,
    LogisticContainer,
    MiningDrill,
    ProgrammableSpeaker,
    Pump,
    Radar,
    RailChainSignal,
    RailSignal,
    Reactor,
    Roboport,
    RocketSilo,
    SelectorCombinator,
    SpacePlatformHub,
    StorageTank,
    TrainStop,
    TransportBelt,
    Turret,
    Wall,
}
pub enum ControlBehaviorTransportBeltContentReadMode {
    EntireBeltHold,
    Hold,
    Pulse,
}
pub enum ControlBehaviorTransportBelt {
    ContentReadMode(ControlBehaviorTransportBeltContentReadMode),
}
pub enum ControlBehaviorRocketSiloReadMode {
    LogisticInventory,
    None,
    OrbitalRequests,
}
pub enum ControlBehaviorRocketSilo {
    ReadMode(ControlBehaviorRocketSiloReadMode),
}
pub enum ControlBehaviorRoboportReadItemsMode {
    Logistics,
    MissingRequests,
    None,
}
pub enum ControlBehaviorRoboport {
    ReadItemsMode(ControlBehaviorRoboportReadItemsMode),
}
pub enum ControlBehaviorMiningDrillResourceReadMode {
    EntirePatch,
    ThisMiner,
}
pub enum ControlBehaviorMiningDrill {
    ResourceReadMode(ControlBehaviorMiningDrillResourceReadMode),
}
pub enum ControlBehaviorLogisticContainerExclusiveMode {
    None,
    SendContents,
    SetRequests,
}
pub enum ControlBehaviorLogisticContainer {
    ExclusiveMode(ControlBehaviorLogisticContainerExclusiveMode),
}
pub enum ControlBehaviorLampColorMode {
    ColorMapping,
    Components,
    PackedRgb,
}
pub enum ControlBehaviorLamp {
    ColorMode(ControlBehaviorLampColorMode),
}
pub enum ControlBehaviorInserterHandReadMode {
    Hold,
    Pulse,
}
pub enum ControlBehaviorInserter {
    HandReadMode(ControlBehaviorInserterHandReadMode),
}
pub enum ControlBehaviorCargoLandingPadExclusiveMode {
    None,
    SendContents,
    SetRequests,
}
pub enum ControlBehaviorCargoLandingPad {
    ExclusiveMode(ControlBehaviorCargoLandingPadExclusiveMode),
}
pub enum ControlBehavior {
    CargoLandingPad(ControlBehaviorCargoLandingPad),
    Inserter(ControlBehaviorInserter),
    Lamp(ControlBehaviorLamp),
    LogisticContainer(ControlBehaviorLogisticContainer),
    MiningDrill(ControlBehaviorMiningDrill),
    Roboport(ControlBehaviorRoboport),
    RocketSilo(ControlBehaviorRocketSilo),
    TransportBelt(ControlBehaviorTransportBelt),
    Type(ControlBehaviorType),
}
pub enum Controllers {
    Character,
    Cutscene,
    Editor,
    Ghost,
    God,
    Remote,
    Spectator,
}
pub enum DeconstructionItemTileSelectionMode {
    Always,
    Never,
    Normal,
    Only,
}
pub enum DeconstructionItemTileFilterMode {
    Blacklist,
    Whitelist,
}
pub enum DeconstructionItemEntityFilterMode {
    Blacklist,
    Whitelist,
}
pub enum DeconstructionItem {
    EntityFilterMode(DeconstructionItemEntityFilterMode),
    TileFilterMode(DeconstructionItemTileFilterMode),
    TileSelectionMode(DeconstructionItemTileSelectionMode),
}
pub enum DefaultIconSize {}
pub enum Difficulty {
    Easy,
    Hard,
    Normal,
}
pub enum Direction {
    East,
    Eastnortheast,
    Eastsoutheast,
    North,
    Northeast,
    Northnortheast,
    Northnorthwest,
    Northwest,
    South,
    Southeast,
    Southsoutheast,
    Southsouthwest,
    Southwest,
    West,
    Westnorthwest,
    Westsouthwest,
}
pub enum DisconnectReason {
    Afk,
    Banned,
    CannotKeepUp,
    DesyncLimitReached,
    Dropped,
    Kicked,
    KickedAndDeleted,
    Quit,
    Reconnect,
    SwitchingServers,
    WrongInput,
}
pub enum Distraction {
    ByAnything,
    ByDamage,
    ByEnemy,
    None,
}
pub enum EntityStatus {
    Broken,
    CantDivideSegments,
    Charging,
    ClosedByCircuitNetwork,
    ComputingNavigation,
    DestinationStopFull,
    Disabled,
    DisabledByControlBehavior,
    DisabledByScript,
    Discharging,
    FluidIngredientShortage,
    Frozen,
    FullBurntResultOutput,
    FullOutput,
    FullyCharged,
    Ghost,
    ItemIngredientShortage,
    LaunchingRocket,
    LowInputFluid,
    LowPower,
    LowTemperature,
    MarkedForDeconstruction,
    MissingRequiredFluid,
    MissingSciencePacks,
    NetworksConnected,
    NetworksDisconnected,
    NoAmmo,
    NoFilter,
    NoFuel,
    NoIngredients,
    NoInputFluid,
    NoMinableResources,
    NoModulesToTransmit,
    NoPath,
    NoPower,
    NoRecipe,
    NoResearchInProgress,
    NoSpotSeedableByInputs,
    Normal,
    NotConnectedToHubOrPad,
    NotConnectedToRail,
    NotEnoughSpaceInOutput,
    NotEnoughThrust,
    NotPluggedInElectricNetwork,
    OnTheWay,
    OpenedByCircuitNetwork,
    OutOfLogisticNetwork,
    Paused,
    PipelineOverextended,
    PreparingRocketForLaunch,
    RechargingAfterPowerOutage,
    RecipeNotResearched,
    ThrustNotRequired,
    TurnedOffDuringDaytime,
    WaitingAtStop,
    WaitingForMoreItems,
    WaitingForPlantsToGrow,
    WaitingForRocketsToArrive,
    WaitingForSourceItems,
    WaitingForSpaceInDestination,
    WaitingForSpaceInPlatformHub,
    WaitingForTargetToBeBuilt,
    WaitingForTrain,
    WaitingInOrbit,
    WaitingToLaunchRocket,
    Working,
}
pub enum EntityStatusDiode {
    Green,
    Red,
    Yellow,
}
pub enum Events {
    OnAchievementGained,
    OnAiCommandCompleted,
    OnAreaCloned,
    OnBiterBaseBuilt,
    OnBrushCloned,
    OnBuildBaseArrived,
    OnBuiltEntity,
    OnCancelledDeconstruction,
    OnCancelledUpgrade,
    OnCharacterCorpseExpired,
    OnChartTagAdded,
    OnChartTagModified,
    OnChartTagRemoved,
    OnChunkCharted,
    OnChunkDeleted,
    OnChunkGenerated,
    OnCombatRobotExpired,
    OnConsoleChat,
    OnConsoleCommand,
    OnCutsceneCancelled,
    OnCutsceneFinished,
    OnCutsceneStarted,
    OnCutsceneWaypointReached,
    OnEntityCloned,
    OnEntityColorChanged,
    OnEntityDamaged,
    OnEntityDied,
    OnEntityLogisticSlotChanged,
    OnEntityRenamed,
    OnEntitySettingsPasted,
    OnEntitySpawned,
    OnEquipmentInserted,
    OnEquipmentRemoved,
    OnForceCeaseFireChanged,
    OnForceCreated,
    OnForceFriendsChanged,
    OnForceReset,
    OnForcesMerged,
    OnForcesMerging,
    OnGameCreatedFromScenario,
    OnGuiCheckedStateChanged,
    OnGuiClick,
    OnGuiClosed,
    OnGuiConfirmed,
    OnGuiElemChanged,
    OnGuiHover,
    OnGuiLeave,
    OnGuiLocationChanged,
    OnGuiOpened,
    OnGuiSelectedTabChanged,
    OnGuiSelectionStateChanged,
    OnGuiSwitchStateChanged,
    OnGuiTextChanged,
    OnGuiValueChanged,
    OnLandMineArmed,
    OnLuaShortcut,
    OnMarkedForDeconstruction,
    OnMarkedForUpgrade,
    OnMarketItemPurchased,
    OnModItemOpened,
    OnObjectDestroyed,
    OnPermissionGroupAdded,
    OnPermissionGroupDeleted,
    OnPermissionGroupEdited,
    OnPermissionStringImported,
    OnPickedUpItem,
    OnPlayerAltReverseSelectedArea,
    OnPlayerAltSelectedArea,
    OnPlayerAmmoInventoryChanged,
    OnPlayerArmorInventoryChanged,
    OnPlayerBanned,
    OnPlayerBuiltTile,
    OnPlayerCancelledCrafting,
    OnPlayerChangedForce,
    OnPlayerChangedPosition,
    OnPlayerChangedSurface,
    OnPlayerCheatModeDisabled,
    OnPlayerCheatModeEnabled,
    OnPlayerClickedGpsTag,
    OnPlayerConfiguredBlueprint,
    OnPlayerControllerChanged,
    OnPlayerCraftedItem,
    OnPlayerCreated,
    OnPlayerCursorStackChanged,
    OnPlayerDeconstructedArea,
    OnPlayerDemoted,
    OnPlayerDied,
    OnPlayerDisplayDensityScaleChanged,
    OnPlayerDisplayResolutionChanged,
    OnPlayerDisplayScaleChanged,
    OnPlayerDrivingChangedState,
    OnPlayerDroppedItem,
    OnPlayerFastTransferred,
    OnPlayerFlippedEntity,
    OnPlayerFlushedFluid,
    OnPlayerGunInventoryChanged,
    OnPlayerInputMethodChanged,
    OnPlayerJoinedGame,
    OnPlayerKicked,
    OnPlayerLeftGame,
    OnPlayerLocaleChanged,
    OnPlayerMainInventoryChanged,
    OnPlayerMinedEntity,
    OnPlayerMinedItem,
    OnPlayerMinedTile,
    OnPlayerMuted,
    OnPlayerPipette,
    OnPlayerPlacedEquipment,
    OnPlayerPromoted,
    OnPlayerRemoved,
    OnPlayerRemovedEquipment,
    OnPlayerRepairedEntity,
    OnPlayerRespawned,
    OnPlayerReverseSelectedArea,
    OnPlayerRotatedEntity,
    OnPlayerSelectedArea,
    OnPlayerSetQuickBarSlot,
    OnPlayerSetupBlueprint,
    OnPlayerToggledAltMode,
    OnPlayerToggledMapEditor,
    OnPlayerTrashInventoryChanged,
    OnPlayerUnbanned,
    OnPlayerUnmuted,
    OnPlayerUsedCapsule,
    OnPlayerUsedSpidertronRemote,
    OnPostEntityDied,
    OnPreBuild,
    OnPreChunkDeleted,
    OnPreEntitySettingsPasted,
    OnPreGhostDeconstructed,
    OnPreGhostUpgraded,
    OnPrePermissionGroupDeleted,
    OnPrePermissionStringImported,
    OnPrePlayerCraftedItem,
    OnPrePlayerDied,
    OnPrePlayerLeftGame,
    OnPrePlayerMinedItem,
    OnPrePlayerRemoved,
    OnPrePlayerToggledMapEditor,
    OnPreRobotExplodedCliff,
    OnPreScenarioFinished,
    OnPreScriptInventoryResized,
    OnPreSurfaceCleared,
    OnPreSurfaceDeleted,
    OnRedoApplied,
    OnResearchCancelled,
    OnResearchFinished,
    OnResearchMoved,
    OnResearchReversed,
    OnResearchStarted,
    OnResourceDepleted,
    OnRobotBuiltEntity,
    OnRobotBuiltTile,
    OnRobotExplodedCliff,
    OnRobotMined,
    OnRobotMinedEntity,
    OnRobotMinedTile,
    OnRobotPreMined,
    OnRocketLaunchOrdered,
    OnRocketLaunched,
    OnRuntimeModSettingChanged,
    OnScriptInventoryResized,
    OnScriptPathRequestFinished,
    OnScriptTriggerEffect,
    OnSectorScanned,
    OnSegmentEntityCreated,
    OnSelectedEntityChanged,
    OnSpacePlatformBuiltEntity,
    OnSpacePlatformBuiltTile,
    OnSpacePlatformChangedState,
    OnSpacePlatformMinedEntity,
    OnSpacePlatformMinedItem,
    OnSpacePlatformMinedTile,
    OnSpacePlatformPreMined,
    OnSpiderCommandCompleted,
    OnStringTranslated,
    OnSurfaceCleared,
    OnSurfaceCreated,
    OnSurfaceDeleted,
    OnSurfaceImported,
    OnSurfaceRenamed,
    OnTechnologyEffectsReset,
    OnTick,
    OnTrainChangedState,
    OnTrainCreated,
    OnTrainScheduleChanged,
    OnTriggerCreatedEntity,
    OnTriggerFiredArtillery,
    OnUndoApplied,
    OnUnitAddedToGroup,
    OnUnitGroupCreated,
    OnUnitGroupFinishedGathering,
    OnUnitRemovedFromGroup,
    OnWorkerRobotExpired,
    ScriptRaisedBuilt,
    ScriptRaisedDestroy,
    ScriptRaisedRevive,
    ScriptRaisedSetTiles,
    ScriptRaisedTeleported,
}
pub enum FlowPrecisionIndex {
    FiftyHours,
    FiveSeconds,
    OneHour,
    OneMinute,
    OneThousandHours,
    TenHours,
    TenMinutes,
    TwoHundredFiftyHours,
}
pub enum GameControllerInteraction {
    Always,
    Never,
    Normal,
}
pub enum GroupState {
    AttackingDistraction,
    AttackingTarget,
    Finished,
    Gathering,
    Moving,
    Pathfinding,
    WanderInGroup,
}
pub enum GuiType {
    Achievement,
    BlueprintLibrary,
    Bonus,
    Controller,
    Custom,
    Entity,
    Equipment,
    GlobalElectricNetwork,
    Item,
    Logistic,
    None,
    OpenedEntityGrid,
    OtherPlayer,
    Permissions,
    PlayerManagement,
    Production,
    ScriptInventory,
    ServerManagement,
    Tile,
    Trains,
}
pub enum InputAction {
    ActivateInterrupt,
    ActivatePaste,
    AddDeciderCombinatorCondition,
    AddDeciderCombinatorOutput,
    AddLogisticSection,
    AddPermissionGroup,
    AddPin,
    AddTrainInterrupt,
    AddTrainStation,
    AdjustBlueprintSnapping,
    AdminAction,
    AltReverseSelectArea,
    AltSelectArea,
    AltSelectBlueprintEntities,
    AlternativeCopy,
    BeginMining,
    BeginMiningTerrain,
    Build,
    BuildRail,
    BuildTerrain,
    CancelCraft,
    CancelDeconstruct,
    CancelDeleteSpacePlatform,
    CancelNewBlueprint,
    CancelResearch,
    CancelUpgrade,
    ChangeActiveCharacterTab,
    ChangeActiveItemGroupForCrafting,
    ChangeActiveItemGroupForFilters,
    ChangeActiveQuickBar,
    ChangeArithmeticCombinatorParameters,
    ChangeEntityLabel,
    ChangeItemLabel,
    ChangeLogisticPointGroup,
    ChangeMultiplayerConfig,
    ChangePickingState,
    ChangeProgrammableSpeakerAlertParameters,
    ChangeProgrammableSpeakerCircuitParameters,
    ChangeProgrammableSpeakerParameters,
    ChangeRidingState,
    ChangeSelectorCombinatorParameters,
    ChangeShootingState,
    ChangeTrainName,
    ChangeTrainStopStation,
    ChangeTrainWaitCondition,
    ChangeTrainWaitConditionData,
    ClearCursor,
    ConnectRollingStock,
    Copy,
    CopyEntitySettings,
    CopyLargeOpenedBlueprint,
    CopyLargeOpenedItem,
    CopyOpenedBlueprint,
    CopyOpenedItem,
    Craft,
    CreateSpacePlatform,
    CursorSplit,
    CursorTransfer,
    CustomInput,
    CycleBlueprintBookBackwards,
    CycleBlueprintBookForwards,
    CycleQualityDown,
    CycleQualityUp,
    Deconstruct,
    DeleteBlueprintLibrary,
    DeleteBlueprintRecord,
    DeleteCustomTag,
    DeleteLogisticGroup,
    DeletePermissionGroup,
    DeleteSpacePlatform,
    DestroyItem,
    DestroyOpenedItem,
    DisconnectRollingStock,
    DragDeciderCombinatorCondition,
    DragDeciderCombinatorOutput,
    DragTrainSchedule,
    DragTrainScheduleInterrupt,
    DragTrainWaitCondition,
    DropBlueprintRecord,
    DropItem,
    EditBlueprintToolPreview,
    EditCustomTag,
    EditDisplayPanel,
    EditDisplayPanelAlwaysShow,
    EditDisplayPanelIcon,
    EditDisplayPanelParameters,
    EditDisplayPanelShowInChart,
    EditInterrupt,
    EditPermissionGroup,
    EditPin,
    EnableTransitionalRequests,
    ExportBlueprint,
    FastEntitySplit,
    FastEntityTransfer,
    FlipEntity,
    FlushOpenedEntityFluid,
    FlushOpenedEntitySpecificFluid,
    GoToTrainStation,
    GrabBlueprintRecord,
    GuiCheckedStateChanged,
    GuiClick,
    GuiConfirmed,
    GuiElemChanged,
    GuiHover,
    GuiLeave,
    GuiLocationChanged,
    GuiSelectedTabChanged,
    GuiSelectionStateChanged,
    GuiSwitchStateChanged,
    GuiTextChanged,
    GuiValueChanged,
    ImportBlueprint,
    ImportBlueprintString,
    ImportBlueprintsFiltered,
    ImportPermissionsString,
    InstantlyCreateSpacePlatform,
    InventorySplit,
    InventoryTransfer,
    LandAtPlanet,
    LaunchRocket,
    LuaShortcut,
    MapEditorAction,
    MarketOffer,
    ModSettingsChanged,
    ModifyDeciderCombinatorCondition,
    ModifyDeciderCombinatorOutput,
    MoveResearch,
    OpenAchievementsGui,
    OpenBlueprintLibraryGui,
    OpenBlueprintRecord,
    OpenBonusGui,
    OpenCharacterGui,
    OpenCurrentVehicleGui,
    OpenEquipment,
    OpenGlobalElectricNetworkGui,
    OpenGui,
    OpenItem,
    OpenLogisticsGui,
    OpenModItem,
    OpenNewPlatformButtonFromRocketSilo,
    OpenOpenedEntityGrid,
    OpenParentOfOpenedItem,
    OpenProductionGui,
    OpenTrainGui,
    OpenTrainStationGui,
    OpenTrainsGui,
    ParametriseBlueprint,
    PasteEntitySettings,
    PinAlertGroup,
    PinCustomAlert,
    PinSearchResult,
    Pipette,
    PlaceEquipment,
    QuickBarPickSlot,
    QuickBarSetSelectedPage,
    QuickBarSetSlot,
    ReassignBlueprint,
    Redo,
    RemoteViewEntity,
    RemoteViewSurface,
    RemoveCables,
    RemoveDeciderCombinatorCondition,
    RemoveDeciderCombinatorOutput,
    RemoveLogisticSection,
    RemovePin,
    RemoveTrainInterrupt,
    RemoveTrainStation,
    RenameInterrupt,
    RenameSpacePlatform,
    ReorderLogisticSection,
    RequestMissingConstructionMaterials,
    ResetAssemblingMachine,
    ReverseSelectArea,
    RotateEntity,
    SelectArea,
    SelectAsteroidChunkSlot,
    SelectBlueprintEntities,
    SelectEntityFilterSlot,
    SelectEntitySlot,
    SelectItemFilter,
    SelectMapperSlotFrom,
    SelectMapperSlotTo,
    SelectNextValidGun,
    SelectTileSlot,
    SendSpidertron,
    SendStackToTrash,
    SendStacksToTrash,
    SendTrainToPinTarget,
    SetBehaviorMode,
    SetCarWeaponsControl,
    SetCheatModeQuality,
    SetCircuitCondition,
    SetCircuitModeOfOperation,
    SetCombinatorDescription,
    SetCopyColorFromTrainStop,
    SetDeconstructionItemTileSelectionMode,
    SetDeconstructionItemTreesAndRocksOnly,
    SetEntityColor,
    SetEntityEnergyProperty,
    SetFilter,
    SetGhostCursor,
    SetHeatInterfaceMode,
    SetHeatInterfaceTemperature,
    SetInfinityContainerFilterItem,
    SetInfinityContainerRemoveUnfilteredItems,
    SetInfinityPipeFilter,
    SetInserterMaxStackSize,
    SetInventoryBar,
    SetLampAlwaysOn,
    SetLinkedContainerLinkID,
    SetLogisticFilterItem,
    SetLogisticNetworkName,
    SetLogisticSectionActive,
    SetPlayerColor,
    SetPumpFluidFilter,
    SetRequestFromBuffers,
    SetResearchFinishedStopsGame,
    SetRocketSiloSendToOrbitAutomatedMode,
    SetScheduleRecordAllowUnloading,
    SetSignal,
    SetSplitterPriority,
    SetSpoilPriority,
    SetTrainStopPriority,
    SetTrainStopped,
    SetTrainsLimit,
    SetTurretIgnoreUnlisted,
    SetUseInserterFilters,
    SetVehicleAutomaticTargetingParameters,
    SetupAssemblingMachine,
    SetupBlueprint,
    SetupSingleBlueprintRecord,
    SpawnItem,
    SpectatorChangeSurface,
    StackSplit,
    StackTransfer,
    StartRepair,
    StartResearch,
    StartWalking,
    StopDragBuild,
    SwapLogisticFilterItems,
    SwitchConnectToLogisticNetwork,
    SwitchConstantCombinatorState,
    SwitchInserterFilterModeState,
    SwitchMiningDrillFilterModeState,
    SwitchPowerSwitchState,
    TakeEquipment,
    ToggleArtilleryAutoTargeting,
    ToggleDeconstructionItemEntityFilterMode,
    ToggleDeconstructionItemTileFilterMode,
    ToggleDriving,
    ToggleEnableVehicleLogisticsWhileMoving,
    ToggleEntityLogisticRequests,
    ToggleEquipmentMovementBonus,
    ToggleMapEditor,
    TogglePersonalLogisticRequests,
    TogglePersonalRoboport,
    ToggleSelectedEntity,
    ToggleShowEntityInfo,
    TranslateString,
    TrashNotRequestedItems,
    Undo,
    Upgrade,
    UpgradeOpenedBlueprintByItem,
    UpgradeOpenedBlueprintByRecord,
    UseItem,
    WireDragging,
    WriteToConsole,
}
pub enum InputMethod {
    GameController,
    KeyboardAndMouse,
}
pub enum Inventory {
    ArtilleryTurretAmmo,
    ArtilleryWagonAmmo,
    AssemblingMachineDump,
    AssemblingMachineInput,
    AssemblingMachineModules,
    AssemblingMachineOutput,
    BeaconModules,
    BurntResult,
    CarAmmo,
    CarTrunk,
    CargoLandingPadMain,
    CargoLandingPadTrash,
    CargoUnit,
    CargoWagon,
    CharacterAmmo,
    CharacterArmor,
    CharacterCorpse,
    CharacterGuns,
    CharacterMain,
    CharacterTrash,
    CharacterVehicle,
    Chest,
    EditorAmmo,
    EditorArmor,
    EditorGuns,
    EditorMain,
    Fuel,
    FurnaceModules,
    FurnaceResult,
    FurnaceSource,
    GodMain,
    HubMain,
    HubTrash,
    ItemMain,
    LabInput,
    LabModules,
    LogisticContainerTrash,
    MiningDrillModules,
    RoboportMaterial,
    RoboportRobot,
    RobotCargo,
    RobotRepair,
    RocketSiloInput,
    RocketSiloModules,
    RocketSiloOutput,
    RocketSiloRocket,
    RocketSiloTrash,
    SpiderAmmo,
    SpiderTrash,
    SpiderTrunk,
    TurretAmmo,
}
pub enum LogisticMemberIndex {
    CharacterProvider,
    CharacterRequester,
    CharacterStorage,
    GenericOnOffBehavior,
    LogisticContainer,
    SpidertronRequester,
    VehicleStorage,
}
pub enum LogisticMode {
    ActiveProvider,
    Buffer,
    None,
    PassiveProvider,
    Requester,
    Storage,
}
pub enum LogisticSectionType {
    CircuitControlled,
    Manual,
    RequestMissingMaterialsControlled,
    TransitionalRequestControlled,
}
pub enum MouseButtonType {
    Left,
    Middle,
    None,
    Right,
}
pub enum MovingState {
    Adaptive,
    Moving,
    Stale,
    Stuck,
}
pub enum PrintSkip {
    IfRedundant,
    IfVisible,
    Never,
}
pub enum PrintSound {
    Always,
    Never,
    UsePlayerSettings,
}
pub enum PrototypesVirtualSignal {
    VirtualSignal,
}
pub enum PrototypesUtilitySprites {
    UtilitySprites,
}
pub enum PrototypesUtilitySounds {
    UtilitySounds,
}
pub enum PrototypesUtilityConstants {
    UtilityConstants,
}
pub enum PrototypesTutorial {
    Tutorial,
}
pub enum PrototypesTrivialSmoke {
    TrivialSmoke,
}
pub enum PrototypesTriggerTargetType {
    TriggerTargetType,
}
pub enum PrototypesTipsAndTricksItemCategory {
    TipsAndTricksItemCategory,
}
pub enum PrototypesTipsAndTricksItem {
    TipsAndTricksItem,
}
pub enum PrototypesTileEffect {
    TileEffect,
}
pub enum PrototypesTile {
    Tile,
}
pub enum PrototypesTechnology {
    Technology,
}
pub enum PrototypesSurfaceProperty {
    SurfaceProperty,
}
pub enum PrototypesSurface {
    Surface,
}
pub enum PrototypesSprite {
    Sprite,
}
pub enum PrototypesSpectatorController {
    SpectatorController,
}
pub enum PrototypesSpaceLocation {
    Planet,
    SpaceLocation,
}
pub enum PrototypesSpaceConnection {
    SpaceConnection,
}
pub enum PrototypesSound {
    Sound,
}
pub enum PrototypesShortcut {
    Shortcut,
}
pub enum PrototypesResourceCategory {
    ResourceCategory,
}
pub enum PrototypesRemoteController {
    RemoteController,
}
pub enum PrototypesRecipeCategory {
    RecipeCategory,
}
pub enum PrototypesRecipe {
    Recipe,
}
pub enum PrototypesQuality {
    Quality,
}
pub enum PrototypesProcessionLayerInheritanceGroup {
    ProcessionLayerInheritanceGroup,
}
pub enum PrototypesProcession {
    Procession,
}
pub enum PrototypesParticle {
    OptimizedParticle,
}
pub enum PrototypesNoiseFunction {
    NoiseFunction,
}
pub enum PrototypesNoiseExpression {
    NoiseExpression,
}
pub enum PrototypesMouseCursor {
    MouseCursor,
}
pub enum PrototypesModuleCategory {
    ModuleCategory,
}
pub enum PrototypesMapSettings {
    MapSettings,
}
pub enum PrototypesMapGenPresets {
    MapGenPresets,
}
pub enum PrototypesItemSubgroup {
    ItemSubgroup,
}
pub enum PrototypesItemGroup {
    ItemGroup,
}
pub enum PrototypesItem {
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
pub enum PrototypesImpactCategory {
    ImpactCategory,
}
pub enum PrototypesGuiStyle {
    GuiStyle,
}
pub enum PrototypesGodController {
    GodController,
}
pub enum PrototypesFuelCategory {
    FuelCategory,
}
pub enum PrototypesFont {
    Font,
}
pub enum PrototypesFluid {
    Fluid,
}
pub enum PrototypesEquipmentGrid {
    EquipmentGrid,
}
pub enum PrototypesEquipmentCategory {
    EquipmentCategory,
}
pub enum PrototypesEquipment {
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
pub enum PrototypesEntity {
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
pub enum PrototypesEditorController {
    EditorController,
}
pub enum PrototypesDeliverImpactCombination {
    DeliverImpactCombination,
}
pub enum PrototypesDeliverCategory {
    DeliverCategory,
}
pub enum PrototypesDecorative {
    OptimizedDecorative,
}
pub enum PrototypesDamageType {
    DamageType,
}
pub enum PrototypesCustomInput {
    CustomInput,
}
pub enum PrototypesCustomEvent {
    CustomEvent,
}
pub enum PrototypesCollisionLayer {
    CollisionLayer,
}
pub enum PrototypesBurnerUsage {
    BurnerUsage,
}
pub enum PrototypesAutoplaceControl {
    AutoplaceControl,
}
pub enum PrototypesAsteroidChunk {
    AsteroidChunk,
}
pub enum PrototypesAnimation {
    Animation,
}
pub enum PrototypesAmmoCategory {
    AmmoCategory,
}
pub enum PrototypesAmbientSound {
    AmbientSound,
}
pub enum PrototypesAirbornePollutant {
    AirbornePollutant,
}
pub enum PrototypesActiveTrigger {
    ChainActiveTrigger,
    DelayedActiveTrigger,
}
pub enum PrototypesAchievement {
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
pub enum Prototypes {
    Achievement(PrototypesAchievement),
    ActiveTrigger(PrototypesActiveTrigger),
    AirbornePollutant(PrototypesAirbornePollutant),
    AmbientSound(PrototypesAmbientSound),
    AmmoCategory(PrototypesAmmoCategory),
    Animation(PrototypesAnimation),
    AsteroidChunk(PrototypesAsteroidChunk),
    AutoplaceControl(PrototypesAutoplaceControl),
    BurnerUsage(PrototypesBurnerUsage),
    CollisionLayer(PrototypesCollisionLayer),
    CustomEvent(PrototypesCustomEvent),
    CustomInput(PrototypesCustomInput),
    DamageType(PrototypesDamageType),
    Decorative(PrototypesDecorative),
    DeliverCategory(PrototypesDeliverCategory),
    DeliverImpactCombination(PrototypesDeliverImpactCombination),
    EditorController(PrototypesEditorController),
    Entity(PrototypesEntity),
    Equipment(PrototypesEquipment),
    EquipmentCategory(PrototypesEquipmentCategory),
    EquipmentGrid(PrototypesEquipmentGrid),
    Fluid(PrototypesFluid),
    Font(PrototypesFont),
    FuelCategory(PrototypesFuelCategory),
    GodController(PrototypesGodController),
    GuiStyle(PrototypesGuiStyle),
    ImpactCategory(PrototypesImpactCategory),
    Item(PrototypesItem),
    ItemGroup(PrototypesItemGroup),
    ItemSubgroup(PrototypesItemSubgroup),
    MapGenPresets(PrototypesMapGenPresets),
    MapSettings(PrototypesMapSettings),
    ModuleCategory(PrototypesModuleCategory),
    MouseCursor(PrototypesMouseCursor),
    NoiseExpression(PrototypesNoiseExpression),
    NoiseFunction(PrototypesNoiseFunction),
    Particle(PrototypesParticle),
    Procession(PrototypesProcession),
    ProcessionLayerInheritanceGroup(PrototypesProcessionLayerInheritanceGroup),
    Quality(PrototypesQuality),
    Recipe(PrototypesRecipe),
    RecipeCategory(PrototypesRecipeCategory),
    RemoteController(PrototypesRemoteController),
    ResourceCategory(PrototypesResourceCategory),
    Shortcut(PrototypesShortcut),
    Sound(PrototypesSound),
    SpaceConnection(PrototypesSpaceConnection),
    SpaceLocation(PrototypesSpaceLocation),
    SpectatorController(PrototypesSpectatorController),
    Sprite(PrototypesSprite),
    Surface(PrototypesSurface),
    SurfaceProperty(PrototypesSurfaceProperty),
    Technology(PrototypesTechnology),
    Tile(PrototypesTile),
    TileEffect(PrototypesTileEffect),
    TipsAndTricksItem(PrototypesTipsAndTricksItem),
    TipsAndTricksItemCategory(PrototypesTipsAndTricksItemCategory),
    TriggerTargetType(PrototypesTriggerTargetType),
    TrivialSmoke(PrototypesTrivialSmoke),
    Tutorial(PrototypesTutorial),
    UtilityConstants(PrototypesUtilityConstants),
    UtilitySounds(PrototypesUtilitySounds),
    UtilitySprites(PrototypesUtilitySprites),
    VirtualSignal(PrototypesVirtualSignal),
}
pub enum RailConnectionDirection {
    Left,
    None,
    Right,
    Straight,
}
pub enum RailDirection {
    Back,
    Front,
}
pub enum RailLayer {
    Elevated,
    Ground,
}
pub enum RelativeGuiPosition {
    Bottom,
    Left,
    Right,
    Top,
}
pub enum RelativeGuiType {
    AccumulatorGui,
    AchievementGui,
    AdditionalEntityInfoGui,
    AdminGui,
    AgricultureTowerGui,
    ArithmeticCombinatorGui,
    ArmorGui,
    AssemblingMachineGui,
    AssemblingMachineSelectRecipeGui,
    AsteroidCollectorGui,
    BeaconGui,
    BlueprintBookGui,
    BlueprintLibraryGui,
    BlueprintSetupGui,
    BonusGui,
    BurnerEquipmentGui,
    CarGui,
    CargoLandingPadGui,
    ConstantCombinatorGui,
    ContainerGui,
    ControllerGui,
    DeciderCombinatorGui,
    DeconstructionItemGui,
    DisplayPanelGui,
    ElectricEnergyInterfaceGui,
    ElectricNetworkGui,
    EntityVariationsGui,
    EntityWithEnergySourceGui,
    EquipmentGridGui,
    FurnaceGui,
    GenericOnOffEntityGui,
    GhostPickerGui,
    GlobalElectricNetworkGui,
    HeatInterfaceGui,
    InfinityPipeGui,
    InserterGui,
    ItemWithInventoryGui,
    LabGui,
    LampGui,
    LinkedContainerGui,
    LoaderGui,
    LogisticGui,
    MarketGui,
    MiningDrillGui,
    OtherPlayerGui,
    PermissionsGui,
    PickStopGui,
    PipeGui,
    PowerSwitchGui,
    ProductionGui,
    ProgrammableSpeakerGui,
    PumpGui,
    RailSignalBaseGui,
    ReactorGui,
    ResourceEntityGui,
    RoboportGui,
    RocketSiloGui,
    ScriptInventoryGui,
    SelectorCombinatorGui,
    ServerConfigGui,
    SpacePlatformHubGui,
    SpiderVehicleGui,
    SplitterGui,
    StandaloneCharacterGui,
    StorageTankGui,
    TileVariationsGui,
    TipsAndTricksGui,
    TrainGui,
    TrainStopGui,
    TrainsGui,
    TransportBeltGui,
    TurretGui,
    UpgradeItemGui,
    WallGui,
}
pub enum RenderMode {
    Chart,
    ChartZoomedIn,
    Game,
}
pub enum RichTextSetting {
    Disabled,
    Enabled,
    Highlight,
}
pub enum RidingDirection {
    Left,
    Right,
    Straight,
}
pub enum RidingAcceleration {
    Accelerating,
    Braking,
    Nothing,
    Reversing,
}
pub enum Riding {
    Acceleration(RidingAcceleration),
    Direction(RidingDirection),
}
pub enum RobotOrderType {
    Construct,
    Deconstruct,
    Deliver,
    DeliverItems,
    ExplodeCliff,
    Pickup,
    PickupItems,
    Repair,
    Upgrade,
}
pub enum RocketSiloStatus {
    ArmsAdvance,
    ArmsRetract,
    BuildingRocket,
    CreateRocket,
    DoorsClosing,
    DoorsOpened,
    DoorsOpening,
    EngineStarting,
    LaunchStarted,
    LaunchStarting,
    LightsBlinkingClose,
    LightsBlinkingOpen,
    RocketFlying,
    RocketReady,
    RocketRising,
}
pub enum SelectionMode {
    AltReverseSelect,
    AltSelect,
    ReverseSelect,
    Select,
}
pub enum Shooting {
    NotShooting,
    ShootingEnemies,
    ShootingSelected,
}
pub enum SignalState {
    Closed,
    Open,
    Reserved,
    ReservedByCircuitNetwork,
}
pub enum SpacePlatformState {
    NoPath,
    NoSchedule,
    OnThePath,
    Paused,
    StarterPackOnTheWay,
    StarterPackRequested,
    WaitingAtStation,
    WaitingForDeparture,
    WaitingForStarterPack,
}
pub enum TargetType {
    Commandable,
    CustomChartTag,
    Entity,
    Equipment,
    EquipmentGrid,
    GuiElement,
    Item,
    LogisticCell,
    LogisticNetwork,
    LogisticSection,
    PermissionGroup,
    Planet,
    Player,
    RailPath,
    RenderObject,
    SpacePlatform,
    Surface,
    Train,
}
pub enum TrainState {
    ArriveSignal,
    ArriveStation,
    DestinationFull,
    ManualControl,
    ManualControlStop,
    NoPath,
    NoSchedule,
    OnThePath,
    WaitSignal,
    WaitStation,
}
pub enum TransportLine {
    LeftLine,
    LeftSplitLine,
    LeftUndergroundLine,
    RightLine,
    RightSplitLine,
    RightUndergroundLine,
    SecondaryLeftLine,
    SecondaryLeftSplitLine,
    SecondaryRightLine,
    SecondaryRightSplitLine,
}
pub enum WireConnectorId {
    CircuitGreen,
    CircuitRed,
    CombinatorInputGreen,
    CombinatorInputRed,
    CombinatorOutputGreen,
    CombinatorOutputRed,
    PoleCopper,
    PowerSwitchLeftCopper,
    PowerSwitchRightCopper,
}
pub enum WireOrigin {
    Player,
    Radars,
    Script,
}
pub enum WireType {
    Copper,
    Green,
    Red,
}
