use std::collections::HashMap;

use super::classes::*;
use super::concepts::*;
use super::defines::*;

/// Called when a [CustomInput](https://wiki.factorio.com/Prototype/CustomInput) is activated.
pub struct CustomInputEvent {
    pub cursor_position: MapPosition,
    pub input_name: String,
    pub name: Events,
    pub player_index: u32,
    pub selected_prototype: Option<SelectedPrototypeData>,
    pub tick: u32,
}

/// Called when a unit/group completes a command.
pub struct OnAiCommandCompleted {
    pub name: Events,
    pub result: BehaviorResult,
    pub tick: u32,
    pub unit_number: u32,
    pub was_distracted: bool,
}

/// Called when an area of the map is cloned.
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

/// Called when a biter migration builds a base.
pub struct OnBiterBaseBuilt {
    pub entity: LuaEntity,
    pub name: Events,
    pub tick: u32,
}

/// Called when a set of positions on the map is cloned.
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

/// Called when a [defines.command.build_base](defines.command.build_base) command reaches its destination, and before building starts.
pub struct OnBuildBaseArrived {
    pub group: Option<LuaUnitGroup>,
    pub name: Events,
    pub tick: u32,
    pub unit: Option<LuaEntity>,
}

/// Called when player builds something. Can be filtered using [LuaPlayerBuiltEntityEventFilter](LuaPlayerBuiltEntityEventFilter).
pub struct OnBuiltEntity {
    pub created_entity: LuaEntity,
    pub item: Option<LuaItemPrototype>,
    pub name: Events,
    pub player_index: u32,
    pub stack: LuaItemStack,
    pub tags: Option<Tags>,
    pub tick: u32,
}

/// Called when the deconstruction of an entity is canceled. Can be filtered using [LuaEntityDeconstructionCancelledEventFilter](LuaEntityDeconstructionCancelledEventFilter).
pub struct OnCancelledDeconstruction {
    pub entity: LuaEntity,
    pub name: Events,
    pub player_index: Option<u32>,
    pub tick: u32,
}

/// Called when the upgrade of an entity is canceled. Can be filtered using [LuaUpgradeCancelledEventFilter](LuaUpgradeCancelledEventFilter).
pub struct OnCancelledUpgrade {
    pub direction: Option<Direction>,
    pub entity: LuaEntity,
    pub name: Events,
    pub player_index: Option<u32>,
    pub target: LuaEntityPrototype,
    pub tick: u32,
}

/// Called when a character corpse expires due to timeout or all of the items being removed from it.
pub struct OnCharacterCorpseExpired {
    pub corpse: LuaEntity,
    pub name: Events,
    pub tick: u32,
}

/// Called when a chart tag is created.
pub struct OnChartTagAdded {
    pub force: LuaForce,
    pub name: Events,
    pub player_index: Option<u32>,
    pub tag: LuaCustomChartTag,
    pub tick: u32,
}

/// Called when a chart tag is modified by a player.
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

/// Called just before a chart tag is deleted.
pub struct OnChartTagRemoved {
    pub force: LuaForce,
    pub name: Events,
    pub player_index: Option<u32>,
    pub tag: LuaCustomChartTag,
    pub tick: u32,
}

/// Called when a chunk is charted or re-charted.
pub struct OnChunkCharted {
    pub area: BoundingBox,
    pub force: LuaForce,
    pub name: Events,
    pub position: ChunkPosition,
    pub surface_index: u32,
    pub tick: u32,
}

/// Called when one or more chunks are deleted using [LuaSurface::delete_chunk](LuaSurface::delete_chunk).
pub struct OnChunkDeleted {
    pub name: Events,
    pub positions: Vec<ChunkPosition>,
    pub surface_index: u32,
    pub tick: u32,
}

/// Called when a chunk is generated.
pub struct OnChunkGenerated {
    pub area: BoundingBox,
    pub name: Events,
    pub position: ChunkPosition,
    pub surface: LuaSurface,
    pub tick: u32,
}

/// Called when a combat robot expires through a lack of energy, or timeout.
pub struct OnCombatRobotExpired {
    pub name: Events,
    pub owner: Option<LuaEntity>,
    pub robot: LuaEntity,
    pub tick: u32,
}

/// Called when a message is sent to the in-game console, either by a player or through the server interface.
pub struct OnConsoleChat {
    pub message: String,
    pub name: Events,
    pub player_index: Option<u32>,
    pub tick: u32,
}

/// Called when someone enters a command-like message regardless of it being a valid command.
pub struct OnConsoleCommand {
    pub command: String,
    pub name: Events,
    pub parameters: String,
    pub player_index: Option<u32>,
    pub tick: u32,
}

/// Called when a cutscene is cancelled by the player or by script.
pub struct OnCutsceneCancelled {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when a cutscene is playing, each time it reaches a waypoint in that cutscene.
/// 
/// This refers to an index in the table previously passed to set_controller which started the cutscene.
pub struct OnCutsceneWaypointReached {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
    pub waypoint_index: u32,
}

/// Called when the map difficulty settings are changed.
pub struct OnDifficultySettingsChanged {
    pub name: Events,
    pub old_recipe_difficulty: u32,
    pub old_technology_difficulty: u32,
    pub tick: u32,
}

/// Called when an entity is cloned. Can be filtered for the source entity using [LuaEntityClonedEventFilter](LuaEntityClonedEventFilter).
pub struct OnEntityCloned {
    pub destination: LuaEntity,
    pub name: Events,
    pub source: LuaEntity,
    pub tick: u32,
}

/// Called when an entity is damaged. Can be filtered using [LuaEntityDamagedEventFilter](LuaEntityDamagedEventFilter).
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

/// Called after an entity is destroyed that has been registered with [LuaBootstrap::register_on_entity_destroyed](LuaBootstrap::register_on_entity_destroyed).
pub struct OnEntityDestroyed {
    pub name: Events,
    pub registration_number: u64,
    pub tick: u32,
    pub unit_number: Option<u32>,
}

/// Called when an entity dies. Can be filtered using [LuaEntityDiedEventFilter](LuaEntityDiedEventFilter).
pub struct OnEntityDied {
    pub cause: Option<LuaEntity>,
    pub damage_type: Option<LuaDamagePrototype>,
    pub entity: LuaEntity,
    pub force: Option<LuaForce>,
    pub loot: LuaInventory,
    pub name: Events,
    pub tick: u32,
}

/// Called when one of an entity's personal logistic slots changes.
pub struct OnEntityLogisticSlotChanged {
    pub entity: LuaEntity,
    pub name: Events,
    pub player_index: Option<u32>,
    pub slot_index: u32,
    pub tick: u32,
}

/// Called after an entity has been renamed either by the player or through script.
pub struct OnEntityRenamed {
    pub by_script: bool,
    pub entity: LuaEntity,
    pub name: Events,
    pub old_name: String,
    pub player_index: Option<u32>,
    pub tick: u32,
}

/// Called after entity copy-paste is done.
pub struct OnEntitySettingsPasted {
    pub destination: LuaEntity,
    pub name: Events,
    pub player_index: u32,
    pub source: LuaEntity,
    pub tick: u32,
}

/// Called when an entity is spawned by a EnemySpawner
pub struct OnEntitySpawned {
    pub entity: LuaEntity,
    pub name: Events,
    pub spawner: LuaEntity,
    pub tick: u32,
}

/// Called after equipment is inserted into an equipment grid.
pub struct OnEquipmentInserted {
    pub equipment: LuaEquipment,
    pub grid: LuaEquipmentGrid,
    pub name: Events,
    pub tick: u32,
}

/// Called after equipment is removed from an equipment grid.
pub struct OnEquipmentRemoved {
    pub count: u32,
    pub equipment: String,
    pub grid: LuaEquipmentGrid,
    pub name: Events,
    pub tick: u32,
}

/// Called when the a forces cease fire values change.
pub struct OnForceCeaseFireChanged {
    pub added: bool,
    pub force: LuaForce,
    pub name: Events,
    pub other_force: LuaForce,
    pub tick: u32,
}

/// Called when a new force is created using `game.create_force()`
pub struct OnForceCreated {
    pub force: LuaForce,
    pub name: Events,
    pub tick: u32,
}

/// Called when the a forces friends change.
pub struct OnForceFriendsChanged {
    pub added: bool,
    pub force: LuaForce,
    pub name: Events,
    pub other_force: LuaForce,
    pub tick: u32,
}

/// Called when [LuaForce::reset](LuaForce::reset) is finished.
pub struct OnForceReset {
    pub force: LuaForce,
    pub name: Events,
    pub tick: u32,
}

/// Called after two forces have been merged using `game.merge_forces()`.
pub struct OnForcesMerged {
    pub destination: LuaForce,
    pub name: Events,
    pub source_index: u32,
    pub source_name: String,
    pub tick: u32,
}

/// Called when two forces are about to be merged using `game.merge_forces()`.
pub struct OnForcesMerging {
    pub destination: LuaForce,
    pub name: Events,
    pub source: LuaForce,
    pub tick: u32,
}

/// Called when a game is created from a scenario. This is fired for every mod, even when the scenario's save data already includes it. In those cases however, [LuaBootstrap::on_init](LuaBootstrap::on_init) is not fired.
pub struct OnGameCreatedFromScenario {
    pub name: Events,
    pub tick: u32,
}

/// Called when [LuaGuiElement](LuaGuiElement) checked state is changed (related to checkboxes and radio buttons).
pub struct OnGuiCheckedStateChanged {
    pub element: LuaGuiElement,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when [LuaGuiElement](LuaGuiElement) is clicked.
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

/// Called when the player closes the GUI they have open.
/// 
/// This can only be raised when the GUI's player controller is still valid. If a GUI is thus closed due to the player disconnecting, dying, or becoming a spectator in other ways, it won't cause this event to be raised.
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

/// Called when a [LuaGuiElement](LuaGuiElement) is confirmed, for example by pressing Enter in a textfield.
pub struct OnGuiConfirmed {
    pub alt: bool,
    pub control: bool,
    pub element: LuaGuiElement,
    pub name: Events,
    pub player_index: u32,
    pub shift: bool,
    pub tick: u32,
}

/// Called when [LuaGuiElement](LuaGuiElement) element value is changed (related to choose element buttons).
pub struct OnGuiElemChanged {
    pub element: LuaGuiElement,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when [LuaGuiElement](LuaGuiElement) is hovered by the mouse.
pub struct OnGuiHover {
    pub element: LuaGuiElement,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when the player's cursor leaves a [LuaGuiElement](LuaGuiElement) that was previously hovered.
pub struct OnGuiLeave {
    pub element: LuaGuiElement,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when [LuaGuiElement](LuaGuiElement) element location is changed (related to frames in `player.gui.screen`).
pub struct OnGuiLocationChanged {
    pub element: LuaGuiElement,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when the player opens a GUI.
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

/// Called when [LuaGuiElement](LuaGuiElement) selected tab is changed (related to tabbed-panes).
pub struct OnGuiSelectedTabChanged {
    pub element: LuaGuiElement,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when [LuaGuiElement](LuaGuiElement) selection state is changed (related to drop-downs and listboxes).
pub struct OnGuiSelectionStateChanged {
    pub element: LuaGuiElement,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when [LuaGuiElement](LuaGuiElement) switch state is changed (related to switches).
pub struct OnGuiSwitchStateChanged {
    pub element: LuaGuiElement,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when [LuaGuiElement](LuaGuiElement) text is changed by the player.
pub struct OnGuiTextChanged {
    pub element: LuaGuiElement,
    pub name: Events,
    pub player_index: u32,
    pub text: String,
    pub tick: u32,
}

/// Called when [LuaGuiElement](LuaGuiElement) slider value is changed (related to the slider element).
pub struct OnGuiValueChanged {
    pub element: LuaGuiElement,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when a land mine is armed.
pub struct OnLandMineArmed {
    pub mine: LuaEntity,
    pub name: Events,
    pub tick: u32,
}

/// Called when a custom Lua shortcut is pressed.
pub struct OnLuaShortcut {
    pub name: Events,
    pub player_index: u32,
    pub prototype_name: String,
    pub tick: u32,
}

/// Called when an entity is marked for deconstruction with the Deconstruction planner or via script. Can be filtered using [LuaEntityMarkedForDeconstructionEventFilter](LuaEntityMarkedForDeconstructionEventFilter).
pub struct OnMarkedForDeconstruction {
    pub entity: LuaEntity,
    pub name: Events,
    pub player_index: Option<u32>,
    pub tick: u32,
}

/// Called when an entity is marked for upgrade with the Upgrade planner or via script. Can be filtered using [LuaEntityMarkedForUpgradeEventFilter](LuaEntityMarkedForUpgradeEventFilter).
pub struct OnMarkedForUpgrade {
    pub direction: Option<Direction>,
    pub entity: LuaEntity,
    pub name: Events,
    pub player_index: Option<u32>,
    pub target: LuaEntityPrototype,
    pub tick: u32,
}

/// Called after a player purchases some offer from a `market` entity.
pub struct OnMarketItemPurchased {
    pub count: u32,
    pub market: LuaEntity,
    pub name: Events,
    pub offer_index: u32,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when the player uses the 'Open item GUI' control on an item defined with the 'mod-openable' flag
pub struct OnModItemOpened {
    pub item: LuaItemPrototype,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called directly after a permission group is added.
pub struct OnPermissionGroupAdded {
    pub group: LuaPermissionGroup,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called directly after a permission group is deleted.
pub struct OnPermissionGroupDeleted {
    pub group_name: String,
    pub id: u32,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called directly after a permission group is edited in some way.
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

/// Called directly after a permission string is imported.
pub struct OnPermissionStringImported {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when a player picks up an item.
pub struct OnPickedUpItem {
    pub item_stack: SimpleItemStack,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called after a player alt-reverse-selects an area with a selection-tool item.
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

/// Called after a player alt-selects an area with a selection-tool item.
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

/// Called after a players ammo inventory changed in some way.
pub struct OnPlayerAmmoInventoryChanged {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called after a players armor inventory changed in some way.
pub struct OnPlayerArmorInventoryChanged {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when a player is banned.
pub struct OnPlayerBanned {
    pub by_player: Option<u32>,
    pub name: Events,
    pub player_index: Option<u32>,
    pub player_name: String,
    pub reason: Option<String>,
    pub tick: u32,
}

/// Called after a player builds tiles.
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

/// Called when a player cancels crafting.
pub struct OnPlayerCancelledCrafting {
    pub cancel_count: u32,
    pub items: LuaInventory,
    pub name: Events,
    pub player_index: u32,
    pub recipe: LuaRecipe,
    pub tick: u32,
}

/// Called after a player changes forces.
pub struct OnPlayerChangedForce {
    pub force: LuaForce,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when the tile position a player is located at changes.
pub struct OnPlayerChangedPosition {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called after a player changes surfaces.
pub struct OnPlayerChangedSurface {
    pub name: Events,
    pub player_index: u32,
    pub surface_index: u32,
    pub tick: u32,
}

/// Called when cheat mode is disabled on a player.
pub struct OnPlayerCheatModeDisabled {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when cheat mode is enabled on a player.
pub struct OnPlayerCheatModeEnabled {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when a player clicks a gps tag
pub struct OnPlayerClickedGpsTag {
    pub name: Events,
    pub player_index: u32,
    pub position: MapPosition,
    pub surface: String,
    pub tick: u32,
}

/// Called when a player clicks the "confirm" button in the configure Blueprint GUI.
pub struct OnPlayerConfiguredBlueprint {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when a player configures spidertron remote to be connected with a given spidertron
pub struct OnPlayerConfiguredSpiderRemote {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
    pub vehicle: LuaEntity,
}

/// Called when the player finishes crafting an item. This event fires just before the results are inserted into the player's inventory, not when the crafting is queued (see [on_pre_player_crafted_item](on_pre_player_crafted_item)).
pub struct OnPlayerCraftedItem {
    pub item_stack: LuaItemStack,
    pub name: Events,
    pub player_index: u32,
    pub recipe: LuaRecipe,
    pub tick: u32,
}

/// Called after the player was created.
pub struct OnPlayerCreated {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called after a player's [cursor stack](LuaControl::cursor_stack) changed in some way.
pub struct OnPlayerCursorStackChanged {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when a player selects an area with a deconstruction planner.
pub struct OnPlayerDeconstructedArea {
    pub alt: bool,
    pub area: BoundingBox,
    pub item: String,
    pub name: Events,
    pub player_index: u32,
    pub surface: LuaSurface,
    pub tick: u32,
}

/// Called when a player is demoted.
pub struct OnPlayerDemoted {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called after a player dies.
pub struct OnPlayerDied {
    pub cause: Option<LuaEntity>,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when the display resolution changes for a given player.
pub struct OnPlayerDisplayResolutionChanged {
    pub name: Events,
    pub old_resolution: DisplayResolution,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when the display scale changes for a given player.
pub struct OnPlayerDisplayScaleChanged {
    pub name: Events,
    pub old_scale: f64,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when the player's driving state has changed, meaning a player has either entered or left a vehicle.
pub struct OnPlayerDrivingChangedState {
    pub entity: Option<LuaEntity>,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when a player drops an item on the ground.
pub struct OnPlayerDroppedItem {
    pub entity: LuaEntity,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when a player fast-transfers something to or from an entity.
pub struct OnPlayerFastTransferred {
    pub entity: LuaEntity,
    pub from_player: bool,
    pub is_split: bool,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called after player flushed fluid
pub struct OnPlayerFlushedFluid {
    pub amount: f64,
    pub entity: LuaEntity,
    pub fluid: String,
    pub name: Events,
    pub only_this_entity: bool,
    pub player_index: u32,
    pub tick: u32,
}

/// Called after a players gun inventory changed in some way.
pub struct OnPlayerGunInventoryChanged {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called after a player joins the game. This is not called when loading a save file in singleplayer, as the player doesn't actually leave the game, and the save is just on pause until they rejoin.
pub struct OnPlayerJoinedGame {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when a player is kicked.
pub struct OnPlayerKicked {
    pub by_player: Option<u32>,
    pub name: Events,
    pub player_index: u32,
    pub reason: Option<String>,
    pub tick: u32,
}

/// Called after a player leaves the game. This is not called when closing a save file in singleplayer, as the player doesn't actually leave the game, and the save is just on pause until they rejoin.
pub struct OnPlayerLeftGame {
    pub name: Events,
    pub player_index: u32,
    pub reason: DisconnectReason,
    pub tick: u32,
}

/// Called after a players main inventory changed in some way.
pub struct OnPlayerMainInventoryChanged {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called after the results of an entity being mined are collected just before the entity is destroyed. After this event any items in the buffer will be transferred into the player as if they came from mining the entity. Can be filtered using [LuaPlayerMinedEntityEventFilter](LuaPlayerMinedEntityEventFilter).
pub struct OnPlayerMinedEntity {
    pub buffer: LuaInventory,
    pub entity: LuaEntity,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when the player mines something.
pub struct OnPlayerMinedItem {
    pub item_stack: SimpleItemStack,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called after a player mines tiles.
pub struct OnPlayerMinedTile {
    pub name: Events,
    pub player_index: u32,
    pub surface_index: u32,
    pub tick: u32,
    pub tiles: Vec<OldTileAndPosition>,
}

/// Called when a player is muted.
pub struct OnPlayerMuted {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when a player invokes the "smart pipette" over an entity.
pub struct OnPlayerPipette {
    pub item: LuaItemPrototype,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
    pub used_cheat_mode: bool,
}

/// Called after the player puts equipment in an equipment grid
pub struct OnPlayerPlacedEquipment {
    pub equipment: LuaEquipment,
    pub grid: LuaEquipmentGrid,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when a player is promoted.
pub struct OnPlayerPromoted {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when a player is removed (deleted) from the game. This is markedly different from a player temporarily [leaving](on_player_left_game) the game, and instead behaves like the player never existed in the save file.
pub struct OnPlayerRemoved {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called after the player removes equipment from an equipment grid
pub struct OnPlayerRemovedEquipment {
    pub count: u32,
    pub equipment: String,
    pub grid: LuaEquipmentGrid,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when a player repairs an entity. Can be filtered using [LuaPlayerRepairedEntityEventFilter](LuaPlayerRepairedEntityEventFilter).
pub struct OnPlayerRepairedEntity {
    pub entity: LuaEntity,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called after a player respawns.
pub struct OnPlayerRespawned {
    pub name: Events,
    pub player_index: u32,
    pub player_port: Option<LuaEntity>,
    pub tick: u32,
}

/// Called after a player reverse-selects an area with a selection-tool item.
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

/// Called when the player rotates an entity. This event is only fired when the entity actually changes its orientation -- pressing the rotate key on an entity that can't be rotated won't fire this event.
pub struct OnPlayerRotatedEntity {
    pub entity: LuaEntity,
    pub name: Events,
    pub player_index: u32,
    pub previous_direction: Direction,
    pub tick: u32,
}

/// Called after a player selects an area with a selection-tool item.
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

/// Called when a player sets a quickbar slot to anything (new value, or set to empty).
pub struct OnPlayerSetQuickBarSlot {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when a player selects an area with a blueprint.
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

/// Called when a player toggles alt mode, also known as "show entity info".
pub struct OnPlayerToggledAltMode {
    pub alt_mode: bool,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when a player toggles the map editor on or off.
pub struct OnPlayerToggledMapEditor {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called after a players trash inventory changed in some way.
pub struct OnPlayerTrashInventoryChanged {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when a player is un-banned.
pub struct OnPlayerUnbanned {
    pub by_player: Option<u32>,
    pub name: Events,
    pub player_index: Option<u32>,
    pub player_name: String,
    pub reason: Option<String>,
    pub tick: u32,
}

/// Called when a player is unmuted.
pub struct OnPlayerUnmuted {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when a player uses a capsule that results in some game action.
pub struct OnPlayerUsedCapsule {
    pub item: LuaItemPrototype,
    pub name: Events,
    pub player_index: u32,
    pub position: MapPosition,
    pub tick: u32,
}

/// Called when a player uses spidertron remote to send a spidertron to a given position
pub struct OnPlayerUsedSpiderRemote {
    pub name: Events,
    pub player_index: u32,
    pub position: MapPosition,
    pub success: bool,
    pub tick: u32,
    pub vehicle: LuaEntity,
}

/// Called after an entity dies. Can be filtered using [LuaPostEntityDiedEventFilter](LuaPostEntityDiedEventFilter).
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

/// Called when players uses an item to build something. Called before [on_built_entity](on_built_entity).
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

/// Called before one or more chunks are deleted using [LuaSurface::delete_chunk](LuaSurface::delete_chunk).
pub struct OnPreChunkDeleted {
    pub name: Events,
    pub positions: Vec<ChunkPosition>,
    pub surface_index: u32,
    pub tick: u32,
}

/// Called before entity copy-paste is done.
pub struct OnPreEntitySettingsPasted {
    pub destination: LuaEntity,
    pub name: Events,
    pub player_index: u32,
    pub source: LuaEntity,
    pub tick: u32,
}

/// Called before a ghost entity is destroyed as a result of being marked for deconstruction. Can be filtered using [LuaPreGhostDeconstructedEventFilter](LuaPreGhostDeconstructedEventFilter).
pub struct OnPreGhostDeconstructed {
    pub ghost: LuaEntity,
    pub name: Events,
    pub player_index: Option<u32>,
    pub tick: u32,
}

/// Called before a ghost entity is upgraded. Can be filtered using [LuaPreGhostUpgradedEventFilter](LuaPreGhostUpgradedEventFilter).
pub struct OnPreGhostUpgraded {
    pub ghost: LuaEntity,
    pub name: Events,
    pub player_index: Option<u32>,
    pub target: LuaEntityPrototype,
    pub tick: u32,
}

/// Called directly before a permission group is deleted.
pub struct OnPrePermissionGroupDeleted {
    pub group: LuaPermissionGroup,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called directly before a permission string is imported.
pub struct OnPrePermissionStringImported {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when a player queues something to be crafted.
pub struct OnPrePlayerCraftedItem {
    pub items: LuaInventory,
    pub name: Events,
    pub player_index: u32,
    pub queued_count: u32,
    pub recipe: LuaRecipe,
    pub tick: u32,
}

/// Called before a players dies.
pub struct OnPrePlayerDied {
    pub cause: Option<LuaEntity>,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called before a player leaves the game.
pub struct OnPrePlayerLeftGame {
    pub name: Events,
    pub player_index: u32,
    pub reason: DisconnectReason,
    pub tick: u32,
}

/// Called when the player completes a mining action, but before the entity is potentially removed from the map. This is called even if the entity does not end up being removed. Can be filtered using [LuaPrePlayerMinedEntityEventFilter](LuaPrePlayerMinedEntityEventFilter).
pub struct OnPrePlayerMinedItem {
    pub entity: LuaEntity,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called before a player is removed (deleted) from the game. This is markedly different from a player temporarily [leaving](on_player_left_game) the game, and instead behaves like the player never existed in the save file.
pub struct OnPrePlayerRemoved {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called before a player toggles the map editor on or off.
pub struct OnPrePlayerToggledMapEditor {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called directly before a robot explodes cliffs.
pub struct OnPreRobotExplodedCliff {
    pub cliff: LuaEntity,
    pub item: LuaItemPrototype,
    pub name: Events,
    pub robot: LuaEntity,
    pub tick: u32,
}

/// Called just before a script inventory is resized.
pub struct OnPreScriptInventoryResized {
    pub inventory: LuaInventory,
    pub mod_name: String,
    pub name: Events,
    pub new_size: u32,
    pub old_size: u32,
    pub player_index: Option<u32>,
    pub tick: u32,
}

/// Called just before a surface is cleared (all entities removed and all chunks deleted).
pub struct OnPreSurfaceCleared {
    pub name: Events,
    pub surface_index: u32,
    pub tick: u32,
}

/// Called just before a surface is deleted.
pub struct OnPreSurfaceDeleted {
    pub name: Events,
    pub surface_index: u32,
    pub tick: u32,
}

/// Called when research is cancelled.
pub struct OnResearchCancelled {
    pub force: LuaForce,
    pub name: Events,
    pub research: HashMap<String, u32>,
    pub tick: u32,
}

/// Called when a research finishes.
pub struct OnResearchFinished {
    pub by_script: bool,
    pub name: Events,
    pub research: LuaTechnology,
    pub tick: u32,
}

/// Called when a research is reversed (unresearched).
pub struct OnResearchReversed {
    pub by_script: bool,
    pub name: Events,
    pub research: LuaTechnology,
    pub tick: u32,
}

/// Called when a technology research starts.
pub struct OnResearchStarted {
    pub last_research: Option<LuaTechnology>,
    pub name: Events,
    pub research: LuaTechnology,
    pub tick: u32,
}

/// Called when a resource entity reaches 0 or its minimum yield for infinite resources.
pub struct OnResourceDepleted {
    pub entity: LuaEntity,
    pub name: Events,
    pub tick: u32,
}

/// Called when a construction robot builds an entity. Can be filtered using [LuaRobotBuiltEntityEventFilter](LuaRobotBuiltEntityEventFilter).
pub struct OnRobotBuiltEntity {
    pub created_entity: LuaEntity,
    pub name: Events,
    pub robot: LuaEntity,
    pub stack: LuaItemStack,
    pub tags: Option<Tags>,
    pub tick: u32,
}

/// Called after a robot builds tiles.
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

/// Called directly after a robot explodes cliffs.
pub struct OnRobotExplodedCliff {
    pub item: LuaItemPrototype,
    pub name: Events,
    pub robot: LuaEntity,
    pub tick: u32,
}

/// Called when a robot mines an entity.
pub struct OnRobotMined {
    pub item_stack: SimpleItemStack,
    pub name: Events,
    pub robot: LuaEntity,
    pub tick: u32,
}

/// Called after the results of an entity being mined are collected just before the entity is destroyed. After this event any items in the buffer will be transferred into the robot as if they came from mining the entity. Can be filtered using [LuaRobotMinedEntityEventFilter](LuaRobotMinedEntityEventFilter).
pub struct OnRobotMinedEntity {
    pub buffer: LuaInventory,
    pub entity: LuaEntity,
    pub name: Events,
    pub robot: LuaEntity,
    pub tick: u32,
}

/// Called after a robot mines tiles.
pub struct OnRobotMinedTile {
    pub name: Events,
    pub robot: LuaEntity,
    pub surface_index: u32,
    pub tick: u32,
    pub tiles: Vec<OldTileAndPosition>,
}

/// Called before a robot mines an entity. Can be filtered using [LuaPreRobotMinedEntityEventFilter](LuaPreRobotMinedEntityEventFilter).
pub struct OnRobotPreMined {
    pub entity: LuaEntity,
    pub name: Events,
    pub robot: LuaEntity,
    pub tick: u32,
}

/// Called when a rocket silo is ordered to be launched.
pub struct OnRocketLaunchOrdered {
    pub name: Events,
    pub player_index: Option<u32>,
    pub rocket: LuaEntity,
    pub rocket_silo: LuaEntity,
    pub tick: u32,
}

/// Called when the rocket is launched.
pub struct OnRocketLaunched {
    pub name: Events,
    pub player_index: Option<u32>,
    pub rocket: LuaEntity,
    pub rocket_silo: Option<LuaEntity>,
    pub tick: u32,
}

/// Called when a runtime mod setting is changed by a player.
pub struct OnRuntimeModSettingChanged {
    pub name: Events,
    pub player_index: Option<u32>,
    pub setting: String,
    pub setting_type: String,
    pub tick: u32,
}

/// Called just after a script inventory is resized.
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

/// Called when a [LuaSurface::request_path](LuaSurface::request_path) call completes.
pub struct OnScriptPathRequestFinished {
    pub id: u32,
    pub name: Events,
    pub path: Option<Vec<PathfinderWaypoint>>,
    pub tick: u32,
    pub try_again_later: bool,
}

/// Called when a script trigger effect is triggered.
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

/// Called when an entity of type `radar` finishes scanning a sector. Can be filtered for the radar using [LuaSectorScannedEventFilter](LuaSectorScannedEventFilter).
pub struct OnSectorScanned {
    pub area: BoundingBox,
    pub chunk_position: ChunkPosition,
    pub name: Events,
    pub radar: LuaEntity,
    pub tick: u32,
}

/// Called after the selected entity changes for a given player.
pub struct OnSelectedEntityChanged {
    pub last_entity: Option<LuaEntity>,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

/// Called when a spider finishes moving to its autopilot position.
pub struct OnSpiderCommandCompleted {
    pub name: Events,
    pub tick: u32,
    pub vehicle: LuaEntity,
}

/// Called when a translation request generated through [LuaPlayer::request_translation](LuaPlayer::request_translation) or [LuaPlayer::request_translations](LuaPlayer::request_translations) has been completed.
pub struct OnStringTranslated {
    pub id: u32,
    pub localised_string: LocalisedString,
    pub name: Events,
    pub player_index: u32,
    pub result: String,
    pub tick: u32,
    pub translated: bool,
}

/// Called just after a surface is cleared (all entities removed and all chunks deleted).
pub struct OnSurfaceCleared {
    pub name: Events,
    pub surface_index: u32,
    pub tick: u32,
}

/// Called when a surface is created.
pub struct OnSurfaceCreated {
    pub name: Events,
    pub surface_index: u32,
    pub tick: u32,
}

/// Called after a surface is deleted.
pub struct OnSurfaceDeleted {
    pub name: Events,
    pub surface_index: u32,
    pub tick: u32,
}

/// Called after a surface is imported via the map editor.
pub struct OnSurfaceImported {
    pub name: Events,
    pub original_name: String,
    pub surface_index: u32,
    pub tick: u32,
}

/// Called when a surface is renamed.
pub struct OnSurfaceRenamed {
    pub name: Events,
    pub new_name: String,
    pub old_name: String,
    pub surface_index: u32,
    pub tick: u32,
}

/// Called when [LuaForce::reset_technology_effects](LuaForce::reset_technology_effects) is finished.
pub struct OnTechnologyEffectsReset {
    pub force: LuaForce,
    pub name: Events,
    pub tick: u32,
}

/// It is fired once every tick. Since this event is fired every tick, its handler shouldn't include performance heavy code.
pub struct OnTick {
    pub name: Events,
    pub tick: u32,
}

/// Called when a train changes state (started to stopped and vice versa)
pub struct OnTrainChangedState {
    pub name: Events,
    pub old_state: TrainState,
    pub tick: u32,
    pub train: LuaTrain,
}

/// Called when a new train is created either through disconnecting/connecting an existing one or building a new one.
pub struct OnTrainCreated {
    pub name: Events,
    pub old_train_id_1: Option<u32>,
    pub old_train_id_2: Option<u32>,
    pub tick: u32,
    pub train: LuaTrain,
}

/// Called when a trains schedule is changed either by the player or through script.
pub struct OnTrainScheduleChanged {
    pub name: Events,
    pub player_index: Option<u32>,
    pub tick: u32,
    pub train: LuaTrain,
}

/// Called when an entity with a trigger prototype (such as capsules) create an entity AND that trigger prototype defined `trigger_created_entity="true"`.
pub struct OnTriggerCreatedEntity {
    pub entity: LuaEntity,
    pub name: Events,
    pub source: Option<LuaEntity>,
    pub tick: u32,
}

/// Called when an entity with a trigger prototype (such as capsules) fire an artillery projectile AND that trigger prototype defined `trigger_fired_artillery="true"`.
pub struct OnTriggerFiredArtillery {
    pub entity: LuaEntity,
    pub name: Events,
    pub source: Option<LuaEntity>,
    pub tick: u32,
}

/// Called when a unit is added to a unit group.
pub struct OnUnitAddedToGroup {
    pub group: LuaUnitGroup,
    pub name: Events,
    pub tick: u32,
    pub unit: LuaEntity,
}

/// Called when a new unit group is created, before any members are added to it.
pub struct OnUnitGroupCreated {
    pub group: LuaUnitGroup,
    pub name: Events,
    pub tick: u32,
}

/// Called when a unit group finishes gathering and starts executing its command.
pub struct OnUnitGroupFinishedGathering {
    pub group: LuaUnitGroup,
    pub name: Events,
    pub tick: u32,
}

/// Called when a unit is removed from a unit group.
pub struct OnUnitRemovedFromGroup {
    pub group: LuaUnitGroup,
    pub name: Events,
    pub tick: u32,
    pub unit: LuaEntity,
}

/// Called when a worker (construction or logistic) robot expires through a lack of energy.
pub struct OnWorkerRobotExpired {
    pub name: Events,
    pub robot: LuaEntity,
    pub tick: u32,
}

/// A static event mods can use to tell other mods they built something by script. This event is only raised if a mod does so with [LuaBootstrap::raise_event](LuaBootstrap::raise_event) or [LuaBootstrap::raise_script_built](LuaBootstrap::raise_script_built), or when `raise_built` is passed to [LuaSurface::create_entity](LuaSurface::create_entity). Can be filtered using [LuaScriptRaisedBuiltEventFilter](LuaScriptRaisedBuiltEventFilter).
pub struct ScriptRaisedBuilt {
    pub entity: LuaEntity,
    pub name: Events,
    pub tick: u32,
}

/// A static event mods can use to tell other mods they destroyed something by script. This event is only raised if a mod does so with [LuaBootstrap::raise_event](LuaBootstrap::raise_event) or [LuaBootstrap::raise_script_destroy](LuaBootstrap::raise_script_destroy), or when `raise_destroy` is passed to [LuaEntity::destroy](LuaEntity::destroy). Can be filtered using [LuaScriptRaisedDestroyEventFilter](LuaScriptRaisedDestroyEventFilter).
pub struct ScriptRaisedDestroy {
    pub entity: LuaEntity,
    pub name: Events,
    pub tick: u32,
}

/// A static event mods can use to tell other mods they revived something by script. This event is only raised if a mod does so with [LuaBootstrap::raise_event](LuaBootstrap::raise_event) or [LuaBootstrap::raise_script_revive](LuaBootstrap::raise_script_revive), or when `raise_revive` is passed to [LuaEntity::revive](LuaEntity::revive). Can be filtered using [LuaScriptRaisedReviveEventFilter](LuaScriptRaisedReviveEventFilter).
pub struct ScriptRaisedRevive {
    pub entity: LuaEntity,
    pub name: Events,
    pub tags: Option<Tags>,
    pub tick: u32,
}

/// A static event mods can use to tell other mods they changed tiles on a surface by script. This event is only raised if a mod does so with [LuaBootstrap::raise_event](LuaBootstrap::raise_event) or [LuaBootstrap::raise_script_set_tiles](LuaBootstrap::raise_script_set_tiles), or when `raise_event` is passed to [LuaSurface::set_tiles](LuaSurface::set_tiles).
pub struct ScriptRaisedSetTiles {
    pub name: Events,
    pub surface_index: u32,
    pub tick: u32,
    pub tiles: Vec<Tile>,
}

/// A static event mods can use to tell other mods they teleported something by script. This event is only raised if a mod does so with [LuaBootstrap::raise_event](LuaBootstrap::raise_event) or [LuaBootstrap::raise_script_teleported](LuaBootstrap::raise_script_teleported), or when `raise_teleported` is passed to [LuaControl::teleport](LuaControl::teleport). Can be filtered using [LuaScriptRaisedTeleportedEventFilter](LuaScriptRaisedTeleportedEventFilter).
pub struct ScriptRaisedTeleported {
    pub entity: LuaEntity,
    pub name: Events,
    pub old_position: MapPosition,
    pub old_surface_index: u8,
    pub tick: u32,
}
