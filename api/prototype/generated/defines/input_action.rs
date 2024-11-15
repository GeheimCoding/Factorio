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
