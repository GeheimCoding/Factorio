use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
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

/// AI command exit status. See [LuaEntity::set_command](LuaEntity::set_command)
#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum BehaviorResult {
    Deleted,
    Fail,
    InProgress,
    Success,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum BuildCheckType {
    BlueprintGhost,
    GhostRevive,
    Manual,
    ManualGhost,
    Script,
    ScriptGhost,
}

/// State of a chain signal.
#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum ChainSignalState {
    AllOpen,
    None,
    NoneOpen,
    PartiallyOpen,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum ChunkGeneratedStatus {
    BasicTiles,
    CorrectedTiles,
    CustomTiles,
    Entities,
    Nothing,
    Tiles,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
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

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
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

/// Command given to units describing what they should do.
#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum CommandDefine {
    /// Attack another entity.
    Attack,
    /// Go to a place and attack what you see.
    AttackArea,
    /// Go to a position and build a base there.
    BuildBase,
    /// Chain commands together, see [defines.compound_command](defines.compound_command).
    Compound,
    /// Flee from another entity.
    Flee,
    /// Go to a specific position.
    GoToLocation,
    /// Do what your group wants you to do.
    Group,
    /// Stop moving and stay where you are.
    Stop,
    /// Chill.
    Wander,
}

/// How commands are joined together in a compound command (see [defines.command.compound](defines.command.compound)).
#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum CompoundCommand {
    /// Fail on first failure. Only succeeds if all commands (executed one after another) succeed.
    LogicalAnd,
    /// Succeed on first success. Only fails if all commands (executed one after another) fail.
    LogicalOr,
    /// Execute all commands in sequence and fail or succeed depending on the return status of the last command.
    ReturnLast,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum ControlBehaviorInserterCircuitModeOfOperation {
    EnableDisable,
    None,
    ReadHandContents,
    SetFilters,
    SetStackSize,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum ControlBehaviorInserterHandReadMode {
    Hold,
    Pulse,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum ControlBehaviorLampCircuitModeOfOperation {
    UseColors,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum ControlBehaviorLogisticContainerCircuitModeOfOperation {
    SendContents,
    SetRequests,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum ControlBehaviorMiningDrillResourceReadMode {
    EntirePatch,
    ThisMiner,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum ControlBehaviorTransportBeltContentReadMode {
    Hold,
    Pulse,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum ControlBehaviorType {
    /// [LuaAccumulatorControlBehavior](LuaAccumulatorControlBehavior)
    Accumulator,
    /// [LuaArithmeticCombinatorControlBehavior](LuaArithmeticCombinatorControlBehavior)
    ArithmeticCombinator,
    /// [LuaConstantCombinatorControlBehavior](LuaConstantCombinatorControlBehavior)
    ConstantCombinator,
    /// [LuaContainerControlBehavior](LuaContainerControlBehavior)
    Container,
    /// [LuaDeciderCombinatorControlBehavior](LuaDeciderCombinatorControlBehavior)
    DeciderCombinator,
    /// [LuaGenericOnOffControlBehavior](LuaGenericOnOffControlBehavior)
    GenericOnOff,
    /// [LuaInserterControlBehavior](LuaInserterControlBehavior)
    Inserter,
    /// [LuaLampControlBehavior](LuaLampControlBehavior)
    Lamp,
    /// [LuaLogisticContainerControlBehavior](LuaLogisticContainerControlBehavior)
    LogisticContainer,
    /// [LuaMiningDrillControlBehavior](LuaMiningDrillControlBehavior)
    MiningDrill,
    /// [LuaProgrammableSpeakerControlBehavior](LuaProgrammableSpeakerControlBehavior)
    ProgrammableSpeaker,
    /// [LuaRailChainSignalControlBehavior](LuaRailChainSignalControlBehavior)
    RailChainSignal,
    /// [LuaRailSignalControlBehavior](LuaRailSignalControlBehavior)
    RailSignal,
    /// [LuaRoboportControlBehavior](LuaRoboportControlBehavior)
    Roboport,
    /// [LuaStorageTankControlBehavior](LuaStorageTankControlBehavior)
    StorageTank,
    /// [LuaTrainStopControlBehavior](LuaTrainStopControlBehavior)
    TrainStop,
    /// [LuaTransportBeltControlBehavior](LuaTransportBeltControlBehavior)
    TransportBelt,
    /// [LuaWallControlBehavior](LuaWallControlBehavior)
    Wall,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum Controllers {
    /// The controller controls a character. This is the default controller in freeplay.
    Character,
    /// The player can't interact with the world, and the camera pans around in a predefined manner.
    Cutscene,
    /// The Editor Controller near ultimate power to do almost anything in the game.
    Editor,
    /// Can't interact with the world, can only observe. Used in the multiplayer waiting-to-respawn screen.
    Ghost,
    /// The controller isn't tied to a character. This is the default controller in sandbox.
    God,
    /// Can't change anything in the world but can view anything.
    Spectator,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum DeconstructionItemEntityFilterMode {
    Blacklist,
    Whitelist,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum DeconstructionItemTileFilterMode {
    Blacklist,
    Whitelist,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum DeconstructionItemTileSelectionMode {
    Always,
    Never,
    Normal,
    Only,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum Difficulty {
    Easy,
    Hard,
    Normal,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum DifficultySettingsRecipeDifficulty {
    Expensive,
    Normal,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum DifficultySettingsTechnologyDifficulty {
    Expensive,
    Normal,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
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

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
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

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum Distraction {
    /// Attack closer enemy entities, including entities "built" by player (belts, inserters, chests).
    ByAnything,
    /// Attack when attacked.
    ByDamage,
    /// Attack closer enemy entities with force.
    ByEnemy,
    /// Perform command even if someone attacks the unit.
    None,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum EntityStatus {
    /// Used by rail signals.
    CantDivideSegments,
    /// Used by accumulators.
    Charging,
    ClosedByCircuitNetwork,
    /// Used by constant combinators: Combinator is turned off via switch in GUI.
    Disabled,
    DisabledByControlBehavior,
    DisabledByScript,
    /// Used by accumulators.
    Discharging,
    /// Used by crafting machines.
    FluidIngredientShortage,
    /// Used by crafting machines, boilers, burner energy sources and reactors: Reactor/burner has full burnt result inventory, boiler has full output fluidbox.
    FullOutput,
    /// Used by accumulators.
    FullyCharged,
    /// Used by crafting machines.
    ItemIngredientShortage,
    /// Used by the rocket silo.
    LaunchingRocket,
    /// Used by boilers and fluid turrets: Boiler still has some fluid but is about to run out.
    LowInputFluid,
    LowPower,
    /// Used by heat energy sources.
    LowTemperature,
    MarkedForDeconstruction,
    /// Used by mining drills when the mining fluid is missing.
    MissingRequiredFluid,
    /// Used by labs.
    MissingSciencePacks,
    /// Used by power switches.
    NetworksConnected,
    /// Used by power switches.
    NetworksDisconnected,
    /// Used by ammo turrets.
    NoAmmo,
    NoFuel,
    /// Used by furnaces.
    NoIngredients,
    /// Used by boilers, fluid turrets and fluid energy sources: Boiler has no fluid to work with.
    NoInputFluid,
    /// Used by mining drills.
    NoMinableResources,
    /// Used by beacons.
    NoModulesToTransmit,
    NoPower,
    /// Used by assembling machines.
    NoRecipe,
    /// Used by labs.
    NoResearchInProgress,
    Normal,
    /// Used by rail signals.
    NotConnectedToRail,
    /// Used by generators and solar panels.
    NotPluggedInElectricNetwork,
    OpenedByCircuitNetwork,
    /// Used by logistic containers.
    OutOfLogisticNetwork,
    /// Used by the rocket silo.
    PreparingRocketForLaunch,
    /// Used by roboports.
    RechargingAfterPowerOutage,
    /// Used by lamps.
    TurnedOffDuringDaytime,
    /// Used by inserters.
    WaitingForSourceItems,
    /// Used by inserters and mining drills.
    WaitingForSpaceInDestination,
    /// Used by inserters targeting entity ghosts.
    WaitingForTargetToBeBuilt,
    /// Used by inserters targeting rails.
    WaitingForTrain,
    /// Used by the rocket silo.
    WaitingToLaunchRocket,
    Working,
}

/// See the [events page](events.html) for more info on what events contain and when they get raised.
#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
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

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
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

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum GroupState {
    AttackingDistraction,
    AttackingTarget,
    Finished,
    Gathering,
    Moving,
    Pathfinding,
    WanderInGroup,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
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

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
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

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum InputMethod {
    GameController,
    KeyboardAndMouse,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
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

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum LogisticMemberIndex {
    CharacterProvider,
    CharacterRequester,
    CharacterStorage,
    GenericOnOffBehavior,
    LogisticContainer,
    VehicleStorage,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum LogisticMode {
    ActiveProvider,
    Buffer,
    None,
    PassiveProvider,
    Requester,
    Storage,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum MouseButtonType {
    Left,
    Middle,
    None,
    Right,
}

/// A dictionary mapping all top-level prototypes by name to a list of their associated subtypes. This list is organized as a lookup table, meaning it maps the sub-prototype names to `0`. As an example, `defines.prototypes['entity']` looks like this: `{furnace=0, inserter=0, container=0, ...}`.
pub struct _Prototypes;

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum RailConnectionDirection {
    Left,
    None,
    Right,
    Straight,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum RailDirection {
    Back,
    Front,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum RelativeGuiPosition {
    Bottom,
    Left,
    Right,
    Top,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
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

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum RenderMode {
    Chart,
    ChartZoomedIn,
    Game,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum RichTextSetting {
    Disabled,
    Enabled,
    Highlight,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum RidingAcceleration {
    Accelerating,
    Braking,
    Nothing,
    Reversing,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum RidingDirection {
    Left,
    Right,
    Straight,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
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

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum Shooting {
    NotShooting,
    ShootingEnemies,
    ShootingSelected,
}

/// State of an ordinary rail signal.
#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum SignalState {
    /// Red.
    Closed,
    /// Green.
    Open,
    /// Orange.
    Reserved,
    /// Red - From circuit network.
    ReservedByCircuitNetwork,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum TrainState {
    /// Braking before a rail signal.
    ArriveSignal,
    /// Braking before a station.
    ArriveStation,
    /// Same as no_path but all candidate train stops are full
    DestinationFull,
    /// Can move if user explicitly sits in and rides the train.
    ManualControl,
    /// Switched to manual control and has to stop.
    ManualControlStop,
    /// Has no path and is stopped.
    NoPath,
    /// Doesn't have anywhere to go.
    NoSchedule,
    /// Normal state -- following the path.
    OnThePath,
    /// Had path and lost it -- must stop.
    PathLost,
    /// Waiting at a signal.
    WaitSignal,
    /// Waiting at a station.
    WaitStation,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
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

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum WireConnectionId {
    ElectricPole,
    PowerSwitchLeft,
    PowerSwitchRight,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum WireType {
    Copper,
    Green,
    Red,
}
