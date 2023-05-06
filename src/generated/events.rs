use std::collections::HashMap;

use super::classes::*;
use super::concepts::*;
use super::defines::*;

pub struct CustomInputEvent {
    pub cursor_position: MapPosition,
    pub input_name: String,
    pub name: Events,
    pub player_index: u32,
    pub selected_prototype: Option<SelectedPrototypeData>,
    pub tick: u32,
}

pub struct OnAiCommandCompleted {
    pub name: Events,
    pub result: BehaviorResult,
    pub tick: u32,
    pub unit_number: u32,
    pub was_distracted: bool,
}

pub struct OnAreaCloned {
    pub clear_destination_decoratives: bool,
    pub clear_destination_entities: bool,
    pub clone_decoratives: bool,
    pub clone_entities: bool,
    pub clone_tiles: bool,
    pub destination_area: BoundingBox,
    pub destination_force: Option<LuaForce>,
    pub destination_surface: LuaSurface,
    pub name: Events,
    pub source_area: BoundingBox,
    pub source_surface: LuaSurface,
    pub tick: u32,
}

pub struct OnBiterBaseBuilt {
    pub entity: LuaEntity,
    pub name: Events,
    pub tick: u32,
}

pub struct OnBrushCloned {
    pub clear_destination_decoratives: bool,
    pub clear_destination_entities: bool,
    pub clone_decoratives: bool,
    pub clone_entities: bool,
    pub clone_tiles: bool,
    pub destination_force: Option<LuaForce>,
    pub destination_offset: TilePosition,
    pub destination_surface: LuaSurface,
    pub name: Events,
    pub source_offset: TilePosition,
    pub source_positions: Vec<TilePosition>,
    pub source_surface: LuaSurface,
    pub tick: u32,
}

pub struct OnBuildBaseArrived {
    pub group: Option<LuaUnitGroup>,
    pub name: Events,
    pub tick: u32,
    pub unit: Option<LuaEntity>,
}

pub struct OnBuiltEntity {
    pub created_entity: LuaEntity,
    pub item: Option<LuaItemPrototype>,
    pub name: Events,
    pub player_index: u32,
    pub stack: LuaItemStack,
    pub tags: Option<Tags>,
    pub tick: u32,
}

pub struct OnCancelledDeconstruction {
    pub entity: LuaEntity,
    pub name: Events,
    pub player_index: Option<u32>,
    pub tick: u32,
}

pub struct OnCancelledUpgrade {
    pub direction: Option<Direction>,
    pub entity: LuaEntity,
    pub name: Events,
    pub player_index: Option<u32>,
    pub target: LuaEntityPrototype,
    pub tick: u32,
}

pub struct OnCharacterCorpseExpired {
    pub corpse: LuaEntity,
    pub name: Events,
    pub tick: u32,
}

pub struct OnChartTagAdded {
    pub force: LuaForce,
    pub name: Events,
    pub player_index: Option<u32>,
    pub tag: LuaCustomChartTag,
    pub tick: u32,
}

pub struct OnChartTagModified {
    pub force: LuaForce,
    pub name: Events,
    pub old_icon: Option<SignalID>,
    pub old_player: Option<u32>,
    pub old_text: String,
    pub player_index: Option<u32>,
    pub tag: LuaCustomChartTag,
    pub tick: u32,
}

pub struct OnChartTagRemoved {
    pub force: LuaForce,
    pub name: Events,
    pub player_index: Option<u32>,
    pub tag: LuaCustomChartTag,
    pub tick: u32,
}

pub struct OnChunkCharted {
    pub area: BoundingBox,
    pub force: LuaForce,
    pub name: Events,
    pub position: ChunkPosition,
    pub surface_index: u32,
    pub tick: u32,
}

pub struct OnChunkDeleted {
    pub name: Events,
    pub positions: Vec<ChunkPosition>,
    pub surface_index: u32,
    pub tick: u32,
}

pub struct OnChunkGenerated {
    pub area: BoundingBox,
    pub name: Events,
    pub position: ChunkPosition,
    pub surface: LuaSurface,
    pub tick: u32,
}

pub struct OnCombatRobotExpired {
    pub name: Events,
    pub owner: Option<LuaEntity>,
    pub robot: LuaEntity,
    pub tick: u32,
}

pub struct OnConsoleChat {
    pub message: String,
    pub name: Events,
    pub player_index: Option<u32>,
    pub tick: u32,
}

pub struct OnConsoleCommand {
    pub command: String,
    pub name: Events,
    pub parameters: String,
    pub player_index: Option<u32>,
    pub tick: u32,
}

pub struct OnCutsceneCancelled {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnCutsceneWaypointReached {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
    pub waypoint_index: u32,
}

pub struct OnDifficultySettingsChanged {
    pub name: Events,
    pub old_recipe_difficulty: u32,
    pub old_technology_difficulty: u32,
    pub tick: u32,
}

pub struct OnEntityCloned {
    pub destination: LuaEntity,
    pub name: Events,
    pub source: LuaEntity,
    pub tick: u32,
}

pub struct OnEntityDamaged {
    pub cause: Option<LuaEntity>,
    pub damage_type: LuaDamagePrototype,
    pub entity: LuaEntity,
    pub final_damage_amount: f32,
    pub final_health: f32,
    pub force: Option<LuaForce>,
    pub name: Events,
    pub original_damage_amount: f32,
    pub tick: u32,
}

pub struct OnEntityDestroyed {
    pub name: Events,
    pub registration_number: u64,
    pub tick: u32,
    pub unit_number: Option<u32>,
}

pub struct OnEntityDied {
    pub cause: Option<LuaEntity>,
    pub damage_type: Option<LuaDamagePrototype>,
    pub entity: LuaEntity,
    pub force: Option<LuaForce>,
    pub loot: LuaInventory,
    pub name: Events,
    pub tick: u32,
}

pub struct OnEntityLogisticSlotChanged {
    pub entity: LuaEntity,
    pub name: Events,
    pub player_index: Option<u32>,
    pub slot_index: u32,
    pub tick: u32,
}

pub struct OnEntityRenamed {
    pub by_script: bool,
    pub entity: LuaEntity,
    pub name: Events,
    pub old_name: String,
    pub player_index: Option<u32>,
    pub tick: u32,
}

pub struct OnEntitySettingsPasted {
    pub destination: LuaEntity,
    pub name: Events,
    pub player_index: u32,
    pub source: LuaEntity,
    pub tick: u32,
}

pub struct OnEntitySpawned {
    pub entity: LuaEntity,
    pub name: Events,
    pub spawner: LuaEntity,
    pub tick: u32,
}

pub struct OnEquipmentInserted {
    pub equipment: LuaEquipment,
    pub grid: LuaEquipmentGrid,
    pub name: Events,
    pub tick: u32,
}

pub struct OnEquipmentRemoved {
    pub count: u32,
    pub equipment: String,
    pub grid: LuaEquipmentGrid,
    pub name: Events,
    pub tick: u32,
}

pub struct OnForceCeaseFireChanged {
    pub added: bool,
    pub force: LuaForce,
    pub name: Events,
    pub other_force: LuaForce,
    pub tick: u32,
}

pub struct OnForceCreated {
    pub force: LuaForce,
    pub name: Events,
    pub tick: u32,
}

pub struct OnForceFriendsChanged {
    pub added: bool,
    pub force: LuaForce,
    pub name: Events,
    pub other_force: LuaForce,
    pub tick: u32,
}

pub struct OnForceReset {
    pub force: LuaForce,
    pub name: Events,
    pub tick: u32,
}

pub struct OnForcesMerged {
    pub destination: LuaForce,
    pub name: Events,
    pub source_index: u32,
    pub source_name: String,
    pub tick: u32,
}

pub struct OnForcesMerging {
    pub destination: LuaForce,
    pub name: Events,
    pub source: LuaForce,
    pub tick: u32,
}

pub struct OnGameCreatedFromScenario {
    pub name: Events,
    pub tick: u32,
}

pub struct OnGuiCheckedStateChanged {
    pub element: LuaGuiElement,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnGuiClick {
    pub alt: bool,
    pub button: MouseButtonType,
    pub control: bool,
    pub element: LuaGuiElement,
    pub name: Events,
    pub player_index: u32,
    pub shift: bool,
    pub tick: u32,
}

pub struct OnGuiClosed {
    pub element: Option<LuaGuiElement>,
    pub entity: Option<LuaEntity>,
    pub equipment: Option<LuaEquipment>,
    pub gui_type: GuiType,
    pub inventory: Option<LuaInventory>,
    pub item: Option<LuaItemStack>,
    pub name: Events,
    pub other_player: Option<LuaPlayer>,
    pub player_index: u32,
    pub technology: Option<LuaTechnology>,
    pub tick: u32,
    pub tile_position: Option<TilePosition>,
}

pub struct OnGuiConfirmed {
    pub alt: bool,
    pub control: bool,
    pub element: LuaGuiElement,
    pub name: Events,
    pub player_index: u32,
    pub shift: bool,
    pub tick: u32,
}

pub struct OnGuiElemChanged {
    pub element: LuaGuiElement,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnGuiHover {
    pub element: LuaGuiElement,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnGuiLeave {
    pub element: LuaGuiElement,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnGuiLocationChanged {
    pub element: LuaGuiElement,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnGuiOpened {
    pub element: Option<LuaGuiElement>,
    pub entity: Option<LuaEntity>,
    pub equipment: Option<LuaEquipment>,
    pub gui_type: GuiType,
    pub inventory: Option<LuaInventory>,
    pub item: Option<LuaItemStack>,
    pub name: Events,
    pub other_player: Option<LuaPlayer>,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnGuiSelectedTabChanged {
    pub element: LuaGuiElement,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnGuiSelectionStateChanged {
    pub element: LuaGuiElement,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnGuiSwitchStateChanged {
    pub element: LuaGuiElement,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnGuiTextChanged {
    pub element: LuaGuiElement,
    pub name: Events,
    pub player_index: u32,
    pub text: String,
    pub tick: u32,
}

pub struct OnGuiValueChanged {
    pub element: LuaGuiElement,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnLandMineArmed {
    pub mine: LuaEntity,
    pub name: Events,
    pub tick: u32,
}

pub struct OnLuaShortcut {
    pub name: Events,
    pub player_index: u32,
    pub prototype_name: String,
    pub tick: u32,
}

pub struct OnMarkedForDeconstruction {
    pub entity: LuaEntity,
    pub name: Events,
    pub player_index: Option<u32>,
    pub tick: u32,
}

pub struct OnMarkedForUpgrade {
    pub direction: Option<Direction>,
    pub entity: LuaEntity,
    pub name: Events,
    pub player_index: Option<u32>,
    pub target: LuaEntityPrototype,
    pub tick: u32,
}

pub struct OnMarketItemPurchased {
    pub count: u32,
    pub market: LuaEntity,
    pub name: Events,
    pub offer_index: u32,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnModItemOpened {
    pub item: LuaItemPrototype,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPermissionGroupAdded {
    pub group: LuaPermissionGroup,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPermissionGroupDeleted {
    pub group_name: String,
    pub id: u32,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPermissionGroupEdited {
    pub action: InputAction,
    pub group: LuaPermissionGroup,
    pub name: Events,
    pub new_name: String,
    pub old_name: String,
    pub other_player_index: u32,
    pub player_index: u32,
    pub tick: u32,
    pub typ: String,
}

pub struct OnPermissionStringImported {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPickedUpItem {
    pub item_stack: SimpleItemStack,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerAltReverseSelectedArea {
    pub area: BoundingBox,
    pub entities: Vec<LuaEntity>,
    pub item: String,
    pub name: Events,
    pub player_index: u32,
    pub surface: LuaSurface,
    pub tick: u32,
    pub tiles: Vec<LuaTile>,
}

pub struct OnPlayerAltSelectedArea {
    pub area: BoundingBox,
    pub entities: Vec<LuaEntity>,
    pub item: String,
    pub name: Events,
    pub player_index: u32,
    pub surface: LuaSurface,
    pub tick: u32,
    pub tiles: Vec<LuaTile>,
}

pub struct OnPlayerAmmoInventoryChanged {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerArmorInventoryChanged {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerBanned {
    pub by_player: Option<u32>,
    pub name: Events,
    pub player_index: Option<u32>,
    pub player_name: String,
    pub reason: Option<String>,
    pub tick: u32,
}

pub struct OnPlayerBuiltTile {
    pub item: Option<LuaItemPrototype>,
    pub name: Events,
    pub player_index: u32,
    pub stack: Option<LuaItemStack>,
    pub surface_index: u32,
    pub tick: u32,
    pub tile: LuaTilePrototype,
    pub tiles: Vec<OldTileAndPosition>,
}

pub struct OnPlayerCancelledCrafting {
    pub cancel_count: u32,
    pub items: LuaInventory,
    pub name: Events,
    pub player_index: u32,
    pub recipe: LuaRecipe,
    pub tick: u32,
}

pub struct OnPlayerChangedForce {
    pub force: LuaForce,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerChangedPosition {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerChangedSurface {
    pub name: Events,
    pub player_index: u32,
    pub surface_index: u32,
    pub tick: u32,
}

pub struct OnPlayerCheatModeDisabled {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerCheatModeEnabled {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerClickedGpsTag {
    pub name: Events,
    pub player_index: u32,
    pub position: MapPosition,
    pub surface: String,
    pub tick: u32,
}

pub struct OnPlayerConfiguredBlueprint {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerConfiguredSpiderRemote {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
    pub vehicle: LuaEntity,
}

pub struct OnPlayerCraftedItem {
    pub item_stack: LuaItemStack,
    pub name: Events,
    pub player_index: u32,
    pub recipe: LuaRecipe,
    pub tick: u32,
}

pub struct OnPlayerCreated {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerCursorStackChanged {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerDeconstructedArea {
    pub alt: bool,
    pub area: BoundingBox,
    pub item: String,
    pub name: Events,
    pub player_index: u32,
    pub surface: LuaSurface,
    pub tick: u32,
}

pub struct OnPlayerDemoted {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerDied {
    pub cause: Option<LuaEntity>,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerDisplayResolutionChanged {
    pub name: Events,
    pub old_resolution: DisplayResolution,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerDisplayScaleChanged {
    pub name: Events,
    pub old_scale: f64,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerDrivingChangedState {
    pub entity: Option<LuaEntity>,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerDroppedItem {
    pub entity: LuaEntity,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerFastTransferred {
    pub entity: LuaEntity,
    pub from_player: bool,
    pub is_split: bool,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerFlushedFluid {
    pub amount: f64,
    pub entity: LuaEntity,
    pub fluid: String,
    pub name: Events,
    pub only_this_entity: bool,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerGunInventoryChanged {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerJoinedGame {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerKicked {
    pub by_player: Option<u32>,
    pub name: Events,
    pub player_index: u32,
    pub reason: Option<String>,
    pub tick: u32,
}

pub struct OnPlayerLeftGame {
    pub name: Events,
    pub player_index: u32,
    pub reason: DisconnectReason,
    pub tick: u32,
}

pub struct OnPlayerMainInventoryChanged {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerMinedEntity {
    pub buffer: LuaInventory,
    pub entity: LuaEntity,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerMinedItem {
    pub item_stack: SimpleItemStack,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerMinedTile {
    pub name: Events,
    pub player_index: u32,
    pub surface_index: u32,
    pub tick: u32,
    pub tiles: Vec<OldTileAndPosition>,
}

pub struct OnPlayerMuted {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerPipette {
    pub item: LuaItemPrototype,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
    pub used_cheat_mode: bool,
}

pub struct OnPlayerPlacedEquipment {
    pub equipment: LuaEquipment,
    pub grid: LuaEquipmentGrid,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerPromoted {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerRemoved {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerRemovedEquipment {
    pub count: u32,
    pub equipment: String,
    pub grid: LuaEquipmentGrid,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerRepairedEntity {
    pub entity: LuaEntity,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerRespawned {
    pub name: Events,
    pub player_index: u32,
    pub player_port: Option<LuaEntity>,
    pub tick: u32,
}

pub struct OnPlayerReverseSelectedArea {
    pub area: BoundingBox,
    pub entities: Vec<LuaEntity>,
    pub item: String,
    pub name: Events,
    pub player_index: u32,
    pub surface: LuaSurface,
    pub tick: u32,
    pub tiles: Vec<LuaTile>,
}

pub struct OnPlayerRotatedEntity {
    pub entity: LuaEntity,
    pub name: Events,
    pub player_index: u32,
    pub previous_direction: Direction,
    pub tick: u32,
}

pub struct OnPlayerSelectedArea {
    pub area: BoundingBox,
    pub entities: Vec<LuaEntity>,
    pub item: String,
    pub name: Events,
    pub player_index: u32,
    pub surface: LuaSurface,
    pub tick: u32,
    pub tiles: Vec<LuaTile>,
}

pub struct OnPlayerSetQuickBarSlot {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerSetupBlueprint {
    pub alt: bool,
    pub area: BoundingBox,
    pub item: String,
    pub mapping: HashMap<u32, LuaEntity>,
    pub name: Events,
    pub player_index: u32,
    pub surface: LuaSurface,
    pub tick: u32,
}

pub struct OnPlayerToggledAltMode {
    pub alt_mode: bool,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerToggledMapEditor {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerTrashInventoryChanged {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerUnbanned {
    pub by_player: Option<u32>,
    pub name: Events,
    pub player_index: Option<u32>,
    pub player_name: String,
    pub reason: Option<String>,
    pub tick: u32,
}

pub struct OnPlayerUnmuted {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPlayerUsedCapsule {
    pub item: LuaItemPrototype,
    pub name: Events,
    pub player_index: u32,
    pub position: MapPosition,
    pub tick: u32,
}

pub struct OnPlayerUsedSpiderRemote {
    pub name: Events,
    pub player_index: u32,
    pub position: MapPosition,
    pub success: bool,
    pub tick: u32,
    pub vehicle: LuaEntity,
}

pub struct OnPostEntityDied {
    pub corpses: Vec<LuaEntity>,
    pub damage_type: Option<LuaDamagePrototype>,
    pub force: Option<LuaForce>,
    pub ghost: Option<LuaEntity>,
    pub name: Events,
    pub position: MapPosition,
    pub prototype: LuaEntityPrototype,
    pub surface_index: u32,
    pub tick: u32,
    pub unit_number: Option<u32>,
}

pub struct OnPreBuild {
    pub created_by_moving: bool,
    pub direction: Direction,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
    pub name: Events,
    pub player_index: u32,
    pub position: MapPosition,
    pub shift_build: bool,
    pub tick: u32,
}

pub struct OnPreChunkDeleted {
    pub name: Events,
    pub positions: Vec<ChunkPosition>,
    pub surface_index: u32,
    pub tick: u32,
}

pub struct OnPreEntitySettingsPasted {
    pub destination: LuaEntity,
    pub name: Events,
    pub player_index: u32,
    pub source: LuaEntity,
    pub tick: u32,
}

pub struct OnPreGhostDeconstructed {
    pub ghost: LuaEntity,
    pub name: Events,
    pub player_index: Option<u32>,
    pub tick: u32,
}

pub struct OnPreGhostUpgraded {
    pub ghost: LuaEntity,
    pub name: Events,
    pub player_index: Option<u32>,
    pub target: LuaEntityPrototype,
    pub tick: u32,
}

pub struct OnPrePermissionGroupDeleted {
    pub group: LuaPermissionGroup,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPrePermissionStringImported {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPrePlayerCraftedItem {
    pub items: LuaInventory,
    pub name: Events,
    pub player_index: u32,
    pub queued_count: u32,
    pub recipe: LuaRecipe,
    pub tick: u32,
}

pub struct OnPrePlayerDied {
    pub cause: Option<LuaEntity>,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPrePlayerLeftGame {
    pub name: Events,
    pub player_index: u32,
    pub reason: DisconnectReason,
    pub tick: u32,
}

pub struct OnPrePlayerMinedItem {
    pub entity: LuaEntity,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPrePlayerRemoved {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPrePlayerToggledMapEditor {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnPreRobotExplodedCliff {
    pub cliff: LuaEntity,
    pub item: LuaItemPrototype,
    pub name: Events,
    pub robot: LuaEntity,
    pub tick: u32,
}

pub struct OnPreScriptInventoryResized {
    pub inventory: LuaInventory,
    pub mod_name: String,
    pub name: Events,
    pub new_size: u32,
    pub old_size: u32,
    pub player_index: Option<u32>,
    pub tick: u32,
}

pub struct OnPreSurfaceCleared {
    pub name: Events,
    pub surface_index: u32,
    pub tick: u32,
}

pub struct OnPreSurfaceDeleted {
    pub name: Events,
    pub surface_index: u32,
    pub tick: u32,
}

pub struct OnResearchCancelled {
    pub force: LuaForce,
    pub name: Events,
    pub research: HashMap<String, u32>,
    pub tick: u32,
}

pub struct OnResearchFinished {
    pub by_script: bool,
    pub name: Events,
    pub research: LuaTechnology,
    pub tick: u32,
}

pub struct OnResearchReversed {
    pub by_script: bool,
    pub name: Events,
    pub research: LuaTechnology,
    pub tick: u32,
}

pub struct OnResearchStarted {
    pub last_research: Option<LuaTechnology>,
    pub name: Events,
    pub research: LuaTechnology,
    pub tick: u32,
}

pub struct OnResourceDepleted {
    pub entity: LuaEntity,
    pub name: Events,
    pub tick: u32,
}

pub struct OnRobotBuiltEntity {
    pub created_entity: LuaEntity,
    pub name: Events,
    pub robot: LuaEntity,
    pub stack: LuaItemStack,
    pub tags: Option<Tags>,
    pub tick: u32,
}

pub struct OnRobotBuiltTile {
    pub item: LuaItemPrototype,
    pub name: Events,
    pub robot: LuaEntity,
    pub stack: LuaItemStack,
    pub surface_index: u32,
    pub tick: u32,
    pub tile: LuaTilePrototype,
    pub tiles: Vec<OldTileAndPosition>,
}

pub struct OnRobotExplodedCliff {
    pub item: LuaItemPrototype,
    pub name: Events,
    pub robot: LuaEntity,
    pub tick: u32,
}

pub struct OnRobotMined {
    pub item_stack: SimpleItemStack,
    pub name: Events,
    pub robot: LuaEntity,
    pub tick: u32,
}

pub struct OnRobotMinedEntity {
    pub buffer: LuaInventory,
    pub entity: LuaEntity,
    pub name: Events,
    pub robot: LuaEntity,
    pub tick: u32,
}

pub struct OnRobotMinedTile {
    pub name: Events,
    pub robot: LuaEntity,
    pub surface_index: u32,
    pub tick: u32,
    pub tiles: Vec<OldTileAndPosition>,
}

pub struct OnRobotPreMined {
    pub entity: LuaEntity,
    pub name: Events,
    pub robot: LuaEntity,
    pub tick: u32,
}

pub struct OnRocketLaunchOrdered {
    pub name: Events,
    pub player_index: Option<u32>,
    pub rocket: LuaEntity,
    pub rocket_silo: LuaEntity,
    pub tick: u32,
}

pub struct OnRocketLaunched {
    pub name: Events,
    pub player_index: Option<u32>,
    pub rocket: LuaEntity,
    pub rocket_silo: Option<LuaEntity>,
    pub tick: u32,
}

pub struct OnRuntimeModSettingChanged {
    pub name: Events,
    pub player_index: Option<u32>,
    pub setting: String,
    pub setting_type: String,
    pub tick: u32,
}

pub struct OnScriptInventoryResized {
    pub inventory: LuaInventory,
    pub mod_name: String,
    pub name: Events,
    pub new_size: u32,
    pub old_size: u32,
    pub overflow_inventory: LuaInventory,
    pub player_index: Option<u32>,
    pub tick: u32,
}

pub struct OnScriptPathRequestFinished {
    pub id: u32,
    pub name: Events,
    pub path: Option<Vec<PathfinderWaypoint>>,
    pub tick: u32,
    pub try_again_later: bool,
}

pub struct OnScriptTriggerEffect {
    pub effect_id: String,
    pub name: Events,
    pub source_entity: Option<LuaEntity>,
    pub source_position: Option<MapPosition>,
    pub surface_index: u32,
    pub target_entity: Option<LuaEntity>,
    pub target_position: Option<MapPosition>,
    pub tick: u32,
}

pub struct OnSectorScanned {
    pub area: BoundingBox,
    pub chunk_position: ChunkPosition,
    pub name: Events,
    pub radar: LuaEntity,
    pub tick: u32,
}

pub struct OnSelectedEntityChanged {
    pub last_entity: Option<LuaEntity>,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

pub struct OnSpiderCommandCompleted {
    pub name: Events,
    pub tick: u32,
    pub vehicle: LuaEntity,
}

pub struct OnStringTranslated {
    pub id: u32,
    pub localised_string: LocalisedString,
    pub name: Events,
    pub player_index: u32,
    pub result: String,
    pub tick: u32,
    pub translated: bool,
}

pub struct OnSurfaceCleared {
    pub name: Events,
    pub surface_index: u32,
    pub tick: u32,
}

pub struct OnSurfaceCreated {
    pub name: Events,
    pub surface_index: u32,
    pub tick: u32,
}

pub struct OnSurfaceDeleted {
    pub name: Events,
    pub surface_index: u32,
    pub tick: u32,
}

pub struct OnSurfaceImported {
    pub name: Events,
    pub original_name: String,
    pub surface_index: u32,
    pub tick: u32,
}

pub struct OnSurfaceRenamed {
    pub name: Events,
    pub new_name: String,
    pub old_name: String,
    pub surface_index: u32,
    pub tick: u32,
}

pub struct OnTechnologyEffectsReset {
    pub force: LuaForce,
    pub name: Events,
    pub tick: u32,
}

pub struct OnTick {
    pub name: Events,
    pub tick: u32,
}

pub struct OnTrainChangedState {
    pub name: Events,
    pub old_state: TrainState,
    pub tick: u32,
    pub train: LuaTrain,
}

pub struct OnTrainCreated {
    pub name: Events,
    pub old_train_id_1: Option<u32>,
    pub old_train_id_2: Option<u32>,
    pub tick: u32,
    pub train: LuaTrain,
}

pub struct OnTrainScheduleChanged {
    pub name: Events,
    pub player_index: Option<u32>,
    pub tick: u32,
    pub train: LuaTrain,
}

pub struct OnTriggerCreatedEntity {
    pub entity: LuaEntity,
    pub name: Events,
    pub source: Option<LuaEntity>,
    pub tick: u32,
}

pub struct OnTriggerFiredArtillery {
    pub entity: LuaEntity,
    pub name: Events,
    pub source: Option<LuaEntity>,
    pub tick: u32,
}

pub struct OnUnitAddedToGroup {
    pub group: LuaUnitGroup,
    pub name: Events,
    pub tick: u32,
    pub unit: LuaEntity,
}

pub struct OnUnitGroupCreated {
    pub group: LuaUnitGroup,
    pub name: Events,
    pub tick: u32,
}

pub struct OnUnitGroupFinishedGathering {
    pub group: LuaUnitGroup,
    pub name: Events,
    pub tick: u32,
}

pub struct OnUnitRemovedFromGroup {
    pub group: LuaUnitGroup,
    pub name: Events,
    pub tick: u32,
    pub unit: LuaEntity,
}

pub struct OnWorkerRobotExpired {
    pub name: Events,
    pub robot: LuaEntity,
    pub tick: u32,
}

pub struct ScriptRaisedBuilt {
    pub entity: LuaEntity,
    pub name: Events,
    pub tick: u32,
}

pub struct ScriptRaisedDestroy {
    pub entity: LuaEntity,
    pub name: Events,
    pub tick: u32,
}

pub struct ScriptRaisedRevive {
    pub entity: LuaEntity,
    pub name: Events,
    pub tags: Option<Tags>,
    pub tick: u32,
}

pub struct ScriptRaisedSetTiles {
    pub name: Events,
    pub surface_index: u32,
    pub tick: u32,
    pub tiles: Vec<Tile>,
}

pub struct ScriptRaisedTeleported {
    pub entity: LuaEntity,
    pub name: Events,
    pub old_position: MapPosition,
    pub old_surface_index: u8,
    pub tick: u32,
}
