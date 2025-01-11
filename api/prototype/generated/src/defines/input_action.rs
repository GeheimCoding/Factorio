#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum InputAction {
    ActivateInterrupt = 176,
    ActivatePaste = 42,
    AddDeciderCombinatorCondition = 157,
    AddDeciderCombinatorOutput = 161,
    AddLogisticSection = 53,
    AddPermissionGroup = 272,
    AddPin = 305,
    AddTrainInterrupt = 175,
    AddTrainStation = 174,
    AdjustBlueprintSnapping = 316,
    AdminAction = 229,
    AltReverseSelectArea = 196,
    AltSelectArea = 194,
    AltSelectBlueprintEntities = 129,
    AlternativeCopy = 127,
    BeginMining = 2,
    BeginMiningTerrain = 67,
    Build = 65,
    BuildRail = 190,
    BuildTerrain = 169,
    CancelCraft = 89,
    CancelDeconstruct = 153,
    CancelDeleteSpacePlatform = 234,
    CancelNewBlueprint = 15,
    CancelResearch = 191,
    CancelUpgrade = 154,
    ChangeActiveCharacterTab = 107,
    ChangeActiveItemGroupForCrafting = 105,
    ChangeActiveItemGroupForFilters = 106,
    ChangeActiveQuickBar = 277,
    ChangeArithmeticCombinatorParameters = 155,
    ChangeEntityLabel = 184,
    ChangeItemLabel = 183,
    ChangeLogisticPointGroup = 186,
    ChangeMultiplayerConfig = 228,
    ChangePickingState = 241,
    ChangeProgrammableSpeakerAlertParameters = 166,
    ChangeProgrammableSpeakerCircuitParameters = 167,
    ChangeProgrammableSpeakerParameters = 165,
    ChangeRidingState = 68,
    ChangeSelectorCombinatorParameters = 164,
    ChangeShootingState = 83,
    ChangeTrainName = 185,
    ChangeTrainStopStation = 104,
    ChangeTrainWaitCondition = 170,
    ChangeTrainWaitConditionData = 171,
    ClearCursor = 11,
    ConnectRollingStock = 8,
    Copy = 126,
    CopyEntitySettings = 17,
    CopyLargeOpenedBlueprint = 133,
    CopyLargeOpenedItem = 21,
    CopyOpenedBlueprint = 132,
    CopyOpenedItem = 20,
    Craft = 81,
    CreateSpacePlatform = 232,
    CursorSplit = 75,
    CursorTransfer = 74,
    CustomInput = 182,
    CycleBlueprintBookBackwards = 29,
    CycleBlueprintBookForwards = 28,
    CycleQualityDown = 31,
    CycleQualityUp = 30,
    Deconstruct = 124,
    DeleteBlueprintLibrary = 40,
    DeleteBlueprintRecord = 138,
    DeleteCustomTag = 270,
    DeleteLogisticGroup = 188,
    DeletePermissionGroup = 271,
    DeleteSpacePlatform = 233,
    DestroyItem = 71,
    DestroyOpenedItem = 19,
    DisconnectRollingStock = 9,
    DragDeciderCombinatorCondition = 156,
    DragDeciderCombinatorOutput = 160,
    DragTrainSchedule = 210,
    DragTrainScheduleInterrupt = 211,
    DragTrainWaitCondition = 212,
    DropBlueprintRecord = 137,
    DropItem = 64,
    EditBlueprintToolPreview = 146,
    EditCustomTag = 202,
    EditDisplayPanel = 298,
    EditDisplayPanelAlwaysShow = 299,
    EditDisplayPanelIcon = 301,
    EditDisplayPanelParameters = 302,
    EditDisplayPanelShowInChart = 300,
    EditInterrupt = 177,
    EditPermissionGroup = 203,
    EditPin = 309,
    EnableTransitionalRequests = 276,
    ExportBlueprint = 148,
    FastEntitySplit = 258,
    FastEntityTransfer = 255,
    FlipEntity = 257,
    FlushOpenedEntityFluid = 51,
    FlushOpenedEntitySpecificFluid = 240,
    GoToTrainStation = 179,
    GrabBlueprintRecord = 136,
    GuiCheckedStateChanged = 109,
    GuiClick = 100,
    GuiConfirmed = 101,
    GuiElemChanged = 208,
    GuiHover = 312,
    GuiLeave = 313,
    GuiLocationChanged = 114,
    GuiSelectedTabChanged = 111,
    GuiSelectionStateChanged = 110,
    GuiSwitchStateChanged = 113,
    GuiTextChanged = 108,
    GuiValueChanged = 112,
    ImportBlueprint = 149,
    ImportBlueprintString = 204,
    ImportBlueprintsFiltered = 150,
    ImportPermissionsString = 205,
    InstantlyCreateSpacePlatform = 239,
    InventorySplit = 88,
    InventoryTransfer = 79,
    LandAtPlanet = 319,
    LaunchRocket = 187,
    LuaShortcut = 230,
    MapEditorAction = 225,
    MarketOffer = 103,
    ModSettingsChanged = 200,
    ModifyDeciderCombinatorCondition = 158,
    ModifyDeciderCombinatorOutput = 162,
    MoveResearch = 192,
    OpenAchievementsGui = 27,
    OpenBlueprintLibraryGui = 62,
    OpenBlueprintRecord = 135,
    OpenBonusGui = 26,
    OpenCharacterGui = 6,
    OpenCurrentVehicleGui = 7,
    OpenEquipment = 73,
    OpenGlobalElectricNetworkGui = 294,
    OpenGui = 5,
    OpenItem = 69,
    OpenLogisticsGui = 61,
    OpenModItem = 72,
    OpenNewPlatformButtonFromRocketSilo = 58,
    OpenOpenedEntityGrid = 55,
    OpenParentOfOpenedItem = 70,
    OpenProductionGui = 13,
    OpenTrainGui = 264,
    OpenTrainStationGui = 284,
    OpenTrainsGui = 265,
    ParametriseBlueprint = 321,
    PasteEntitySettings = 18,
    PinAlertGroup = 307,
    PinCustomAlert = 308,
    PinSearchResult = 306,
    Pipette = 86,
    PlaceEquipment = 115,
    QuickBarPickSlot = 222,
    QuickBarSetSelectedPage = 223,
    QuickBarSetSlot = 221,
    ReassignBlueprint = 134,
    Redo = 44,
    RemoteViewEntity = 237,
    RemoteViewSurface = 236,
    RemoveCables = 147,
    RemoveDeciderCombinatorCondition = 159,
    RemoveDeciderCombinatorOutput = 163,
    RemoveLogisticSection = 297,
    RemovePin = 310,
    RemoveTrainInterrupt = 173,
    RemoveTrainStation = 172,
    RenameInterrupt = 178,
    RenameSpacePlatform = 235,
    ReorderLogisticSection = 303,
    RequestMissingConstructionMaterials = 259,
    ResetAssemblingMachine = 12,
    ReverseSelectArea = 195,
    RotateEntity = 256,
    SelectArea = 193,
    SelectAsteroidChunkSlot = 216,
    SelectBlueprintEntities = 128,
    SelectEntityFilterSlot = 215,
    SelectEntitySlot = 214,
    SelectItemFilter = 213,
    SelectMapperSlotFrom = 218,
    SelectMapperSlotTo = 219,
    SelectNextValidGun = 38,
    SelectTileSlot = 217,
    SendSpidertron = 118,
    SendStackToTrash = 77,
    SendStacksToTrash = 78,
    SendTrainToPinTarget = 311,
    SetBehaviorMode = 254,
    SetCarWeaponsControl = 274,
    SetCheatModeQuality = 96,
    SetCircuitCondition = 93,
    SetCircuitModeOfOperation = 99,
    SetCombinatorDescription = 246,
    SetCopyColorFromTrainStop = 267,
    SetDeconstructionItemTileSelectionMode = 269,
    SetDeconstructionItemTreesAndRocksOnly = 268,
    SetEntityColor = 266,
    SetEntityEnergyProperty = 201,
    SetFilter = 90,
    SetGhostCursor = 142,
    SetHeatInterfaceMode = 283,
    SetHeatInterfaceTemperature = 282,
    SetInfinityContainerFilterItem = 198,
    SetInfinityContainerRemoveUnfilteredItems = 273,
    SetInfinityPipeFilter = 199,
    SetInserterMaxStackSize = 263,
    SetInventoryBar = 119,
    SetLampAlwaysOn = 293,
    SetLinkedContainerLinkID = 291,
    SetLogisticFilterItem = 97,
    SetLogisticNetworkName = 189,
    SetLogisticSectionActive = 304,
    SetPlayerColor = 287,
    SetPumpFluidFilter = 295,
    SetRequestFromBuffers = 275,
    SetResearchFinishedStopsGame = 262,
    SetRocketSiloSendToOrbitAutomatedMode = 323,
    SetScheduleRecordAllowUnloading = 181,
    SetSignal = 94,
    SetSplitterPriority = 280,
    SetSpoilPriority = 91,
    SetTrainStopPriority = 317,
    SetTrainStopped = 180,
    SetTrainsLimit = 289,
    SetTurretIgnoreUnlisted = 292,
    SetUseInserterFilters = 250,
    SetVehicleAutomaticTargetingParameters = 168,
    SetupAssemblingMachine = 84,
    SetupBlueprint = 130,
    SetupSingleBlueprintRecord = 131,
    SpawnItem = 141,
    SpectatorChangeSurface = 315,
    StackSplit = 87,
    StackTransfer = 76,
    StartRepair = 123,
    StartResearch = 95,
    StartWalking = 66,
    StopDragBuild = 50,
    SwapLogisticFilterItems = 98,
    SwitchConnectToLogisticNetwork = 253,
    SwitchConstantCombinatorState = 247,
    SwitchInserterFilterModeState = 249,
    SwitchMiningDrillFilterModeState = 252,
    SwitchPowerSwitchState = 248,
    TakeEquipment = 116,
    ToggleArtilleryAutoTargeting = 49,
    ToggleDeconstructionItemEntityFilterMode = 36,
    ToggleDeconstructionItemTileFilterMode = 37,
    ToggleDriving = 4,
    ToggleEnableVehicleLogisticsWhileMoving = 35,
    ToggleEntityLogisticRequests = 48,
    ToggleEquipmentMovementBonus = 46,
    ToggleMapEditor = 39,
    TogglePersonalLogisticRequests = 47,
    TogglePersonalRoboport = 45,
    ToggleSelectedEntity = 59,
    ToggleShowEntityInfo = 22,
    TranslateString = 231,
    TrashNotRequestedItems = 260,
    Undo = 43,
    Upgrade = 125,
    UpgradeOpenedBlueprintByItem = 140,
    UpgradeOpenedBlueprintByRecord = 139,
    UseItem = 117,
    WireDragging = 82,
    WriteToConsole = 102,
}