use serde::Deserialize;

use super::classes::*;
use super::concepts::*;
use super::events::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "serde_type")]
pub enum FactorioType {
    Class(Class),
    Concept(Concept),
    Event(Event),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "serde_tag")]
pub enum Class {
    LuaAISettings(LuaAISettings),
    LuaAccumulatorControlBehavior(LuaAccumulatorControlBehavior),
    LuaAchievementPrototype(LuaAchievementPrototype),
    LuaAmmoCategoryPrototype(LuaAmmoCategoryPrototype),
    LuaArithmeticCombinatorControlBehavior(LuaArithmeticCombinatorControlBehavior),
    LuaAutoplaceControlPrototype(LuaAutoplaceControlPrototype),
    LuaBootstrap(LuaBootstrap),
    LuaBurner(LuaBurner),
    LuaBurnerPrototype(LuaBurnerPrototype),
    LuaChunkIterator(LuaChunkIterator),
    LuaCircuitNetwork(LuaCircuitNetwork),
    LuaCombinatorControlBehavior(LuaCombinatorControlBehavior),
    LuaCommandProcessor(LuaCommandProcessor),
    LuaConstantCombinatorControlBehavior(LuaConstantCombinatorControlBehavior),
    LuaContainerControlBehavior(LuaContainerControlBehavior),
    LuaControl(LuaControl),
    LuaControlBehavior(LuaControlBehavior),
    LuaCustomChartTag(LuaCustomChartTag),
    LuaCustomInputPrototype(LuaCustomInputPrototype),
    LuaCustomTable(LuaCustomTable),
    LuaDamagePrototype(LuaDamagePrototype),
    LuaDeciderCombinatorControlBehavior(LuaDeciderCombinatorControlBehavior),
    LuaDecorativePrototype(LuaDecorativePrototype),
    LuaElectricEnergySourcePrototype(LuaElectricEnergySourcePrototype),
    LuaEntity(LuaEntity),
    LuaEntityPrototype(LuaEntityPrototype),
    LuaEquipment(LuaEquipment),
    LuaEquipmentCategoryPrototype(LuaEquipmentCategoryPrototype),
    LuaEquipmentGrid(LuaEquipmentGrid),
    LuaEquipmentGridPrototype(LuaEquipmentGridPrototype),
    LuaEquipmentPrototype(LuaEquipmentPrototype),
    LuaFlowStatistics(LuaFlowStatistics),
    LuaFluidBox(LuaFluidBox),
    LuaFluidBoxPrototype(LuaFluidBoxPrototype),
    LuaFluidEnergySourcePrototype(LuaFluidEnergySourcePrototype),
    LuaFluidPrototype(LuaFluidPrototype),
    LuaFontPrototype(LuaFontPrototype),
    LuaForce(LuaForce),
    LuaFuelCategoryPrototype(LuaFuelCategoryPrototype),
    LuaGameScript(LuaGameScript),
    LuaGenericOnOffControlBehavior(LuaGenericOnOffControlBehavior),
    LuaGroup(LuaGroup),
    LuaGui(LuaGui),
    LuaGuiElement(LuaGuiElement),
    LuaHeatBufferPrototype(LuaHeatBufferPrototype),
    LuaHeatEnergySourcePrototype(LuaHeatEnergySourcePrototype),
    LuaInserterControlBehavior(LuaInserterControlBehavior),
    LuaInventory(LuaInventory),
    LuaItemPrototype(LuaItemPrototype),
    LuaItemStack(LuaItemStack),
    LuaLampControlBehavior(LuaLampControlBehavior),
    LuaLazyLoadedValue(LuaLazyLoadedValue),
    LuaLogisticCell(LuaLogisticCell),
    LuaLogisticContainerControlBehavior(LuaLogisticContainerControlBehavior),
    LuaLogisticNetwork(LuaLogisticNetwork),
    LuaLogisticPoint(LuaLogisticPoint),
    LuaMiningDrillControlBehavior(LuaMiningDrillControlBehavior),
    LuaModSettingPrototype(LuaModSettingPrototype),
    LuaModuleCategoryPrototype(LuaModuleCategoryPrototype),
    LuaNamedNoiseExpression(LuaNamedNoiseExpression),
    LuaNoiseLayerPrototype(LuaNoiseLayerPrototype),
    LuaParticlePrototype(LuaParticlePrototype),
    LuaPermissionGroup(LuaPermissionGroup),
    LuaPermissionGroups(LuaPermissionGroups),
    LuaPlayer(LuaPlayer),
    LuaProfiler(LuaProfiler),
    LuaProgrammableSpeakerControlBehavior(LuaProgrammableSpeakerControlBehavior),
    LuaRCON(LuaRCON),
    LuaRailChainSignalControlBehavior(LuaRailChainSignalControlBehavior),
    LuaRailPath(LuaRailPath),
    LuaRailSignalControlBehavior(LuaRailSignalControlBehavior),
    LuaRandomGenerator(LuaRandomGenerator),
    LuaRecipe(LuaRecipe),
    LuaRecipeCategoryPrototype(LuaRecipeCategoryPrototype),
    LuaRecipePrototype(LuaRecipePrototype),
    LuaRemote(LuaRemote),
    LuaRendering(LuaRendering),
    LuaResourceCategoryPrototype(LuaResourceCategoryPrototype),
    LuaRoboportControlBehavior(LuaRoboportControlBehavior),
    LuaSettings(LuaSettings),
    LuaShortcutPrototype(LuaShortcutPrototype),
    LuaStorageTankControlBehavior(LuaStorageTankControlBehavior),
    LuaStyle(LuaStyle),
    LuaSurface(LuaSurface),
    LuaTechnology(LuaTechnology),
    LuaTechnologyPrototype(LuaTechnologyPrototype),
    LuaTile(LuaTile),
    LuaTilePrototype(LuaTilePrototype),
    LuaTrain(LuaTrain),
    LuaTrainStopControlBehavior(LuaTrainStopControlBehavior),
    LuaTransportBeltControlBehavior(LuaTransportBeltControlBehavior),
    LuaTransportLine(LuaTransportLine),
    LuaTrivialSmokePrototype(LuaTrivialSmokePrototype),
    LuaUnitGroup(LuaUnitGroup),
    LuaVirtualSignalPrototype(LuaVirtualSignalPrototype),
    LuaVoidEnergySourcePrototype(LuaVoidEnergySourcePrototype),
    LuaWallControlBehavior(LuaWallControlBehavior),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "serde_tag")]
pub enum Concept {
    AchievementPrototypeFilter(AchievementPrototypeFilter),
    AdvancedMapGenSettings(AdvancedMapGenSettings),
    Alert(Alert),
    Alignment(Alignment),
    AmmoType(AmmoType),
    Any(Any),
    AnyBasic(AnyBasic),
    ArithmeticCombinatorParameters(ArithmeticCombinatorParameters),
    AttackParameterFluid(AttackParameterFluid),
    AttackParameters(AttackParameters),
    AutoplaceControl(AutoplaceControl),
    AutoplaceSettings(AutoplaceSettings),
    AutoplaceSpecification(AutoplaceSpecification),
    AutoplaceSpecificationPeak(AutoplaceSpecificationPeak),
    AutoplaceSpecificationRestriction(AutoplaceSpecificationRestriction),
    BeamTarget(BeamTarget),
    BlueprintEntity(BlueprintEntity),
    BlueprintSignalIcon(BlueprintSignalIcon),
    BoundingBox(BoundingBox),
    CapsuleAction(CapsuleAction),
    ChartTagSpec(ChartTagSpec),
    ChunkPosition(ChunkPosition),
    ChunkPositionAndArea(ChunkPositionAndArea),
    CircuitCondition(CircuitCondition),
    CircuitConditionDefinition(CircuitConditionDefinition),
    CircuitConnectionDefinition(CircuitConnectionDefinition),
    CircularParticleCreationSpecification(CircularParticleCreationSpecification),
    CircularProjectileCreationSpecification(CircularProjectileCreationSpecification),
    CliffOrientation(CliffOrientation),
    CliffPlacementSettings(CliffPlacementSettings),
    CollisionMask(CollisionMask),
    CollisionMaskLayer(CollisionMaskLayer),
    CollisionMaskWithFlags(CollisionMaskWithFlags),
    Color(Color),
    ColorModifier(ColorModifier),
    Command(Command),
    ComparatorString(ComparatorString),
    ConfigurationChangedData(ConfigurationChangedData),
    ConstantCombinatorParameters(ConstantCombinatorParameters),
    CraftingQueueItem(CraftingQueueItem),
    CursorBoxRenderType(CursorBoxRenderType),
    CustomCommandData(CustomCommandData),
    CutsceneWaypoint(CutsceneWaypoint),
    DeciderCombinatorParameters(DeciderCombinatorParameters),
    Decorative(Decorative),
    DecorativePrototypeFilter(DecorativePrototypeFilter),
    DecorativeResult(DecorativeResult),
    DifficultySettings(DifficultySettings),
    DisplayResolution(DisplayResolution),
    DragTarget(DragTarget),
    EnemyEvolutionMapSettings(EnemyEvolutionMapSettings),
    EnemyExpansionMapSettings(EnemyExpansionMapSettings),
    EntityPrototypeFilter(EntityPrototypeFilter),
    EntityPrototypeFlags(EntityPrototypeFlags),
    EntityPrototypeIdentification(EntityPrototypeIdentification),
    EquipmentPoint(EquipmentPoint),
    EquipmentPosition(EquipmentPosition),
    EquipmentPrototypeFilter(EquipmentPrototypeFilter),
    EventData(EventData),
    EventFilter(EventFilter),
    Fluid(Fluid),
    FluidBoxConnection(FluidBoxConnection),
    FluidBoxFilter(FluidBoxFilter),
    FluidBoxFilterSpec(FluidBoxFilterSpec),
    FluidIdentification(FluidIdentification),
    FluidPrototypeFilter(FluidPrototypeFilter),
    ForceCondition(ForceCondition),
    ForceIdentification(ForceIdentification),
    GameViewSettings(GameViewSettings),
    GuiAnchor(GuiAnchor),
    GuiArrowSpecification(GuiArrowSpecification),
    GuiLocation(GuiLocation),
    HeatConnection(HeatConnection),
    HeatSetting(HeatSetting),
    InfinityInventoryFilter(InfinityInventoryFilter),
    InfinityPipeFilter(InfinityPipeFilter),
    Ingredient(Ingredient),
    InserterCircuitConditions(InserterCircuitConditions),
    InventoryFilter(InventoryFilter),
    ItemPrototypeFilter(ItemPrototypeFilter),
    ItemPrototypeFlags(ItemPrototypeFlags),
    ItemPrototypeIdentification(ItemPrototypeIdentification),
    ItemStackDefinition(ItemStackDefinition),
    ItemStackIdentification(ItemStackIdentification),
    ItemStackLocation(ItemStackLocation),
    LocalisedString(LocalisedString),
    LogisticFilter(LogisticFilter),
    LogisticParameters(LogisticParameters),
    Loot(Loot),
    LuaEntityClonedEventFilter(LuaEntityClonedEventFilter),
    LuaEntityDamagedEventFilter(LuaEntityDamagedEventFilter),
    LuaEntityDeconstructionCancelledEventFilter(LuaEntityDeconstructionCancelledEventFilter),
    LuaEntityDiedEventFilter(LuaEntityDiedEventFilter),
    LuaEntityMarkedForDeconstructionEventFilter(LuaEntityMarkedForDeconstructionEventFilter),
    LuaEntityMarkedForUpgradeEventFilter(LuaEntityMarkedForUpgradeEventFilter),
    LuaPlayerBuiltEntityEventFilter(LuaPlayerBuiltEntityEventFilter),
    LuaPlayerMinedEntityEventFilter(LuaPlayerMinedEntityEventFilter),
    LuaPlayerRepairedEntityEventFilter(LuaPlayerRepairedEntityEventFilter),
    LuaPostEntityDiedEventFilter(LuaPostEntityDiedEventFilter),
    LuaPreGhostDeconstructedEventFilter(LuaPreGhostDeconstructedEventFilter),
    LuaPreGhostUpgradedEventFilter(LuaPreGhostUpgradedEventFilter),
    LuaPrePlayerMinedEntityEventFilter(LuaPrePlayerMinedEntityEventFilter),
    LuaPreRobotMinedEntityEventFilter(LuaPreRobotMinedEntityEventFilter),
    LuaRobotBuiltEntityEventFilter(LuaRobotBuiltEntityEventFilter),
    LuaRobotMinedEntityEventFilter(LuaRobotMinedEntityEventFilter),
    LuaScriptRaisedBuiltEventFilter(LuaScriptRaisedBuiltEventFilter),
    LuaScriptRaisedDestroyEventFilter(LuaScriptRaisedDestroyEventFilter),
    LuaScriptRaisedReviveEventFilter(LuaScriptRaisedReviveEventFilter),
    LuaScriptRaisedTeleportedEventFilter(LuaScriptRaisedTeleportedEventFilter),
    LuaSectorScannedEventFilter(LuaSectorScannedEventFilter),
    LuaUpgradeCancelledEventFilter(LuaUpgradeCancelledEventFilter),
    MapAndDifficultySettings(MapAndDifficultySettings),
    MapExchangeStringData(MapExchangeStringData),
    MapGenPreset(MapGenPreset),
    MapGenSettings(MapGenSettings),
    MapGenSize(MapGenSize),
    MapPosition(MapPosition),
    MapSettings(MapSettings),
    MapViewSettings(MapViewSettings),
    ModChangeData(ModChangeData),
    ModSetting(ModSetting),
    ModSettingPrototypeFilter(ModSettingPrototypeFilter),
    ModuleEffectValue(ModuleEffectValue),
    ModuleEffects(ModuleEffects),
    MouseButtonFlags(MouseButtonFlags),
    NoiseExpression(NoiseExpression),
    NthTickEventData(NthTickEventData),
    Offer(Offer),
    OldTileAndPosition(OldTileAndPosition),
    PathFinderMapSettings(PathFinderMapSettings),
    PathfinderFlags(PathfinderFlags),
    PathfinderWaypoint(PathfinderWaypoint),
    PlaceAsTileResult(PlaceAsTileResult),
    PlayerIdentification(PlayerIdentification),
    PollutionMapSettings(PollutionMapSettings),
    Product(Product),
    ProgrammableSpeakerAlertParameters(ProgrammableSpeakerAlertParameters),
    ProgrammableSpeakerCircuitParameters(ProgrammableSpeakerCircuitParameters),
    ProgrammableSpeakerInstrument(ProgrammableSpeakerInstrument),
    ProgrammableSpeakerParameters(ProgrammableSpeakerParameters),
    PrototypeFilter(PrototypeFilter),
    PrototypeHistory(PrototypeHistory),
    RealOrientation(RealOrientation),
    RecipePrototypeFilter(RecipePrototypeFilter),
    RenderLayer(RenderLayer),
    Resistance(Resistance),
    RidingState(RidingState),
    ScriptArea(ScriptArea),
    ScriptPosition(ScriptPosition),
    ScriptRenderTarget(ScriptRenderTarget),
    ScriptRenderVertexTarget(ScriptRenderVertexTarget),
    SelectedPrototypeData(SelectedPrototypeData),
    SelectionModeFlags(SelectionModeFlags),
    Signal(Signal),
    SignalID(SignalID),
    SimpleItemStack(SimpleItemStack),
    SmokeSource(SmokeSource),
    SoundPath(SoundPath),
    SoundType(SoundType),
    SpawnPointDefinition(SpawnPointDefinition),
    SpritePath(SpritePath),
    SteeringMapSetting(SteeringMapSetting),
    SteeringMapSettings(SteeringMapSettings),
    SurfaceIdentification(SurfaceIdentification),
    TabAndContent(TabAndContent),
    Tags(Tags),
    TechnologyIdentification(TechnologyIdentification),
    TechnologyModifier(TechnologyModifier),
    TechnologyPrototypeFilter(TechnologyPrototypeFilter),
    Tile(Tile),
    TilePosition(TilePosition),
    TilePrototypeFilter(TilePrototypeFilter),
    TrainSchedule(TrainSchedule),
    TrainScheduleRecord(TrainScheduleRecord),
    TriggerDelivery(TriggerDelivery),
    TriggerEffectItem(TriggerEffectItem),
    TriggerItem(TriggerItem),
    TriggerTargetMask(TriggerTargetMask),
    UnitGroupMapSettings(UnitGroupMapSettings),
    UnitSpawnDefinition(UnitSpawnDefinition),
    UpgradeFilter(UpgradeFilter),
    Vector(Vector),
    VehicleAutomaticTargetingParameters(VehicleAutomaticTargetingParameters),
    WaitCondition(WaitCondition),
    WireConnectionDefinition(WireConnectionDefinition),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "serde_tag")]
pub enum Event {
    CustomInputEvent(CustomInputEvent),
    OnAiCommandCompleted(OnAiCommandCompleted),
    OnAreaCloned(OnAreaCloned),
    OnBiterBaseBuilt(OnBiterBaseBuilt),
    OnBrushCloned(OnBrushCloned),
    OnBuildBaseArrived(OnBuildBaseArrived),
    OnBuiltEntity(OnBuiltEntity),
    OnCancelledDeconstruction(OnCancelledDeconstruction),
    OnCancelledUpgrade(OnCancelledUpgrade),
    OnCharacterCorpseExpired(OnCharacterCorpseExpired),
    OnChartTagAdded(OnChartTagAdded),
    OnChartTagModified(OnChartTagModified),
    OnChartTagRemoved(OnChartTagRemoved),
    OnChunkCharted(OnChunkCharted),
    OnChunkDeleted(OnChunkDeleted),
    OnChunkGenerated(OnChunkGenerated),
    OnCombatRobotExpired(OnCombatRobotExpired),
    OnConsoleChat(OnConsoleChat),
    OnConsoleCommand(OnConsoleCommand),
    OnCutsceneCancelled(OnCutsceneCancelled),
    OnCutsceneWaypointReached(OnCutsceneWaypointReached),
    OnDifficultySettingsChanged(OnDifficultySettingsChanged),
    OnEntityCloned(OnEntityCloned),
    OnEntityDamaged(OnEntityDamaged),
    OnEntityDestroyed(OnEntityDestroyed),
    OnEntityDied(OnEntityDied),
    OnEntityLogisticSlotChanged(OnEntityLogisticSlotChanged),
    OnEntityRenamed(OnEntityRenamed),
    OnEntitySettingsPasted(OnEntitySettingsPasted),
    OnEntitySpawned(OnEntitySpawned),
    OnEquipmentInserted(OnEquipmentInserted),
    OnEquipmentRemoved(OnEquipmentRemoved),
    OnForceCeaseFireChanged(OnForceCeaseFireChanged),
    OnForceCreated(OnForceCreated),
    OnForceFriendsChanged(OnForceFriendsChanged),
    OnForceReset(OnForceReset),
    OnForcesMerged(OnForcesMerged),
    OnForcesMerging(OnForcesMerging),
    OnGameCreatedFromScenario(OnGameCreatedFromScenario),
    OnGuiCheckedStateChanged(OnGuiCheckedStateChanged),
    OnGuiClick(OnGuiClick),
    OnGuiClosed(OnGuiClosed),
    OnGuiConfirmed(OnGuiConfirmed),
    OnGuiElemChanged(OnGuiElemChanged),
    OnGuiHover(OnGuiHover),
    OnGuiLeave(OnGuiLeave),
    OnGuiLocationChanged(OnGuiLocationChanged),
    OnGuiOpened(OnGuiOpened),
    OnGuiSelectedTabChanged(OnGuiSelectedTabChanged),
    OnGuiSelectionStateChanged(OnGuiSelectionStateChanged),
    OnGuiSwitchStateChanged(OnGuiSwitchStateChanged),
    OnGuiTextChanged(OnGuiTextChanged),
    OnGuiValueChanged(OnGuiValueChanged),
    OnLandMineArmed(OnLandMineArmed),
    OnLuaShortcut(OnLuaShortcut),
    OnMarkedForDeconstruction(OnMarkedForDeconstruction),
    OnMarkedForUpgrade(OnMarkedForUpgrade),
    OnMarketItemPurchased(OnMarketItemPurchased),
    OnModItemOpened(OnModItemOpened),
    OnPermissionGroupAdded(OnPermissionGroupAdded),
    OnPermissionGroupDeleted(OnPermissionGroupDeleted),
    OnPermissionGroupEdited(OnPermissionGroupEdited),
    OnPermissionStringImported(OnPermissionStringImported),
    OnPickedUpItem(OnPickedUpItem),
    OnPlayerAltReverseSelectedArea(OnPlayerAltReverseSelectedArea),
    OnPlayerAltSelectedArea(OnPlayerAltSelectedArea),
    OnPlayerAmmoInventoryChanged(OnPlayerAmmoInventoryChanged),
    OnPlayerArmorInventoryChanged(OnPlayerArmorInventoryChanged),
    OnPlayerBanned(OnPlayerBanned),
    OnPlayerBuiltTile(OnPlayerBuiltTile),
    OnPlayerCancelledCrafting(OnPlayerCancelledCrafting),
    OnPlayerChangedForce(OnPlayerChangedForce),
    OnPlayerChangedPosition(OnPlayerChangedPosition),
    OnPlayerChangedSurface(OnPlayerChangedSurface),
    OnPlayerCheatModeDisabled(OnPlayerCheatModeDisabled),
    OnPlayerCheatModeEnabled(OnPlayerCheatModeEnabled),
    OnPlayerClickedGpsTag(OnPlayerClickedGpsTag),
    OnPlayerConfiguredBlueprint(OnPlayerConfiguredBlueprint),
    OnPlayerConfiguredSpiderRemote(OnPlayerConfiguredSpiderRemote),
    OnPlayerCraftedItem(OnPlayerCraftedItem),
    OnPlayerCreated(OnPlayerCreated),
    OnPlayerCursorStackChanged(OnPlayerCursorStackChanged),
    OnPlayerDeconstructedArea(OnPlayerDeconstructedArea),
    OnPlayerDemoted(OnPlayerDemoted),
    OnPlayerDied(OnPlayerDied),
    OnPlayerDisplayResolutionChanged(OnPlayerDisplayResolutionChanged),
    OnPlayerDisplayScaleChanged(OnPlayerDisplayScaleChanged),
    OnPlayerDrivingChangedState(OnPlayerDrivingChangedState),
    OnPlayerDroppedItem(OnPlayerDroppedItem),
    OnPlayerFastTransferred(OnPlayerFastTransferred),
    OnPlayerFlushedFluid(OnPlayerFlushedFluid),
    OnPlayerGunInventoryChanged(OnPlayerGunInventoryChanged),
    OnPlayerJoinedGame(OnPlayerJoinedGame),
    OnPlayerKicked(OnPlayerKicked),
    OnPlayerLeftGame(OnPlayerLeftGame),
    OnPlayerMainInventoryChanged(OnPlayerMainInventoryChanged),
    OnPlayerMinedEntity(OnPlayerMinedEntity),
    OnPlayerMinedItem(OnPlayerMinedItem),
    OnPlayerMinedTile(OnPlayerMinedTile),
    OnPlayerMuted(OnPlayerMuted),
    OnPlayerPipette(OnPlayerPipette),
    OnPlayerPlacedEquipment(OnPlayerPlacedEquipment),
    OnPlayerPromoted(OnPlayerPromoted),
    OnPlayerRemoved(OnPlayerRemoved),
    OnPlayerRemovedEquipment(OnPlayerRemovedEquipment),
    OnPlayerRepairedEntity(OnPlayerRepairedEntity),
    OnPlayerRespawned(OnPlayerRespawned),
    OnPlayerReverseSelectedArea(OnPlayerReverseSelectedArea),
    OnPlayerRotatedEntity(OnPlayerRotatedEntity),
    OnPlayerSelectedArea(OnPlayerSelectedArea),
    OnPlayerSetQuickBarSlot(OnPlayerSetQuickBarSlot),
    OnPlayerSetupBlueprint(OnPlayerSetupBlueprint),
    OnPlayerToggledAltMode(OnPlayerToggledAltMode),
    OnPlayerToggledMapEditor(OnPlayerToggledMapEditor),
    OnPlayerTrashInventoryChanged(OnPlayerTrashInventoryChanged),
    OnPlayerUnbanned(OnPlayerUnbanned),
    OnPlayerUnmuted(OnPlayerUnmuted),
    OnPlayerUsedCapsule(OnPlayerUsedCapsule),
    OnPlayerUsedSpiderRemote(OnPlayerUsedSpiderRemote),
    OnPostEntityDied(OnPostEntityDied),
    OnPreBuild(OnPreBuild),
    OnPreChunkDeleted(OnPreChunkDeleted),
    OnPreEntitySettingsPasted(OnPreEntitySettingsPasted),
    OnPreGhostDeconstructed(OnPreGhostDeconstructed),
    OnPreGhostUpgraded(OnPreGhostUpgraded),
    OnPrePermissionGroupDeleted(OnPrePermissionGroupDeleted),
    OnPrePermissionStringImported(OnPrePermissionStringImported),
    OnPrePlayerCraftedItem(OnPrePlayerCraftedItem),
    OnPrePlayerDied(OnPrePlayerDied),
    OnPrePlayerLeftGame(OnPrePlayerLeftGame),
    OnPrePlayerMinedItem(OnPrePlayerMinedItem),
    OnPrePlayerRemoved(OnPrePlayerRemoved),
    OnPrePlayerToggledMapEditor(OnPrePlayerToggledMapEditor),
    OnPreRobotExplodedCliff(OnPreRobotExplodedCliff),
    OnPreScriptInventoryResized(OnPreScriptInventoryResized),
    OnPreSurfaceCleared(OnPreSurfaceCleared),
    OnPreSurfaceDeleted(OnPreSurfaceDeleted),
    OnResearchCancelled(OnResearchCancelled),
    OnResearchFinished(OnResearchFinished),
    OnResearchReversed(OnResearchReversed),
    OnResearchStarted(OnResearchStarted),
    OnResourceDepleted(OnResourceDepleted),
    OnRobotBuiltEntity(OnRobotBuiltEntity),
    OnRobotBuiltTile(OnRobotBuiltTile),
    OnRobotExplodedCliff(OnRobotExplodedCliff),
    OnRobotMined(OnRobotMined),
    OnRobotMinedEntity(OnRobotMinedEntity),
    OnRobotMinedTile(OnRobotMinedTile),
    OnRobotPreMined(OnRobotPreMined),
    OnRocketLaunchOrdered(OnRocketLaunchOrdered),
    OnRocketLaunched(OnRocketLaunched),
    OnRuntimeModSettingChanged(OnRuntimeModSettingChanged),
    OnScriptInventoryResized(OnScriptInventoryResized),
    OnScriptPathRequestFinished(OnScriptPathRequestFinished),
    OnScriptTriggerEffect(OnScriptTriggerEffect),
    OnSectorScanned(OnSectorScanned),
    OnSelectedEntityChanged(OnSelectedEntityChanged),
    OnSpiderCommandCompleted(OnSpiderCommandCompleted),
    OnStringTranslated(OnStringTranslated),
    OnSurfaceCleared(OnSurfaceCleared),
    OnSurfaceCreated(OnSurfaceCreated),
    OnSurfaceDeleted(OnSurfaceDeleted),
    OnSurfaceImported(OnSurfaceImported),
    OnSurfaceRenamed(OnSurfaceRenamed),
    OnTechnologyEffectsReset(OnTechnologyEffectsReset),
    OnTick(OnTick),
    OnTrainChangedState(OnTrainChangedState),
    OnTrainCreated(OnTrainCreated),
    OnTrainScheduleChanged(OnTrainScheduleChanged),
    OnTriggerCreatedEntity(OnTriggerCreatedEntity),
    OnTriggerFiredArtillery(OnTriggerFiredArtillery),
    OnUnitAddedToGroup(OnUnitAddedToGroup),
    OnUnitGroupCreated(OnUnitGroupCreated),
    OnUnitGroupFinishedGathering(OnUnitGroupFinishedGathering),
    OnUnitRemovedFromGroup(OnUnitRemovedFromGroup),
    OnWorkerRobotExpired(OnWorkerRobotExpired),
    ScriptRaisedBuilt(ScriptRaisedBuilt),
    ScriptRaisedDestroy(ScriptRaisedDestroy),
    ScriptRaisedRevive(ScriptRaisedRevive),
    ScriptRaisedSetTiles(ScriptRaisedSetTiles),
    ScriptRaisedTeleported(ScriptRaisedTeleported),
}
