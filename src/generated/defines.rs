pub enum AlertType {
    Custom,
    EntityDestroyed,
    EntityUnderAttack,
    NoMaterialForConstruction,
    NoStorage,
    NotEnoughConstructionRobots,
    NotEnoughRepairPacks,
    TrainOutOfFuel,
    TurretFire,
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


pub enum CircuitConditionIndex {
    ArithmeticCombinator,
    ConstantCombinator,
    DeciderCombinator,
    InserterCircuit,
    InserterLogistic,
    Lamp,
    OffshorePump,
    Pump,
}


pub enum CircuitConnectorId {
    Accumulator,
    CombinatorInput,
    CombinatorOutput,
    ConstantCombinator,
    Container,
    ElectricPole,
    Inserter,
    Lamp,
    LinkedContainer,
    OffshorePump,
    ProgrammableSpeaker,
    Pump,
    RailChainSignal,
    RailSignal,
    Roboport,
    StorageTank,
    Wall,
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


pub enum ControlBehaviorInserterCircuitModeOfOperation {
    EnableDisable,
    None,
    ReadHandContents,
    SetFilters,
    SetStackSize,
}

pub enum ControlBehaviorInserterHandReadMode {
    Hold,
    Pulse,
}

pub enum ControlBehaviorLampCircuitModeOfOperation {
    UseColors,
}

pub enum ControlBehaviorLogisticContainerCircuitModeOfOperation {
    SendContents,
    SetRequests,
}

pub enum ControlBehaviorMiningDrillResourceReadMode {
    EntirePatch,
    ThisMiner,
}

pub enum ControlBehaviorTransportBeltContentReadMode {
    Hold,
    Pulse,
}

pub enum ControlBehaviorType {
    Accumulator,
    ArithmeticCombinator,
    ConstantCombinator,
    Container,
    DeciderCombinator,
    GenericOnOff,
    Inserter,
    Lamp,
    LogisticContainer,
    MiningDrill,
    ProgrammableSpeaker,
    RailChainSignal,
    RailSignal,
    Roboport,
    StorageTank,
    TrainStop,
    TransportBelt,
    Wall,
}


pub enum Controllers {
    Character,
    Cutscene,
    Editor,
    Ghost,
    God,
    Spectator,
}


pub enum DeconstructionItemEntityFilterMode {
    Blacklist,
    Whitelist,
}

pub enum DeconstructionItemTileFilterMode {
    Blacklist,
    Whitelist,
}

pub enum DeconstructionItemTileSelectionMode {
    Always,
    Never,
    Normal,
    Only,
}


pub enum Difficulty {
    Easy,
    Hard,
    Normal,
}


pub enum DifficultySettingsRecipeDifficulty {
    Expensive,
    Normal,
}

pub enum DifficultySettingsTechnologyDifficulty {
    Expensive,
    Normal,
}


pub enum Direction {
    East,
    North,
    Northeast,
    Northwest,
    South,
    Southeast,
    Southwest,
    West,
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
    CantDivideSegments,
    Charging,
    ClosedByCircuitNetwork,
    Disabled,
    DisabledByControlBehavior,
    DisabledByScript,
    Discharging,
    FluidIngredientShortage,
    FullOutput,
    FullyCharged,
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
    NoFuel,
    NoIngredients,
    NoInputFluid,
    NoMinableResources,
    NoModulesToTransmit,
    NoPower,
    NoRecipe,
    NoResearchInProgress,
    Normal,
    NotConnectedToRail,
    NotPluggedInElectricNetwork,
    OpenedByCircuitNetwork,
    OutOfLogisticNetwork,
    PreparingRocketForLaunch,
    RechargingAfterPowerOutage,
    TurnedOffDuringDaytime,
    WaitingForSourceItems,
    WaitingForSpaceInDestination,
    WaitingForTargetToBeBuilt,
    WaitingForTrain,
    WaitingToLaunchRocket,
    Working,
}


pub enum Events {
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
    OnCutsceneWaypointReached,
    OnDifficultySettingsChanged,
    OnEntityCloned,
    OnEntityDamaged,
    OnEntityDestroyed,
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
    OnPlayerConfiguredSpiderRemote,
    OnPlayerCraftedItem,
    OnPlayerCreated,
    OnPlayerCursorStackChanged,
    OnPlayerDeconstructedArea,
    OnPlayerDemoted,
    OnPlayerDied,
    OnPlayerDisplayResolutionChanged,
    OnPlayerDisplayScaleChanged,
    OnPlayerDrivingChangedState,
    OnPlayerDroppedItem,
    OnPlayerFastTransferred,
    OnPlayerFlushedFluid,
    OnPlayerGunInventoryChanged,
    OnPlayerJoinedGame,
    OnPlayerKicked,
    OnPlayerLeftGame,
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
    OnPlayerUsedSpiderRemote,
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
    OnPreScriptInventoryResized,
    OnPreSurfaceCleared,
    OnPreSurfaceDeleted,
    OnResearchCancelled,
    OnResearchFinished,
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
    OnSelectedEntityChanged,
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
    Item,
    Logistic,
    None,
    OtherPlayer,
    Permissions,
    PlayerManagement,
    Production,
    Research,
    ScriptInventory,
    ServerManagement,
    Tile,
    Trains,
    Tutorials,
}


pub enum InputAction {
    ActivateCopy,
    ActivateCut,
    ActivatePaste,
    AddPermissionGroup,
    AddTrainStation,
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
    CancelNewBlueprint,
    CancelResearch,
    CancelUpgrade,
    ChangeActiveCharacterTab,
    ChangeActiveItemGroupForCrafting,
    ChangeActiveItemGroupForFilters,
    ChangeActiveQuickBar,
    ChangeArithmeticCombinatorParameters,
    ChangeDeciderCombinatorParameters,
    ChangeEntityLabel,
    ChangeItemDescription,
    ChangeItemLabel,
    ChangeMultiplayerConfig,
    ChangePickingState,
    ChangeProgrammableSpeakerAlertParameters,
    ChangeProgrammableSpeakerCircuitParameters,
    ChangeProgrammableSpeakerParameters,
    ChangeRidingState,
    ChangeShootingState,
    ChangeTrainStopStation,
    ChangeTrainWaitCondition,
    ChangeTrainWaitConditionData,
    ClearCursor,
    ConnectRollingStock,
    Copy,
    CopyEntitySettings,
    CopyOpenedBlueprint,
    CopyOpenedItem,
    Craft,
    CursorSplit,
    CursorTransfer,
    CustomInput,
    CycleBlueprintBookBackwards,
    CycleBlueprintBookForwards,
    Deconstruct,
    DeleteBlueprintLibrary,
    DeleteBlueprintRecord,
    DeleteCustomTag,
    DeletePermissionGroup,
    DestroyItem,
    DestroyOpenedItem,
    DisconnectRollingStock,
    DragTrainSchedule,
    DragTrainWaitCondition,
    DropBlueprintRecord,
    DropItem,
    EditBlueprintToolPreview,
    EditCustomTag,
    EditPermissionGroup,
    ExportBlueprint,
    FastEntitySplit,
    FastEntityTransfer,
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
    InventorySplit,
    InventoryTransfer,
    LaunchRocket,
    LuaShortcut,
    MapEditorAction,
    MarketOffer,
    ModSettingsChanged,
    OpenAchievementsGui,
    OpenBlueprintLibraryGui,
    OpenBlueprintRecord,
    OpenBonusGui,
    OpenCharacterGui,
    OpenCurrentVehicleGui,
    OpenEquipment,
    OpenGui,
    OpenItem,
    OpenLogisticGui,
    OpenModItem,
    OpenParentOfOpenedItem,
    OpenProductionGui,
    OpenTechnologyGui,
    OpenTipsAndTricksGui,
    OpenTrainGui,
    OpenTrainStationGui,
    OpenTrainsGui,
    PasteEntitySettings,
    PlaceEquipment,
    QuickBarPickSlot,
    QuickBarSetSelectedPage,
    QuickBarSetSlot,
    ReassignBlueprint,
    RemoveCables,
    RemoveTrainStation,
    ResetAssemblingMachine,
    ResetItem,
    ReverseSelectArea,
    RotateEntity,
    SelectArea,
    SelectBlueprintEntities,
    SelectEntitySlot,
    SelectItem,
    SelectMapperSlot,
    SelectNextValidGun,
    SelectTileSlot,
    SendSpidertron,
    SetAutoLaunchRocket,
    SetAutosortInventory,
    SetBehaviorMode,
    SetCarWeaponsControl,
    SetCircuitCondition,
    SetCircuitModeOfOperation,
    SetControllerLogisticTrashFilterItem,
    SetDeconstructionItemTileSelectionMode,
    SetDeconstructionItemTreesAndRocksOnly,
    SetEntityColor,
    SetEntityEnergyProperty,
    SetEntityLogisticTrashFilterItem,
    SetFilter,
    SetFlatControllerGui,
    SetHeatInterfaceMode,
    SetHeatInterfaceTemperature,
    SetInfinityContainerFilterItem,
    SetInfinityContainerRemoveUnfilteredItems,
    SetInfinityPipeFilter,
    SetInserterMaxStackSize,
    SetInventoryBar,
    SetLinkedContainerLinkID,
    SetLogisticFilterItem,
    SetLogisticFilterSignal,
    SetPlayerColor,
    SetRecipeNotifications,
    SetRequestFromBuffers,
    SetResearchFinishedStopsGame,
    SetSignal,
    SetSplitterPriority,
    SetTrainStopped,
    SetTrainsLimit,
    SetVehicleAutomaticTargetingParameters,
    SetupAssemblingMachine,
    SetupBlueprint,
    SetupSingleBlueprintRecord,
    SmartPipette,
    SpawnItem,
    StackSplit,
    StackTransfer,
    StartRepair,
    StartResearch,
    StartWalking,
    StopBuildingByMoving,
    SwitchConnectToLogisticNetwork,
    SwitchConstantCombinatorState,
    SwitchInserterFilterModeState,
    SwitchPowerSwitchState,
    SwitchToRenameStopGui,
    TakeEquipment,
    ToggleDeconstructionItemEntityFilterMode,
    ToggleDeconstructionItemTileFilterMode,
    ToggleDriving,
    ToggleEnableVehicleLogisticsWhileMoving,
    ToggleEntityLogisticRequests,
    ToggleEquipmentMovementBonus,
    ToggleMapEditor,
    TogglePersonalLogisticRequests,
    TogglePersonalRoboport,
    ToggleShowEntityInfo,
    TranslateString,
    Undo,
    Upgrade,
    UpgradeOpenedBlueprintByItem,
    UpgradeOpenedBlueprintByRecord,
    UseArtilleryRemote,
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
    AssemblingMachineInput,
    AssemblingMachineModules,
    AssemblingMachineOutput,
    BeaconModules,
    BurntResult,
    CarAmmo,
    CarTrunk,
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
    ItemMain,
    LabInput,
    LabModules,
    MiningDrillModules,
    RoboportMaterial,
    RoboportRobot,
    RobotCargo,
    RobotRepair,
    Rocket,
    RocketSiloInput,
    RocketSiloModules,
    RocketSiloOutput,
    RocketSiloResult,
    RocketSiloRocket,
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


pub enum MouseButtonType {
    Left,
    Middle,
    None,
    Right,
}


pub struct Prototypes;


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
    ArithmeticCombinatorGui,
    ArmorGui,
    AssemblingMachineGui,
    AssemblingMachineSelectRecipeGui,
    BeaconGui,
    BlueprintBookGui,
    BlueprintLibraryGui,
    BlueprintSetupGui,
    BonusGui,
    BurnerEquipmentGui,
    CarGui,
    ConstantCombinatorGui,
    ContainerGui,
    ControllerGui,
    DeciderCombinatorGui,
    DeconstructionItemGui,
    ElectricEnergyInterfaceGui,
    ElectricNetworkGui,
    EntityVariationsGui,
    EntityWithEnergySourceGui,
    EquipmentGridGui,
    FurnaceGui,
    GenericOnOffEntityGui,
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
    PipeGui,
    PowerSwitchGui,
    ProductionGui,
    ProgrammableSpeakerGui,
    RailChainSignalGui,
    RailSignalGui,
    ReactorGui,
    RenameStopGui,
    ResourceEntityGui,
    RoboportGui,
    RocketSiloGui,
    ScriptInventoryGui,
    ServerConfigGui,
    SpiderVehicleGui,
    SplitterGui,
    StandaloneCharacterGui,
    StorageTankGui,
    TileVariationsGui,
    TrainGui,
    TrainStopGui,
    TrainsGui,
    TransportBeltGui,
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


pub enum RidingAcceleration {
    Accelerating,
    Braking,
    Nothing,
    Reversing,
}

pub enum RidingDirection {
    Left,
    Right,
    Straight,
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


pub enum TrainState {
    ArriveSignal,
    ArriveStation,
    DestinationFull,
    ManualControl,
    ManualControlStop,
    NoPath,
    NoSchedule,
    OnThePath,
    PathLost,
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


pub enum WireConnectionId {
    ElectricPole,
    PowerSwitchLeft,
    PowerSwitchRight,
}


pub enum WireType {
    Copper,
    Green,
    Red,
}


