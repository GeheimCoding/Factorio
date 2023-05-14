use std::collections::HashMap;
use std::collections::HashSet;

use super::concepts::*;
use super::defines::*;

/// Collection of settings for overriding default ai behavior.
pub struct LuaAISettings {
    /// If enabled, units that repeatedly fail to succeed at commands will be destroyed.
    pub allow_destroy_when_commands_fail: bool,
    /// If enabled, units that have nothing else to do will attempt to return to a spawner.
    pub allow_try_return_to_spawner: bool,
    /// If enabled, units will try to separate themselves from nearby friendly units.
    pub do_separation: bool,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// Defines how coarse the pathfinder's grid is, where smaller values mean a coarser grid. Defaults to `0`, which equals a resolution of `1x1` tiles, centered on tile centers. Values range from `-8` to `8` inclusive, where each integer increment doubles/halves the resolution. So, a resolution of `-8` equals a grid of `256x256` tiles, and a resolution of `8` equals `1/256` of a tile.
    pub path_resolution_modifier: i8,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Collection of settings for overriding default ai behavior.
pub trait LuaAISettingsMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Control behavior for accumulators.
pub struct LuaAccumulatorControlBehavior {
    pub lua_control_behavior: Box<LuaControlBehavior>,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    pub output_signal: SignalID,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Control behavior for accumulators.
pub trait LuaAccumulatorControlBehaviorMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Prototype of a achievement.
pub struct LuaAchievementPrototype {
    pub allowed_without_fight: bool,
    pub hidden: bool,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    /// Name of this prototype.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of a achievement.
pub trait LuaAchievementPrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Prototype of a ammo category.
pub struct LuaAmmoCategoryPrototype {
    pub bonus_gui_order: String,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    /// Name of this prototype.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of a ammo category.
pub trait LuaAmmoCategoryPrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Control behavior for arithmetic combinators.
pub struct LuaArithmeticCombinatorControlBehavior {
    pub lua_combinator_control_behavior: Box<LuaCombinatorControlBehavior>,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// This arithmetic combinator's parameters.
    pub parameters: ArithmeticCombinatorParameters,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Control behavior for arithmetic combinators.
pub trait LuaArithmeticCombinatorControlBehaviorMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Prototype of an autoplace control.
pub struct LuaAutoplaceControlPrototype {
    pub can_be_disabled: bool,
    /// Category name of this prototype.
    pub category: String,
    pub control_order: String,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    /// Name of this prototype.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    pub richness: bool,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of an autoplace control.
pub trait LuaAutoplaceControlPrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

pub struct LuaBootstrapLevel {
    /// The campaign name if any.
    pub campaign_name: Option<String>,
    /// Is this level a simulation? (The main menu and 'Tips and tricks' use simulations)
    pub is_simulation: Option<bool>,
    /// Is this level a tutorial?
    pub is_tutorial: Option<bool>,
    /// The level name.
    pub level_name: String,
    /// The mod name if any.
    pub mod_name: Option<String>,
}

/// Entry point for registering event handlers. It is accessible through the global object named `script`.
pub struct LuaBootstrap {
    /// A dictionary listing the names of all currently active mods and mapping them to their version.
    pub active_mods: HashMap<String, String>,
    /// Information about the currently running scenario/campaign/tutorial.
    pub level: LuaBootstrapLevel,
    /// The name of the mod from the environment this is used in.
    pub mod_name: String,
    /// This object's name.
    pub object_name: String,
}

pub enum LuaBootstrapMethodsOnConfigurationChangedHandlerUnion {
    Function(function),
    Nil,
}

pub enum LuaBootstrapMethodsOnEventEventUnion {
    DefinesEvents(Events),
    Array(Vec<Events>),
    String(String),
}

pub enum LuaBootstrapMethodsOnEventHandlerUnion {
    Function(function),
    Nil,
}

pub enum LuaBootstrapMethodsOnInitHandlerUnion {
    Function(function),
    Nil,
}

pub enum LuaBootstrapMethodsOnLoadHandlerUnion {
    Function(function),
    Nil,
}

pub enum LuaBootstrapMethodsOnNthTickHandlerUnion {
    Function(function),
    Nil,
}

pub enum LuaBootstrapMethodsOnNthTickTickUnion {
    Uint(u32),
    Array(Vec<u32>),
    Nil,
}

/// Entry point for registering event handlers. It is accessible through the global object named `script`.
pub trait LuaBootstrapMethods {
    /// Generate a new, unique event ID that can be used to raise custom events with [LuaBootstrap::raise_event](LuaBootstrap::raise_event).
    fn generate_event_name() -> u32;
    /// Gets the filters for the given event.
    fn get_event_filter(event: u32) -> Option<EventFilter>;
    /// Find the event handler for an event.
    fn get_event_handler(event: u32) -> Option<fn(EventData) -> ()>;
    /// Gets the mod event order as a string.
    fn get_event_order() -> String;
    /// Gets the prototype history for the given type and name.
    fn get_prototype_history(name: String, typ: String) -> PrototypeHistory;
    /// Register a function to be run when mod configuration changes. This is called when the game version or any mod version changed, when any mod was added or removed, when a startup setting has changed, when any prototypes have been added or removed, or when a migration was applied. It allows the mod to make any changes it deems appropriate to both the data structures in its [global](global) table or to the game state through [LuaGameScript](LuaGameScript).
    fn on_configuration_changed(handler: LuaBootstrapMethodsOnConfigurationChangedHandlerUnion);
    /// Register a handler to run on the specified event(s). Each mod can only register once for every event, as any additional registration will overwrite the previous one. This holds true even if different filters are used for subsequent registrations.
    fn on_event(event: LuaBootstrapMethodsOnEventEventUnion, filters: EventFilter, handler: LuaBootstrapMethodsOnEventHandlerUnion);
    /// Register a function to be run on mod initialization. This is only called when a new save game is created or when a save file is loaded that previously didn't contain the mod. During it, the mod gets the chance to set up initial values that it will use for its lifetime. It has full access to [LuaGameScript](LuaGameScript) and the [global](global) table and can change anything about them that it deems appropriate. No other events will be raised for the mod until it has finished this step.
    fn on_init(handler: LuaBootstrapMethodsOnInitHandlerUnion);
    /// Register a function to be run on save load. This is only called for mods that have been part of the save previously, or for players connecting to a running multiplayer session.
    ///
    /// It gives the mod the opportunity to rectify potential differences in local state introduced by the save/load cycle. Doing anything other than the following three will lead to desyncs, breaking multiplayer and replay functionality. Access to [LuaGameScript](LuaGameScript) is not available. The [global](global) table can be accessed and is safe to read from, but not write to, as doing so will lead to an error.
    ///
    /// The only legitimate uses of this event are these:
    /// - Re-setup [metatables](https://www.lua.org/pil/13.html) as they are not persisted through the save/load cycle.
    /// - Re-setup conditional event handlers, meaning subscribing to an event only when some condition is met to save processing time.
    /// - Create local references to data stored in the [global](global) table.
    ///
    /// For all other purposes, [LuaBootstrap::on_init](LuaBootstrap::on_init), [LuaBootstrap::on_configuration_changed](LuaBootstrap::on_configuration_changed) or [migrations](migrations) should be used instead.
    fn on_load(handler: LuaBootstrapMethodsOnLoadHandlerUnion);
    /// Register a handler to run every nth-tick(s). When the game is on tick 0 it will trigger all registered handlers.
    fn on_nth_tick(handler: LuaBootstrapMethodsOnNthTickHandlerUnion, tick: LuaBootstrapMethodsOnNthTickTickUnion);
    fn raise_biter_base_built(entity: LuaEntity);
    fn raise_console_chat(message: String, player_index: u32);
    /// Raise an event. Only events generated with [LuaBootstrap::generate_event_name](LuaBootstrap::generate_event_name) and the following can be raised:
    ///
    /// - [on_console_chat](on_console_chat)
    /// - [on_player_crafted_item](on_player_crafted_item)
    /// - [on_player_fast_transferred](on_player_fast_transferred)
    /// - [on_biter_base_built](on_biter_base_built)
    /// - [on_market_item_purchased](on_market_item_purchased)
    /// - [script_raised_built](script_raised_built)
    /// - [script_raised_destroy](script_raised_destroy)
    /// - [script_raised_revive](script_raised_revive)
    /// - [script_raised_teleported](script_raised_teleported)
    /// - [script_raised_set_tiles](script_raised_set_tiles)
    fn raise_event(data: table, event: u32);
    fn raise_market_item_purchased(count: u32, market: LuaEntity, offer_index: u32, player_index: u32);
    fn raise_player_crafted_item(item_stack: LuaItemStack, player_index: u32, recipe: LuaRecipe);
    fn raise_player_fast_transferred(entity: LuaEntity, from_player: bool, is_split: bool, player_index: u32);
    fn raise_script_built(entity: LuaEntity);
    fn raise_script_destroy(entity: LuaEntity);
    fn raise_script_revive(entity: LuaEntity, tags: Tags);
    fn raise_script_set_tiles(surface_index: u32, tiles: Vec<Tile>);
    fn raise_script_teleported(entity: LuaEntity, old_position: MapPosition, old_surface_index: u8);
    /// Register a metatable to have linkage recorded and restored when saving/loading. The metatable itself will not be saved. Instead, only the linkage to a registered metatable is saved, and the metatable registered under that name will be used when loading the table.
    fn register_metatable(metatable: table, name: String);
    /// Registers an entity so that after it's destroyed, [on_entity_destroyed](on_entity_destroyed) is called. Once an entity is registered, it stays registered until it is actually destroyed, even through save/load cycles. The registration is global across all mods, meaning once one mod registers an entity, all mods listening to [on_entity_destroyed](on_entity_destroyed) will receive the event when it is destroyed. Registering the same entity multiple times will still only fire the destruction event once, and will return the same registration number.
    fn register_on_entity_destroyed(entity: LuaEntity) -> u64;
    /// Sets the filters for the given event. The filters are only retained when set after the actual event registration, because registering for an event with different or no filters will overwrite previously set ones.
    fn set_event_filter(event: u32, filters: EventFilter);
}

pub enum LuaBurnerOwnerUnion {
    LuaEntity(LuaEntity),
    LuaEquipment(LuaEquipment),
}

/// A reference to the burner energy source owned by a specific [LuaEntity](LuaEntity) or [LuaEquipment](LuaEquipment).
pub struct LuaBurner {
    /// The burnt result inventory.
    pub burnt_result_inventory: LuaInventory,
    /// The currently burning item. Writing `nil` will void the currently burning item without producing a [LuaBurner::burnt_result](LuaBurner::burnt_result).
    pub currently_burning: Option<LuaItemPrototype>,
    /// The fuel categories this burner uses.
    pub fuel_categories: HashMap<String, bool>,
    /// The current heat (energy) stored in this burner.
    pub heat: f64,
    /// The maximum heat (maximum energy) that this burner can store.
    pub heat_capacity: f64,
    /// The fuel inventory.
    pub inventory: LuaInventory,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The owner of this burner energy source
    pub owner: LuaBurnerOwnerUnion,
    /// The amount of energy left in the currently-burning fuel item.
    pub remaining_burning_fuel: f64,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// A reference to the burner energy source owned by a specific [LuaEntity](LuaEntity) or [LuaEquipment](LuaEquipment).
pub trait LuaBurnerMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

pub struct LuaBurnerPrototypeLightFlicker {
    pub border_fix_speed: f32,
    pub color: Color,
    pub derivation_change_deviation: f32,
    pub derivation_change_frequency: f32,
    pub light_intensity_to_size_coefficient: f32,
    pub maximum_intensity: f32,
    pub minimum_intensity: f32,
    pub minimum_light_size: f32,
}

/// Prototype of a burner energy source.
pub struct LuaBurnerPrototype {
    pub burnt_inventory_size: u32,
    pub effectivity: f64,
    /// The emissions of this energy source in `pollution/Joule`. Multiplying it by energy consumption in `Watt` gives `pollution/second`.
    pub emissions: f64,
    pub fuel_categories: HashMap<String, bool>,
    pub fuel_inventory_size: u32,
    /// The light flicker definition for this burner prototype.
    pub light_flicker: Option<LuaBurnerPrototypeLightFlicker>,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    pub render_no_network_icon: bool,
    pub render_no_power_icon: bool,
    /// The smoke sources for this burner prototype.
    pub smoke: Option<Vec<SmokeSource>>,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of a burner energy source.
pub trait LuaBurnerPrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// A chunk iterator can be used for iterating chunks coordinates of a surface.
///
/// The returned type is a [ChunkPositionAndArea](ChunkPositionAndArea) containing the chunk coordinates and its area.
pub struct LuaChunkIterator {
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// A chunk iterator can be used for iterating chunks coordinates of a surface.
///
/// The returned type is a [ChunkPositionAndArea](ChunkPositionAndArea) containing the chunk coordinates and its area.
pub trait LuaChunkIteratorMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// A circuit network associated with a given entity, connector, and wire type.
pub struct LuaCircuitNetwork {
    /// The circuit connector ID on the associated entity this network was gotten from.
    pub circuit_connector_id: CircuitConnectorId,
    /// The number of circuits connected to this network.
    pub connected_circuit_count: u32,
    /// The entity this circuit network reference is associated with.
    pub entity: LuaEntity,
    /// The circuit networks ID.
    pub network_id: u32,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The circuit network signals last tick. `nil` if there were no signals last tick.
    pub signals: Option<Vec<Signal>>,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
    /// The wire type this network is associated with.
    pub wire_type: WireType,
}

/// A circuit network associated with a given entity, connector, and wire type.
pub trait LuaCircuitNetworkMethods {
    fn get_signal(signal: SignalID) -> i32;
    /// All methods and properties that this object supports.
    fn help() -> String;
}

pub struct LuaCombinatorControlBehavior {
    pub lua_control_behavior: Box<LuaControlBehavior>,
    /// The circuit network signals sent by this combinator last tick.
    pub signals_last_tick: Vec<Signal>,
}

///
pub trait LuaCombinatorControlBehaviorMethods {
    /// Gets the value of a specific signal sent by this combinator behavior last tick or `nil` if the signal didn't exist.
    fn get_signal_last_tick(signal: SignalID) -> Option<i32>;
}

/// Allows for the registration of custom console commands through the global object named `commands`. Similarly to [event subscriptions](LuaBootstrap::on_event), these don't persist through a save-and-load cycle.
pub struct LuaCommandProcessor {
    /// Lists the custom commands registered by scripts through `LuaCommandProcessor`.
    pub commands: HashMap<String, LocalisedString>,
    /// Lists the built-in commands of the core game. The [wiki](https://wiki.factorio.com/Console) has an overview of these.
    pub game_commands: HashMap<String, LocalisedString>,
    /// This object's name.
    pub object_name: String,
}

/// Allows for the registration of custom console commands through the global object named `commands`. Similarly to [event subscriptions](LuaBootstrap::on_event), these don't persist through a save-and-load cycle.
pub trait LuaCommandProcessorMethods {
    /// Add a custom console command.
    fn add_command(function: fn(CustomCommandData) -> (), help: LocalisedString, name: String);
    /// Remove a custom console command.
    fn remove_command(name: String) -> bool;
}

/// Control behavior for constant combinators.
pub struct LuaConstantCombinatorControlBehavior {
    pub lua_control_behavior: Box<LuaControlBehavior>,
    /// Turns this constant combinator on and off.
    pub enabled: bool,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// This constant combinator's parameters. `nil` if the [item_slot_count](LuaEntityPrototype::item_slot_count) of the combinator's prototype is `0`.
    ///
    /// Writing `nil` clears the combinator's parameters.
    pub parameters: Option<Vec<ConstantCombinatorParameters>>,
    /// The number of signals this constant combinator supports.
    pub signals_count: u32,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Control behavior for constant combinators.
pub trait LuaConstantCombinatorControlBehaviorMethods {
    /// Gets the signal at the given index. Returned [Signal](Signal) will not contain signal if none is set for the index.
    fn get_signal(index: u32) -> Signal;
    /// All methods and properties that this object supports.
    fn help() -> String;
    /// Sets the signal at the given index.
    fn set_signal(index: u32, signal: Signal);
}

/// Control behavior for container entities.
pub struct LuaContainerControlBehavior {
    pub lua_control_behavior: Box<LuaControlBehavior>,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Control behavior for container entities.
pub trait LuaContainerControlBehaviorMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

pub enum LuaControlOpenedUnion {
    LuaEntity(LuaEntity),
    LuaItemStack(LuaItemStack),
    LuaEquipment(LuaEquipment),
    LuaEquipmentGrid(LuaEquipmentGrid),
    LuaPlayer(LuaPlayer),
    LuaGuiElement(LuaGuiElement),
    LuaInventory(LuaInventory),
    LuaTechnology(LuaTechnology),
    DefinesGuiType(GuiType),
}

pub struct LuaControlMiningState {
    /// Whether the player is mining at all.
    pub mining: bool,
    /// What location the player is mining. Only relevant if `mining` is `true`.
    pub position: Option<MapPosition>,
}

pub struct LuaControlRepairState {
    /// The position being repaired
    pub position: MapPosition,
    /// The current state
    pub repairing: bool,
}

pub struct LuaControlShootingState {
    /// The position being shot at
    pub position: MapPosition,
    /// The current state
    pub state: Shooting,
}

pub struct LuaControlWalkingState {
    /// Direction where the player is walking
    pub direction: Direction,
    /// If `false`, the player is currently not walking; otherwise it's going somewhere
    pub walking: bool,
}

/// This is an abstract base class containing the common functionality between [LuaPlayer](LuaPlayer) and entities (see [LuaEntity](LuaEntity)). When accessing player-related functions through a [LuaEntity](LuaEntity), it must refer to a character entity.
pub struct LuaControl {
    /// The build distance of this character or max uint when not a character or player connected to a character.
    pub build_distance: u32,
    pub character_additional_mining_categories: Vec<String>,
    pub character_build_distance_bonus: u32,
    pub character_crafting_speed_modifier: f64,
    pub character_health_bonus: f32,
    pub character_inventory_slots_bonus: u32,
    pub character_item_drop_distance_bonus: u32,
    pub character_item_pickup_distance_bonus: u32,
    pub character_loot_pickup_distance_bonus: u32,
    pub character_maximum_following_robot_count_bonus: u32,
    /// The current mining progress between 0 and 1 of this character, or 0 if they aren't mining.
    pub character_mining_progress: f64,
    pub character_mining_speed_modifier: f64,
    /// If personal logistic requests are enabled for this character or players character.
    pub character_personal_logistic_requests_enabled: bool,
    pub character_reach_distance_bonus: u32,
    pub character_resource_reach_distance_bonus: u32,
    /// The current movement speed of this character, including effects from exoskeletons, tiles, stickers and shooting.
    pub character_running_speed: f64,
    /// Modifies the running speed of this character by the given value as a percentage. Setting the running modifier to `0.5` makes the character run 50% faster. The minimum value of `-1` reduces the movement speed by 100%, resulting in a speed of `0`.
    pub character_running_speed_modifier: f64,
    pub character_trash_slot_count_bonus: u32,
    /// When `true` hand crafting is free and instant.
    pub cheat_mode: bool,
    /// The current crafting queue items.
    pub crafting_queue: Vec<CraftingQueueItem>,
    /// The crafting queue progress in the range `[0-1]`. `0` when no recipe is being crafted.
    pub crafting_queue_progress: f64,
    /// Size of the crafting queue.
    pub crafting_queue_size: u32,
    /// The ghost prototype in the player's cursor. When read, it will be a [LuaItemPrototype](LuaItemPrototype).
    pub cursor_ghost: Option<ItemPrototypeIdentification>,
    /// The player's cursor stack. `nil` if the player controller is a spectator.
    pub cursor_stack: Option<LuaItemStack>,
    /// `true` if the player is in a vehicle. Writing to this attribute puts the player in or out of a vehicle.
    pub driving: bool,
    /// The item drop distance of this character or max uint when not a character or player connected to a character.
    pub drop_item_distance: u32,
    /// The current combat robots following the character.
    pub following_robots: Vec<LuaEntity>,
    /// The force of this entity. Reading will always give a [LuaForce](LuaForce), but it is possible to assign either [string](string) or [LuaForce](LuaForce) to this attribute to change the force.
    pub force: ForceIdentification,
    /// Unique [index](LuaForce::index) (ID) associated with the force of this entity.
    pub force_index: u32,
    /// Whether this character entity is in combat.
    pub in_combat: bool,
    /// The item pickup distance of this character or max double when not a character or player connected to a character.
    pub item_pickup_distance: f64,
    /// The loot pickup distance of this character or max double when not a character or player connected to a character.
    pub loot_pickup_distance: f64,
    /// Current mining state.
    pub mining_state: LuaControlMiningState,
    /// The GUI the player currently has open.
    ///
    /// This is the GUI that will asked to close (by firing the [on_gui_closed](on_gui_closed) event) when the `Esc` or `E` keys are pressed. If this attribute is not `nil`, and a new GUI is written to it, the existing one will be asked to close.
    pub opened: Option<LuaControlOpenedUnion>,
    pub opened_gui_type: Option<GuiType>,
    /// Current item-picking state.
    pub picking_state: bool,
    /// The current position of the entity.
    pub position: MapPosition,
    /// The reach distance of this character or max uint when not a character or player connected to a character.
    pub reach_distance: u32,
    /// Current repair state.
    pub repair_state: LuaControlRepairState,
    /// The resource reach distance of this character or max double when not a character or player connected to a character.
    pub resource_reach_distance: f64,
    /// Current riding state of this car, or of the car this player is riding in.
    pub riding_state: RidingState,
    /// The currently selected entity. Assigning an entity will select it if is selectable, otherwise the selection is cleared.
    pub selected: Option<LuaEntity>,
    /// Current shooting state.
    pub shooting_state: LuaControlShootingState,
    /// The surface this entity is currently on.
    pub surface: LuaSurface,
    /// Unique [index](LuaSurface::index) (ID) associated with the surface this entity is currently on.
    pub surface_index: u32,
    /// The vehicle the player is currently sitting in.
    pub vehicle: Option<LuaEntity>,
    /// If personal logistic requests are enabled for this vehicle (spidertron).
    pub vehicle_logistic_requests_enabled: bool,
    /// Current walking state.
    pub walking_state: LuaControlWalkingState,
}

pub enum LuaControlMethodsBeginCraftingRecipeUnion {
    String(String),
    LuaRecipe(LuaRecipe),
}

pub enum LuaControlMethodsGetCraftableCountRecipeUnion {
    String(String),
    LuaRecipe(LuaRecipe),
}

/// This is an abstract base class containing the common functionality between [LuaPlayer](LuaPlayer) and entities (see [LuaEntity](LuaEntity)). When accessing player-related functions through a [LuaEntity](LuaEntity), it must refer to a character entity.
pub trait LuaControlMethods {
    /// Begins crafting the given count of the given recipe.
    fn begin_crafting(count: u32, recipe: LuaControlMethodsBeginCraftingRecipeUnion, silent: bool) -> u32;
    /// Can at least some items be inserted?
    fn can_insert(items: ItemStackIdentification) -> bool;
    /// Can a given entity be opened or accessed?
    fn can_reach_entity(entity: LuaEntity) -> bool;
    /// Cancels crafting the given count of the given crafting queue index.
    fn cancel_crafting(count: u32, index: u32);
    /// Removes the arrow created by `set_gui_arrow`.
    fn clear_gui_arrow();
    /// Remove all items from this entity.
    fn clear_items_inside();
    fn clear_personal_logistic_slot(slot_index: u32);
    /// Unselect any selected entity.
    fn clear_selected_entity();
    fn clear_vehicle_logistic_slot(slot_index: u32);
    /// Disable the flashlight.
    fn disable_flashlight();
    /// Enable the flashlight.
    fn enable_flashlight();
    /// Gets the entities that are part of the currently selected blueprint, regardless of it being in a blueprint book or picked from the blueprint library.
    fn get_blueprint_entities() -> Option<Vec<BlueprintEntity>>;
    /// Gets the count of the given recipe that can be crafted.
    fn get_craftable_count(recipe: LuaControlMethodsGetCraftableCountRecipeUnion) -> u32;
    /// Get an inventory belonging to this entity. This can be either the "main" inventory or some auxiliary one, like the module slots or logistic trash slots.
    fn get_inventory(inventory: Inventory) -> Option<LuaInventory>;
    /// Get the number of all or some items in this entity.
    fn get_item_count(item: String) -> u32;
    /// Gets the main inventory for this character or player if this is a character or player.
    fn get_main_inventory() -> Option<LuaInventory>;
    /// The maximum inventory index this entity may use.
    fn get_max_inventory_index() -> Inventory;
    /// Gets the parameters of a personal logistic request and auto-trash slot.
    fn get_personal_logistic_slot(slot_index: u32) -> LogisticParameters;
    /// Gets the parameters of a vehicle logistic request and auto-trash slot. Only used on `spider-vehicle`.
    fn get_vehicle_logistic_slot(slot_index: u32) -> LogisticParameters;
    /// Does this entity have any item inside it?
    fn has_items_inside() -> bool;
    /// Insert items into this entity. This works the same way as inserters or shift-clicking: the "best" inventory is chosen automatically.
    fn insert(items: ItemStackIdentification) -> u32;
    /// Returns whether the player is holding a blueprint. This takes both blueprint items as well as blueprint records from the blueprint library into account.
    ///
    /// Note that both this method and [LuaControl::get_blueprint_entities](LuaControl::get_blueprint_entities) refer to the currently selected blueprint, meaning a blueprint book with a selected blueprint will return the information as well.
    fn is_cursor_blueprint() -> bool;
    /// Returns whether the player is holding something in the cursor. It takes into account items from the blueprint library, as well as items and ghost cursor.
    fn is_cursor_empty() -> bool;
    /// Is the flashlight enabled.
    fn is_flashlight_enabled() -> bool;
    /// When `true` control adapter is a LuaPlayer object, `false` for entities including characters with players.
    fn is_player() -> bool;
    /// Mines the given entity as if this player (or character) mined it.
    fn mine_entity(entity: LuaEntity, force: bool) -> bool;
    /// Mines the given tile as if this player (or character) mined it.
    fn mine_tile(tile: LuaTile) -> bool;
    /// Open the technology GUI and select a given technology.
    fn open_technology_gui(technology: TechnologyIdentification);
    /// Remove items from this entity.
    fn remove_item(items: ItemStackIdentification) -> u32;
    /// Create an arrow which points at this entity. This is used in the tutorial. For examples, see `control.lua` in the campaign missions.
    fn set_gui_arrow(typ: String);
    /// Sets a personal logistic request and auto-trash slot to the given value.
    fn set_personal_logistic_slot(slot_index: u32, value: LogisticParameters) -> bool;
    /// Sets a vehicle logistic request and auto-trash slot to the given value. Only used on `spider-vehicle`.
    fn set_vehicle_logistic_slot(slot_index: u32, value: LogisticParameters) -> bool;
    /// Teleport the entity to a given position, possibly on another surface.
    fn teleport(position: MapPosition, raise_teleported: bool, surface: SurfaceIdentification) -> bool;
    /// Select an entity, as if by hovering the mouse above it.
    fn update_selected_entity(position: MapPosition);
}

/// The control behavior for an entity. Inserters have logistic network and circuit network behavior logic, lamps have circuit logic and so on. This is an abstract base class that concrete control behaviors inherit.
pub struct LuaControlBehavior {
    /// The entity this control behavior belongs to.
    pub entity: LuaEntity,
    /// The concrete type of this control behavior.
    pub typ: ControlBehaviorType,
}

/// The control behavior for an entity. Inserters have logistic network and circuit network behavior logic, lamps have circuit logic and so on. This is an abstract base class that concrete control behaviors inherit.
pub trait LuaControlBehaviorMethods {
    fn get_circuit_network(circuit_connector: CircuitConnectorId, wire: WireType) -> LuaCircuitNetwork;
}

/// A custom tag that shows on the map view.
pub struct LuaCustomChartTag {
    /// The force this tag belongs to.
    pub force: LuaForce,
    /// This tag's icon, if it has one. Writing `nil` removes it.
    pub icon: SignalID,
    /// The player who last edited this tag.
    pub last_user: Option<LuaPlayer>,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The position of this tag.
    pub position: MapPosition,
    /// The surface this tag belongs to.
    pub surface: LuaSurface,
    /// The unique ID for this tag on this force.
    pub tag_number: u32,
    pub text: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// A custom tag that shows on the map view.
pub trait LuaCustomChartTagMethods {
    /// Destroys this tag.
    fn destroy();
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Prototype of a custom input.
pub struct LuaCustomInputPrototype {
    /// The action that happens when this custom input is triggered.
    pub action: String,
    /// The default alternative key sequence for this custom input, if any
    pub alternative_key_sequence: Option<String>,
    /// The consuming type: `"none"` or `"game-only"`.
    pub consuming: String,
    /// Whether this custom input is enabled. Disabled custom inputs exist but are not used by the game.
    pub enabled: bool,
    /// Whether this custom input is enabled while using the cutscene controller.
    pub enabled_while_in_cutscene: bool,
    /// Whether this custom input is enabled while using the spectator controller.
    pub enabled_while_spectating: bool,
    /// Whether this custom input will include the selected prototype (if any) when triggered.
    pub include_selected_prototype: bool,
    /// The item that gets spawned when this custom input is fired, if any.
    pub item_to_spawn: Option<LuaItemPrototype>,
    /// The default key sequence for this custom input.
    pub key_sequence: String,
    /// The linked game control name, if any.
    pub linked_game_control: Option<String>,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    /// Name of this prototype.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of a custom input.
pub trait LuaCustomInputPrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Lazily evaluated table. For performance reasons, we sometimes return a custom table-like type instead of a native Lua table. This custom type lazily constructs the necessary Lua wrappers of the corresponding C++ objects, therefore preventing their unnecessary construction in some cases.
///
/// There are some notable consequences to the usage of a custom table type rather than the native Lua table type: Iterating a custom table is only possible using the `pairs` Lua function; `ipairs` won't work. Another key difference is that custom tables cannot be serialised into a game save file -- if saving the game would require serialisation of a custom table, an error will be displayed and the game will not be saved.
pub struct LuaCustomTable {
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Lazily evaluated table. For performance reasons, we sometimes return a custom table-like type instead of a native Lua table. This custom type lazily constructs the necessary Lua wrappers of the corresponding C++ objects, therefore preventing their unnecessary construction in some cases.
///
/// There are some notable consequences to the usage of a custom table type rather than the native Lua table type: Iterating a custom table is only possible using the `pairs` Lua function; `ipairs` won't work. Another key difference is that custom tables cannot be serialised into a game save file -- if saving the game would require serialisation of a custom table, an error will be displayed and the game will not be saved.
pub trait LuaCustomTableMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Prototype of a damage.
pub struct LuaDamagePrototype {
    /// Whether this damage type is hidden from entity tooltips.
    pub hidden: bool,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    /// Name of this prototype.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of a damage.
pub trait LuaDamagePrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Control behavior for decider combinators.
pub struct LuaDeciderCombinatorControlBehavior {
    pub lua_combinator_control_behavior: Box<LuaCombinatorControlBehavior>,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// This decider combinator's parameters.
    pub parameters: DeciderCombinatorParameters,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Control behavior for decider combinators.
pub trait LuaDeciderCombinatorControlBehaviorMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Prototype of an optimized decorative.
pub struct LuaDecorativePrototype {
    /// Autoplace specification for this decorative prototype, if any.
    pub autoplace_specification: Option<AutoplaceSpecification>,
    /// The bounding box used for collision checking.
    pub collision_box: BoundingBox,
    /// The collision masks this decorative uses
    pub collision_mask: CollisionMask,
    pub collision_mask_with_flags: CollisionMaskWithFlags,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    /// Name of this prototype.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of an optimized decorative.
pub trait LuaDecorativePrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Prototype of an electric energy source.
pub struct LuaElectricEnergySourcePrototype {
    pub buffer_capacity: f64,
    pub drain: f64,
    /// The emissions of this energy source in `pollution/Joule`. Multiplying it by energy consumption in `Watt` gives `pollution/second`.
    pub emissions: f64,
    pub input_flow_limit: f64,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    pub output_flow_limit: f64,
    pub render_no_network_icon: bool,
    pub render_no_power_icon: bool,
    pub usage_priority: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of an electric energy source.
pub trait LuaElectricEnergySourcePrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

pub enum LuaEntityAssociatedPlayerUnion {
    LuaPlayer(LuaPlayer),
    PlayerIdentification(PlayerIdentification),
}

pub enum LuaEntityGhostPrototypeUnion {
    LuaEntityPrototype(LuaEntityPrototype),
    LuaTilePrototype(LuaTilePrototype),
}

pub enum LuaEntityLastUserUnion {
    LuaPlayer(LuaPlayer),
    PlayerIdentification(PlayerIdentification),
}

pub enum LuaEntityNeighboursUnion {
    Dictionary(HashMap<String, Vec<LuaEntity>>),
    Array(Vec<Vec<LuaEntity>>),
    LuaEntity(LuaEntity),
}

pub enum LuaEntityRenderPlayerUnion {
    LuaPlayer(LuaPlayer),
    PlayerIdentification(PlayerIdentification),
}

pub struct LuaEntityCircuitConnectedEntities {
    /// Entities connected via the green wire.
    pub green: Vec<LuaEntity>,
    /// Entities connected via the red wire.
    pub red: Vec<LuaEntity>,
}

/// The primary interface for interacting with entities through the Lua API. Entities are everything that exists on the map except for tiles (see [LuaTile](LuaTile)).
///
/// Most functions on LuaEntity also work when the entity is contained in a ghost.
pub struct LuaEntity {
    pub lua_control: Box<LuaControl>,
    /// Deactivating an entity will stop all its operations (car will stop moving, inserters will stop working, fish will stop moving etc).
    pub active: bool,
    /// The ai settings of this unit.
    pub ai_settings: LuaAISettings,
    pub alert_parameters: ProgrammableSpeakerAlertParameters,
    /// Whether this character's personal roboports are allowed to dispatch robots.
    pub allow_dispatching_robots: bool,
    /// Count of resource units contained.
    pub amount: u32,
    /// Whether this land mine is armed.
    pub armed: bool,
    /// The player this character is associated with, if any. Set to `nil` to clear.
    ///
    /// The player will be automatically disassociated when a controller is set on the character. Also, all characters associated to a player will be logged off when the player logs off in multiplayer.
    ///
    /// Reading this property will return a [LuaPlayer](LuaPlayer), while [PlayerIdentification](PlayerIdentification) can be used when writing.
    pub associated_player: Option<LuaEntityAssociatedPlayerUnion>,
    /// Whether this rocket silo automatically launches the rocket when cargo is inserted.
    pub auto_launch: bool,
    /// Destination of this spidertron's autopilot, if any. Writing `nil` clears all destinations.
    pub autopilot_destination: Option<MapPosition>,
    /// The queued destination positions of spidertron's autopilot.
    pub autopilot_destinations: Vec<MapPosition>,
    /// The backer name assigned to this entity. Entities that support backer names are labs, locomotives, radars, roboports, and train stops. `nil` if this entity doesn't support backer names.
    pub backer_name: Option<String>,
    /// Number of beacons affecting this effect receiver. Can only be used when the entity has an effect receiver (AssemblingMachine, Furnace, Lab, MiningDrills)
    pub beacons_count: Option<u32>,
    /// The belt connectable neighbours of this belt connectable entity. Only entities that input to or are outputs of this entity. Does not contain the other end of an underground belt, see [LuaEntity::neighbours](LuaEntity::neighbours) for that. This is a dictionary with `"inputs"`, `"outputs"` entries that are arrays of transport belt connectable entities, or empty tables if no entities.
    pub belt_neighbours: HashMap<String, Vec<LuaEntity>>,
    /// `"input"` or `"output"`, depending on whether this underground belt goes down or up.
    pub belt_to_ground_type: String,
    /// The bonus mining progress for this mining drill. Read yields a number in range [0, mining_target.prototype.mineable_properties.mining_time]. `nil` if this isn't a mining drill.
    pub bonus_mining_progress: Option<f64>,
    /// The current productivity bonus progress, as a number in range [0, 1].
    pub bonus_progress: f64,
    /// [LuaEntityPrototype::collision_box](LuaEntityPrototype::collision_box) around entity's given position and respecting the current entity orientation.
    pub bounding_box: BoundingBox,
    /// The burner energy source for this entity, if any.
    pub burner: Option<Box<LuaBurner>>,
    /// The state of this chain signal.
    pub chain_signal_state: ChainSignalState,
    /// The reason this character corpse character died. `""` if there is no reason.
    pub character_corpse_death_cause: LocalisedString,
    /// The player index associated with this character corpse.
    pub character_corpse_player_index: u32,
    /// The tick this character corpse died at.
    pub character_corpse_tick_of_death: u32,
    /// Entities that are directly connected to this entity via the circuit network. `nil` if this entity can't be connected to the circuit network.
    pub circuit_connected_entities: Option<LuaEntityCircuitConnectedEntities>,
    /// The connection definition for entities that are directly connected to this entity via the circuit network. `nil` if this entity can't be connected to the circuit network.
    pub circuit_connection_definitions: Option<Vec<CircuitConnectionDefinition>>,
    /// The orientation of this cliff.
    pub cliff_orientation: CliffOrientation,
    /// The color of this character, rolling stock, train stop, car, spider-vehicle, flying text, corpse or simple-entity-with-owner. `nil` if this entity doesn't use custom colors.
    pub color: Option<Color>,
    /// The owner of this combat robot, if any.
    pub combat_robot_owner: Option<Box<LuaEntity>>,
    /// The command given to this unit, if any.
    pub command: Option<Box<Command>>,
    /// The rail entity this train stop is connected to, if any.
    pub connected_rail: Option<Box<LuaEntity>>,
    /// Rail direction to which this train stop is binding. This returns a value even when no rails are present.
    pub connected_rail_direction: RailDirection,
    /// The consumption bonus of this entity.
    pub consumption_bonus: f64,
    /// Multiplies the energy consumption.
    pub consumption_modifier: f32,
    /// Whether this corpse will ever fade away.
    pub corpse_expires: bool,
    /// If true, corpse won't be destroyed when entities are placed over it. If false, whether corpse will be removed or not depends on value of CorpsePrototype::remove_on_entity_placement.
    pub corpse_immune_to_entity_placement: bool,
    /// The current crafting progress, as a number in range [0, 1].
    pub crafting_progress: f32,
    /// The current crafting speed, including speed bonuses from modules and beacons.
    pub crafting_speed: f64,
    /// The damage dealt by this turret, artillery turret, or artillery wagon.
    pub damage_dealt: f64,
    /// If set to `false`, this entity can't be damaged and won't be attacked automatically. It can however still be mined.
    pub destructible: bool,
    /// The current direction this entity is facing.
    pub direction: Direction,
    /// The distraction command given to this unit, if any.
    pub distraction_command: Option<Box<Command>>,
    /// Whether the driver of this car or spidertron is the gunner. If `false`, the passenger is the gunner. `nil` if this is neither a car or a spidertron.
    pub driver_is_gunner: Option<bool>,
    /// Position where the entity puts its stuff.
    pub drop_position: MapPosition,
    /// The entity this entity is putting its items to. If there are multiple possible entities at the drop-off point, writing to this attribute allows a mod to choose which one to drop off items to. The entity needs to collide with the tile box under the drop-off position. `nil` if there is no entity to put items to, or if this is not an entity that puts items somewhere.
    pub drop_target: Option<Box<LuaEntity>>,
    /// The current speed of this unit in tiles per tick, taking into account any walking speed modifier given by the tile the unit is standing on. `nil` if this is not a unit.
    pub effective_speed: Option<f32>,
    /// Multiplies the acceleration the vehicle can create for one unit of energy. Defaults to `1`.
    pub effectivity_modifier: f32,
    /// The effects being applied to this entity, if any. For beacons, this is the effect the beacon is broadcasting.
    pub effects: Option<ModuleEffects>,
    /// The buffer size for the electric energy source. `nil` if the entity doesn't have an electric energy source.
    pub electric_buffer_size: Option<f64>,
    /// The electric drain for the electric energy source. `nil` if the entity doesn't have an electric energy source.
    pub electric_drain: Option<f64>,
    /// The emissions for the electric energy source. `nil` if the entity doesn't have an electric energy source.
    pub electric_emissions: Option<f64>,
    /// The input flow limit for the electric energy source. `nil` if the entity doesn't have an electric energy source.
    pub electric_input_flow_limit: Option<f64>,
    /// Returns the id of the electric network that this entity is connected to, if any.
    pub electric_network_id: Option<u32>,
    /// The electric network statistics for this electric pole.
    pub electric_network_statistics: LuaFlowStatistics,
    /// The output flow limit for the electric energy source. `nil` if the entity doesn't have an electric energy source.
    pub electric_output_flow_limit: Option<f64>,
    /// Whether equipment grid logistics are enabled while this vehicle is moving.
    pub enable_logistics_while_moving: bool,
    /// Energy stored in the entity (heat in furnace, energy stored in electrical devices etc.). always 0 for entities that don't have the concept of energy stored inside.
    pub energy: f64,
    /// How much energy this generator generated in the last tick.
    pub energy_generated_last_tick: f64,
    /// The label on this spider-vehicle entity, if any. `nil` if this is not a spider-vehicle.
    pub entity_label: Option<String>,
    /// The number of filter slots this inserter, loader, or logistic storage container has. 0 if not one of those entities.
    pub filter_slot_count: u32,
    /// Fluidboxes of this entity.
    pub fluidbox: LuaFluidBox,
    /// The follow offset of this spidertron, if any entity is being followed. This is randomized each time the follow entity is set.
    pub follow_offset: Option<Vector>,
    /// The follow target of this spidertron, if any.
    pub follow_target: Option<Box<LuaEntity>>,
    /// Multiplies the car friction rate.
    pub friction_modifier: f32,
    pub ghost_localised_description: LocalisedString,
    /// Localised name of the entity or tile contained in this ghost.
    pub ghost_localised_name: LocalisedString,
    /// Name of the entity or tile contained in this ghost
    pub ghost_name: String,
    /// The prototype of the entity or tile contained in this ghost.
    pub ghost_prototype: LuaEntityGhostPrototypeUnion,
    /// The prototype type of the entity or tile contained in this ghost.
    pub ghost_type: String,
    /// The [unit_number](LuaEntity::unit_number) of the entity contained in this ghost. It is the same as the unit number of the [EntityWithOwner](https://wiki.factorio.com/Prototype/EntityWithOwner) that was destroyed to create this ghost. If it was created by other means, or if the inner entity does not support unit numbers, this property is `nil`.
    pub ghost_unit_number: Option<u32>,
    /// The graphics variation for this entity. `nil` if this entity doesn't use graphics variations.
    pub graphics_variation: Option<u8>,
    /// This entity's equipment grid, if any.
    pub grid: Option<LuaEquipmentGrid>,
    /// The current health of the entity, if any. Health is automatically clamped to be between `0` and max health (inclusive). Entities with a health of `0` can not be attacked.
    pub health: Option<f32>,
    /// The item stack currently held in an inserter's hand.
    pub held_stack: LuaItemStack,
    /// Current position of the inserter's "hand".
    pub held_stack_position: MapPosition,
    /// The blink interval of this highlight box entity. 0 indicates no blink.
    pub highlight_box_blink_interval: u32,
    /// The hightlight box type of this highlight box entity.
    pub highlight_box_type: String,
    /// The filters for this infinity container.
    pub infinity_container_filters: Vec<InfinityInventoryFilter>,
    /// Count of initial resource units contained. `nil` if this is not an infinite resource.
    pub initial_amount: Option<u32>,
    /// The filter mode for this filter inserter. Either `"whitelist"` or `"blacklist"`. `nil` if this inserter doesn't use filters.
    pub inserter_filter_mode: Option<String>,
    /// Sets the stack size limit on this inserter. If the stack size is > than the force stack size limit the value is ignored.
    pub inserter_stack_size_override: u32,
    /// Returns the current target pickup count of the inserter.
    pub inserter_target_pickup_count: u32,
    /// (deprecated by 1.1.51) If this entity is a MilitaryTarget. Returns same value as LuaEntity::is_military_target
    pub is_entity_with_force: bool,
    /// If this entity is EntityWithHealth
    pub is_entity_with_health: bool,
    /// If this entity is EntityWithOwner
    pub is_entity_with_owner: bool,
    /// Whether this entity is a MilitaryTarget. Can be written to if [LuaEntityPrototype::allow_run_time_change_of_is_military_target](LuaEntityPrototype::allow_run_time_change_of_is_military_target) returns `true`.
    pub is_military_target: bool,
    /// Items this ghost will request when revived or items this item request proxy is requesting. Result is a dictionary mapping each item prototype name to the required count.
    pub item_requests: HashMap<String, u32>,
    /// The number of units killed by this turret, artillery turret, or artillery wagon.
    pub kills: u32,
    /// The last player that changed any setting on this entity. This includes building the entity, changing its color, or configuring its circuit network. `nil` if the last user is not part of the save anymore.
    ///
    /// Reading this property will return a [LuaPlayer](LuaPlayer), while [PlayerIdentification](PlayerIdentification) can be used when writing.
    pub last_user: Option<LuaEntityLastUserUnion>,
    /// The link ID this linked container is using.
    pub link_id: u32,
    /// Neighbour to which this linked belt is connected to, if any.
    pub linked_belt_neighbour: Option<Box<LuaEntity>>,
    /// Type of linked belt: it is either `"input"` or `"output"`. Changing type will also flip direction so the belt is out of the same side
    pub linked_belt_type: String,
    /// The container entity this loader is pointing at/pulling from depending on the [LuaEntity::loader_type](LuaEntity::loader_type), if any.
    pub loader_container: Option<Box<LuaEntity>>,
    /// `"input"` or `"output"`, depending on whether this loader puts to or gets from a container.
    pub loader_type: String,
    pub localised_description: LocalisedString,
    /// Localised name of the entity.
    pub localised_name: LocalisedString,
    /// The logistic cell this entity is a part of. Will be `nil` if this entity is not a part of any logistic cell.
    pub logistic_cell: LuaLogisticCell,
    /// The logistic network this entity is a part of, or `nil` if this entity is not a part of any logistic network.
    pub logistic_network: LuaLogisticNetwork,
    pub minable: bool,
    /// The mining progress for this mining drill. Is a number in range [0, mining_target.prototype.mineable_properties.mining_time]. `nil` if this isn't a mining drill.
    pub mining_progress: Option<f64>,
    /// The mining target, if any.
    pub mining_target: Option<Box<LuaEntity>>,
    /// Returns true if this unit is moving.
    pub moving: bool,
    /// Name of the entity prototype. E.g. "inserter" or "filter-inserter".
    pub name: String,
    /// The current total neighbour bonus of this reactor.
    pub neighbour_bonus: f64,
    /// A list of neighbours for certain types of entities. Applies to electric poles, power switches, underground belts, walls, gates, reactors, cliffs, and pipe-connectable entities.
    ///
    /// - When called on an electric pole, this is a dictionary of all connections, indexed by the strings `"copper"`, `"red"`, and `"green"`.
    /// - When called on a pipe-connectable entity, this is an array of entity arrays of all entities a given fluidbox is connected to.
    /// - When called on an underground transport belt, this is the other end of the underground belt connection, or `nil` if none.
    /// - When called on a wall-connectable entity or reactor, this is a dictionary of all connections indexed by the connection direction "north", "south", "east", and "west".
    /// - When called on a cliff entity, this is a dictionary of all connections indexed by the connection direction "north", "south", "east", and "west".
    pub neighbours: Box<LuaEntityNeighboursUnion>,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// Player can't open gui of this entity and he can't quick insert/input stuff in to the entity when it is not operable.
    pub operable: bool,
    /// The smooth orientation of this entity.
    pub orientation: RealOrientation,
    pub parameters: ProgrammableSpeakerParameters,
    /// Where the inserter will pick up items from.
    pub pickup_position: MapPosition,
    /// The entity this inserter will attempt to pick up items from. If there are multiple possible entities at the pick-up point, writing to this attribute allows a mod to choose which one to pick up items from. The entity needs to collide with the tile box under the pick-up position. `nil` if there is no entity to pull items from.
    pub pickup_target: Option<Box<LuaEntity>>,
    /// The player connected to this character, if any.
    pub player: Option<LuaPlayer>,
    /// The pollution bonus of this entity.
    pub pollution_bonus: f64,
    /// The power production specific to the ElectricEnergyInterface entity type.
    pub power_production: f64,
    /// The state of this power switch.
    pub power_switch_state: bool,
    /// The power usage specific to the ElectricEnergyInterface entity type.
    pub power_usage: f64,
    /// The previous recipe this furnace was using, if any.
    pub previous_recipe: Option<LuaRecipe>,
    /// The productivity bonus of this entity.
    pub productivity_bonus: f64,
    /// The number of products this machine finished crafting in its lifetime.
    pub products_finished: u32,
    /// The entity prototype of this entity.
    pub prototype: LuaEntityPrototype,
    /// The target entity for this item-request-proxy, if any.
    pub proxy_target: Option<Box<LuaEntity>>,
    /// The rail target of this pump, if any.
    pub pump_rail_target: Option<Box<LuaEntity>>,
    /// The current radar scan progress, as a number in range [0, 1].
    pub radar_scan_progress: f32,
    /// When locked; the recipe in this assembling machine can't be changed by the player.
    pub recipe_locked: bool,
    /// The relative orientation of the vehicle turret, artillery turret, artillery wagon. `nil` if this entity isn't a vehicle with a vehicle turret or artillery turret/wagon.
    pub relative_turret_orientation: Option<RealOrientation>,
    /// Whether items not included in this infinity container filters should be removed from the container.
    pub remove_unfiltered_items: bool,
    /// The player that this `simple-entity-with-owner`, `simple-entity-with-force`, `flying-text`, or `highlight-box` is visible to. `nil` when this entity is rendered for all players.
    ///
    /// Reading this property will return a [LuaPlayer](LuaPlayer), while [PlayerIdentification](PlayerIdentification) can be used when writing.
    pub render_player: Option<LuaEntityRenderPlayerUnion>,
    /// The forces that this `simple-entity-with-owner`, `simple-entity-with-force`, or `flying-text` is visible to. `nil` or an empty array when this entity is rendered for all forces.
    pub render_to_forces: Option<Vec<ForceIdentification>>,
    /// Whether this requester chest is set to also request from buffer chests.
    pub request_from_buffers: bool,
    /// The index of the configured request with the highest index for this entity. This means 0 if no requests are set and e.g. 20 if the 20th request slot is configured.
    pub request_slot_count: u32,
    /// Number of rocket parts in the silo.
    pub rocket_parts: u32,
    /// The status of this rocket silo entity.
    pub rocket_silo_status: RocketSiloStatus,
    /// When entity is not to be rotatable (inserter, transport belt etc), it can't be rotated by player using the R key.
    pub rotatable: bool,
    /// The secondary bounding box of this entity or `nil` if it doesn't have one. This only exists for curved rails, and is automatically determined by the game.
    pub secondary_bounding_box: Option<BoundingBox>,
    /// The secondary selection box of this entity or `nil` if it doesn't have one. This only exists for curved rails, and is automatically determined by the game.
    pub secondary_selection_box: Option<BoundingBox>,
    /// Index of the currently selected weapon slot of this character, car, or spidertron. `nil` if this entity doesn't have guns.
    pub selected_gun_index: Option<u32>,
    /// [LuaEntityPrototype::selection_box](LuaEntityPrototype::selection_box) around entity's given position and respecting the current entity orientation.
    pub selection_box: BoundingBox,
    /// The shooting target for this turret, if any. Can't be set to `nil` via script.
    pub shooting_target: Option<Box<LuaEntity>>,
    /// The state of this rail signal.
    pub signal_state: SignalState,
    /// The spawner associated with this unit entity, if any.
    pub spawner: Option<Box<LuaEntity>>,
    /// The current speed if this is a car, rolling stock, projectile or spidertron, or the maximum speed if this is a unit. The speed is in tiles per tick. `nil` if this is not a car, rolling stock, unit, projectile or spidertron.
    ///
    /// Only the speed of units, cars, and projectiles are writable.
    pub speed: Option<f32>,
    /// The speed bonus of this entity.
    pub speed_bonus: f64,
    /// The filter for this splitter, if any is set.
    pub splitter_filter: Option<LuaItemPrototype>,
    /// The input priority for this splitter. Either `"left"`, `"none"`, or `"right"`.
    pub splitter_input_priority: String,
    /// The output priority for this splitter. Either `"left"`, `"none"`, or `"right"`.
    pub splitter_output_priority: String,
    pub stack: LuaItemStack,
    /// The status of this entity, if any.
    pub status: Option<EntityStatus>,
    /// The entity this sticker is sticked to.
    pub sticked_to: Box<LuaEntity>,
    /// The sticker entities attached to this entity, if any.
    pub stickers: Option<Vec<LuaEntity>>,
    /// The storage filter for this logistic storage container.
    pub storage_filter: LuaItemPrototype,
    /// Whether the entity has direction. When it is false for this entity, it will always return north direction when asked for.
    pub supports_direction: bool,
    /// The tags associated with this entity ghost. `nil` if this is not an entity ghost.
    pub tags: Option<Tags>,
    /// The temperature of this entity's heat energy source. `nil` if this entity does not use a heat energy source.
    pub temperature: Option<f64>,
    /// The text of this flying-text entity.
    pub text: LocalisedString,
    /// The last tick this character entity was attacked.
    pub tick_of_last_attack: u32,
    /// The last tick this character entity was damaged.
    pub tick_of_last_damage: u32,
    /// Specifies the tiling size of the entity, is used to decide, if the center should be in the center of the tile (odd tile size dimension) or on the tile border (even tile size dimension). Uses the current direction of the entity.
    pub tile_height: u32,
    /// Specifies the tiling size of the entity, is used to decide, if the center should be in the center of the tile (odd tile size dimension) or on the tile border (even tile size dimension). Uses the current direction of the entity.
    pub tile_width: u32,
    /// The ticks left before a ghost, combat robot, highlight box or smoke with trigger is destroyed.
    ///
    /// - for ghosts set to uint32 max (4,294,967,295) to never expire.
    /// - for ghosts Cannot be set higher than [LuaForce::ghost_time_to_live](LuaForce::ghost_time_to_live) of the entity's force.
    pub time_to_live: u32,
    /// The ticks until the next trigger effect of this smoke-with-trigger.
    pub time_to_next_effect: u32,
    /// The timeout that's left on this landmine in ticks. It describes the time between the landmine being placed and it being armed.
    pub timeout: u32,
    /// Will this entity be picked up automatically when the player walks over it?
    pub to_be_looted: bool,
    /// The torso orientation of this spider vehicle.
    pub torso_orientation: RealOrientation,
    /// The train this rolling stock belongs to, if any. `nil` if this is not a rolling stock.
    pub train: Option<Box<LuaTrain>>,
    /// Amount of trains related to this particular train stop. Includes train stopped at this train stop (until it finds a path to next target) and trains having this train stop as goal or waypoint.
    pub trains_count: u32,
    /// The number of trains in this rail block for this rail entity.
    pub trains_in_block: u32,
    /// Amount of trains above which no new trains will be sent to this train stop. Writing nil will disable the limit (will set a maximum possible value).
    pub trains_limit: u32,
    /// Index of the tree color.
    pub tree_color_index: u8,
    /// Maximum index of the tree colors.
    pub tree_color_index_max: u8,
    /// Index of the tree gray stage
    pub tree_gray_stage_index: u8,
    /// Maximum index of the tree gray stages.
    pub tree_gray_stage_index_max: u8,
    /// Index of the tree stage.
    pub tree_stage_index: u8,
    /// Maximum index of the tree stages.
    pub tree_stage_index_max: u8,
    /// The entity prototype type of this entity.
    pub typ: String,
    /// The unit group this unit is a member of, if any.
    pub unit_group: Option<Box<LuaUnitGroup>>,
    /// A unique number identifying this entity for the lifetime of the save. These are allocated sequentially, and not re-used (until overflow).
    ///
    /// Only entities inheriting from [EntityWithOwner](https://wiki.factorio.com/Prototype/EntityWithOwner), as well as [ItemRequestProxy](https://wiki.factorio.com/Prototype/ItemRequestProxy) and [EntityGhost](https://wiki.factorio.com/Prototype/EntityGhost) are assigned a unit number. Returns `nil` otherwise.
    pub unit_number: Option<u32>,
    /// The units associated with this spawner entity.
    pub units: Vec<LuaEntity>,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
    /// Read when this spidertron auto-targets enemies
    pub vehicle_automatic_targeting_parameters: VehicleAutomaticTargetingParameters,
}

pub enum LuaEntityMethodsConnectNeighbourTargetUnion {
    LuaEntity(LuaEntity),
    WireConnectionDefinition(WireConnectionDefinition),
}

pub enum LuaEntityMethodsDisconnectNeighbourTargetUnion {
    DefinesWireType(WireType),
    LuaEntity(LuaEntity),
    WireConnectionDefinition(WireConnectionDefinition),
}

pub enum LuaEntityMethodsGetDriverUnion {
    LuaEntity(LuaEntity),
    LuaPlayer(LuaPlayer),
}

pub enum LuaEntityMethodsGetLogisticPointUnion {
    LuaLogisticPoint(LuaLogisticPoint),
    Array(Vec<LuaLogisticPoint>),
}

pub enum LuaEntityMethodsGetPassengerUnion {
    LuaEntity(LuaEntity),
    LuaPlayer(LuaPlayer),
}

pub enum LuaEntityMethodsRotateForceUnion {
    LuaForce(LuaForce),
    String(String),
}

pub enum LuaEntityMethodsSetBeamSourceSourceUnion {
    LuaEntity(LuaEntity),
    MapPosition(MapPosition),
}

pub enum LuaEntityMethodsSetBeamTargetTargetUnion {
    LuaEntity(LuaEntity),
    MapPosition(MapPosition),
}

pub enum LuaEntityMethodsSetDriverDriverUnion {
    LuaEntity(LuaEntity),
    PlayerIdentification(PlayerIdentification),
}

pub enum LuaEntityMethodsSetFilterItemUnion {
    String(String),
    Nil,
}

pub enum LuaEntityMethodsSetInfinityContainerFilterFilterUnion {
    InfinityInventoryFilter(InfinityInventoryFilter),
    Nil,
}

pub enum LuaEntityMethodsSetInfinityPipeFilterFilterUnion {
    InfinityPipeFilter(InfinityPipeFilter),
    Nil,
}

pub enum LuaEntityMethodsSetPassengerPassengerUnion {
    LuaEntity(LuaEntity),
    PlayerIdentification(PlayerIdentification),
}

pub enum LuaEntityMethodsSetRecipeRecipeUnion {
    String(String),
    LuaRecipe(LuaRecipe),
}

/// The primary interface for interacting with entities through the Lua API. Entities are everything that exists on the map except for tiles (see [LuaTile](LuaTile)).
///
/// Most functions on LuaEntity also work when the entity is contained in a ghost.
pub trait LuaEntityMethods {
    /// Adds the given position to this spidertron's autopilot's queue of destinations.
    fn add_autopilot_destination(position: MapPosition);
    /// Offer a thing on the market.
    fn add_market_item(offer: Offer);
    /// Checks if the entity can be destroyed
    fn can_be_destroyed() -> bool;
    /// Whether this character can shoot the given entity or position.
    fn can_shoot(position: MapPosition, target: LuaEntity) -> bool;
    /// Can wires reach between these entities.
    fn can_wires_reach(entity: LuaEntity) -> bool;
    /// Cancels deconstruction if it is scheduled, does nothing otherwise.
    fn cancel_deconstruction(force: ForceIdentification, player: PlayerIdentification);
    /// Cancels upgrade if it is scheduled, does nothing otherwise.
    fn cancel_upgrade(force: ForceIdentification, player: PlayerIdentification) -> bool;
    /// Remove all fluids from this entity.
    fn clear_fluid_inside();
    /// Removes all offers from a market.
    fn clear_market_items();
    /// Clear a logistic requester slot.
    fn clear_request_slot(slot: u32);
    /// Clones this entity.
    fn clone(create_build_effect_smoke: bool, force: ForceIdentification, position: MapPosition, surface: LuaSurface) -> Option<LuaEntity>;
    /// Connects current linked belt with another one.
    ///
    /// Neighbours have to be of different type. If given linked belt is connected to something else it will be disconnected first. If provided neighbour is connected to something else it will also be disconnected first. Automatically updates neighbour to be connected back to this one.
    fn connect_linked_belts(neighbour: LuaEntity);
    /// Connect two devices with a circuit wire or copper cable. Depending on which type of connection should be made, there are different procedures:
    ///
    /// - To connect two electric poles, `target` must be a [LuaEntity](LuaEntity) that specifies another electric pole. This will connect them with copper cable.
    /// - To connect two devices with circuit wire, `target` must be a table of type [WireConnectionDefinition](WireConnectionDefinition).
    fn connect_neighbour(target: LuaEntityMethodsConnectNeighbourTargetUnion) -> bool;
    /// Connects the rolling stock in the given direction.
    fn connect_rolling_stock(direction: RailDirection) -> bool;
    /// Copies settings from the given entity onto this entity.
    fn copy_settings(by_player: PlayerIdentification, entity: LuaEntity) -> HashMap<String, u32>;
    /// Creates the same smoke that is created when you place a building by hand. You can play the building sound to go with it by using [LuaSurface::play_sound](LuaSurface::play_sound), eg: entity.surface.play_sound{path="entity-build/"..entity.prototype.name, position=entity.position}
    fn create_build_effect_smoke();
    /// Damages the entity.
    fn damage(damage: f32, dealer: LuaEntity, force: ForceIdentification, typ: String) -> f32;
    /// Depletes and destroys this resource entity.
    fn deplete();
    /// Destroys the entity.
    fn destroy(do_cliff_correction: bool, raise_destroy: bool) -> bool;
    /// Immediately kills the entity. Does nothing if the entity doesn't have health.
    ///
    /// Unlike [LuaEntity::destroy](LuaEntity::destroy), `die` will trigger the [on_entity_died](on_entity_died) event and the entity will produce a corpse and drop loot if it has any.
    fn die(cause: LuaEntity, force: ForceIdentification) -> bool;
    /// Disconnects linked belt from its neighbour.
    fn disconnect_linked_belts();
    /// Disconnect circuit wires or copper cables between devices. Depending on which type of connection should be cut, there are different procedures:
    ///
    /// - To remove all copper cables, leave the `target` parameter blank: `pole.disconnect_neighbour()`.
    /// - To remove all wires of a specific color, set `target` to [defines.wire_type.red](defines.wire_type.red) or [defines.wire_type.green](defines.wire_type.green).
    /// - To remove a specific copper cable between two electric poles, `target` must be a [LuaEntity](LuaEntity) that specifies the other pole: `pole1.disconnect_neighbour(pole2)`.
    /// - To remove a specific circuit wire, `target` must be a table of type [WireConnectionDefinition](WireConnectionDefinition).
    fn disconnect_neighbour(target: LuaEntityMethodsDisconnectNeighbourTargetUnion);
    /// Tries to disconnect this rolling stock in the given direction.
    fn disconnect_rolling_stock(direction: RailDirection) -> bool;
    /// Returns a table with all entities affected by this beacon
    fn get_beacon_effect_receivers() -> Vec<LuaEntity>;
    /// Returns a table with all beacons affecting this effect receiver. Can only be used when the entity has an effect receiver (AssemblingMachine, Furnace, Lab, MiningDrills)
    fn get_beacons() -> Option<Vec<LuaEntity>>;
    /// Get the source of this beam.
    fn get_beam_source() -> Option<BeamTarget>;
    /// Get the target of this beam.
    fn get_beam_target() -> Option<BeamTarget>;
    /// The burnt result inventory for this entity or `nil` if this entity doesn't have a burnt result inventory.
    fn get_burnt_result_inventory() -> Option<LuaInventory>;
    /// Returns all child signals. Child signals can be either RailSignal or RailChainSignal. Child signals are signals which are checked by this signal to determine a chain state.
    fn get_child_signals() -> Vec<LuaEntity>;
    fn get_circuit_network(circuit_connector: CircuitConnectorId, wire: WireType) -> Option<LuaCircuitNetwork>;
    fn get_connected_rail(rail_connection_direction: RailConnectionDirection, rail_direction: RailDirection) -> (Option<LuaEntity>, Option<RailDirection>, Option<RailConnectionDirection>);
    /// Get the rails that this signal is connected to.
    fn get_connected_rails() -> Vec<LuaEntity>;
    /// Gets rolling stock connected to the given end of this stock.
    fn get_connected_rolling_stock(direction: RailDirection) -> (Option<LuaEntity>, Option<RailDirection>);
    /// Gets the control behavior of the entity (if any).
    fn get_control_behavior() -> Option<LuaControlBehavior>;
    /// Returns the amount of damage to be taken by this entity.
    fn get_damage_to_be_taken() -> Option<f32>;
    /// Gets the driver of this vehicle if any.
    fn get_driver() -> Option<LuaEntityMethodsGetDriverUnion>;
    /// Get the filter for a slot in an inserter, loader, or logistic storage container.
    fn get_filter(slot_index: u32) -> Option<String>;
    /// Get amounts of all fluids in this entity.
    fn get_fluid_contents() -> HashMap<String, f64>;
    /// Get the amount of all or some fluid in this entity.
    fn get_fluid_count(fluid: String) -> f64;
    /// The fuel inventory for this entity or `nil` if this entity doesn't have a fuel inventory.
    fn get_fuel_inventory() -> Option<LuaInventory>;
    /// The health ratio of this entity between 1 and 0 (for full health and no health respectively).
    fn get_health_ratio() -> Option<f32>;
    /// Gets the heat setting for this heat interface.
    fn get_heat_setting() -> HeatSetting;
    /// Returns all signals guarding entrance to a rail block this rail belongs to.
    fn get_inbound_signals() -> Vec<LuaEntity>;
    /// Gets the filter for this infinity container at the given index, or `nil` if the filter index doesn't exist or is empty.
    fn get_infinity_container_filter(index: u32) -> Option<InfinityInventoryFilter>;
    /// Gets the filter for this infinity pipe, or `nil` if the filter is empty.
    fn get_infinity_pipe_filter() -> Option<InfinityPipeFilter>;
    /// Gets all the `LuaLogisticPoint`s that this entity owns. Optionally returns only the point specified by the index parameter.
    fn get_logistic_point(index: LogisticMemberIndex) -> Option<LuaEntityMethodsGetLogisticPointUnion>;
    /// Get all offers in a market as an array.
    fn get_market_items() -> Vec<Offer>;
    /// Get the maximum transport line index of a belt or belt connectable entity.
    fn get_max_transport_line_index() -> u32;
    /// Read a single signal from the combined circuit networks.
    fn get_merged_signal(circuit_connector: CircuitConnectorId, signal: SignalID) -> i32;
    /// The merged circuit network signals or `nil` if there are no signals.
    fn get_merged_signals(circuit_connector: CircuitConnectorId) -> Option<Vec<Signal>>;
    /// Inventory for storing modules of this entity; `nil` if this entity has no module inventory.
    fn get_module_inventory() -> Option<LuaInventory>;
    /// Gets (and or creates if needed) the control behavior of the entity.
    fn get_or_create_control_behavior() -> Option<LuaControlBehavior>;
    /// Returns all signals guarding exit from a rail block this rail belongs to.
    fn get_outbound_signals() -> Vec<LuaEntity>;
    /// Gets the entity's output inventory if it has one.
    fn get_output_inventory() -> Option<LuaInventory>;
    /// Returns all parent signals. Parent signals are always RailChainSignal. Parent signals are those signals that are checking state of this signal to determine their own chain state.
    fn get_parent_signals() -> Vec<LuaEntity>;
    /// Gets the passenger of this car or spidertron if any.
    fn get_passenger() -> Option<LuaEntityMethodsGetPassengerUnion>;
    /// The radius of this entity.
    fn get_radius() -> f64;
    /// Get the rail at the end of the rail segment this rail is in.
    fn get_rail_segment_end(direction: RailDirection) -> (LuaEntity, RailDirection);
    /// Get the rail signal or train stop at the start/end of the rail segment this rail is in.
    fn get_rail_segment_entity(direction: RailDirection, in_else_out: bool) -> Option<LuaEntity>;
    /// Get the length of the rail segment this rail is in.
    fn get_rail_segment_length() -> f64;
    /// Get a rail from each rail segment that overlaps with this rail's rail segment.
    fn get_rail_segment_overlaps() -> Vec<LuaEntity>;
    /// Get all rails of a rail segment this rail is in
    fn get_rail_segment_rails(direction: RailDirection) -> Vec<LuaEntity>;
    /// Current recipe being assembled by this machine, if any.
    fn get_recipe() -> Option<LuaRecipe>;
    /// Get a logistic requester slot.
    fn get_request_slot(slot: u32) -> Option<SimpleItemStack>;
    /// Gets legs of given SpiderVehicle.
    fn get_spider_legs() -> Vec<LuaEntity>;
    /// The train currently stopped at this train stop, if any.
    fn get_stopped_train() -> Option<LuaTrain>;
    /// The trains scheduled to stop at this train stop.
    fn get_train_stop_trains() -> Vec<LuaTrain>;
    /// Get a transport line of a belt or belt connectable entity.
    fn get_transport_line(index: u32) -> LuaTransportLine;
    /// Returns the new entity direction after upgrading.
    fn get_upgrade_direction() -> Option<Direction>;
    /// Returns the new entity prototype.
    fn get_upgrade_target() -> Option<LuaEntityPrototype>;
    /// Same as [LuaEntity::has_flag](LuaEntity::has_flag), but targets the inner entity on a entity ghost.
    fn ghost_has_flag(flag: String) -> bool;
    /// Has this unit been assigned a command?
    fn has_command() -> bool;
    /// Test whether this entity's prototype has a certain flag set.
    fn has_flag(flag: String) -> bool;
    /// All methods and properties that this object supports.
    fn help() -> String;
    /// Insert fluid into this entity. Fluidbox is chosen automatically.
    fn insert_fluid(fluid: Fluid) -> f64;
    fn is_closed() -> bool;
    fn is_closing() -> bool;
    /// Returns `true` if this entity produces or consumes electricity and is connected to an electric network that has at least one entity that can produce power.
    fn is_connected_to_electric_network() -> bool;
    /// Returns whether a craft is currently in process. It does not indicate whether progress is currently being made, but whether a crafting process has been started in this machine.
    fn is_crafting() -> bool;
    fn is_opened() -> bool;
    fn is_opening() -> bool;
    /// Checks if this rail and other rail both belong to the same rail block.
    fn is_rail_in_same_rail_block_as(other_rail: LuaEntity) -> bool;
    /// Checks if this rail and other rail both belong to the same rail segment.
    fn is_rail_in_same_rail_segment_as(other_rail: LuaEntity) -> bool;
    /// Is this entity or tile ghost or item request proxy registered for construction? If false, it means a construction robot has been dispatched to build the entity, or it is not an entity that can be constructed.
    fn is_registered_for_construction() -> bool;
    /// Is this entity registered for deconstruction with this force? If false, it means a construction robot has been dispatched to deconstruct it, or it is not marked for deconstruction. The complexity is effectively O(1) - it depends on the number of objects targeting this entity which should be small enough.
    fn is_registered_for_deconstruction(force: ForceIdentification) -> bool;
    /// Is this entity registered for repair? If false, it means a construction robot has been dispatched to upgrade it, or it is not damaged. This is worst-case O(N) complexity where N is the current number of things in the repair queue.
    fn is_registered_for_repair() -> bool;
    /// Is this entity registered for upgrade? If false, it means a construction robot has been dispatched to upgrade it, or it is not marked for upgrade. This is worst-case O(N) complexity where N is the current number of things in the upgrade queue.
    fn is_registered_for_upgrade() -> bool;
    fn launch_rocket() -> bool;
    /// Mines this entity.
    fn mine(force: bool, ignore_minable: bool, inventory: LuaInventory, raise_destroyed: bool) -> bool;
    /// Sets the entity to be deconstructed by construction robots.
    fn order_deconstruction(force: ForceIdentification, player: PlayerIdentification) -> bool;
    /// Sets the entity to be upgraded by construction robots.
    fn order_upgrade(direction: Direction, force: ForceIdentification, player: PlayerIdentification, target: EntityPrototypeIdentification) -> bool;
    /// Plays a note with the given instrument and note.
    fn play_note(instrument: u32, note: u32) -> bool;
    /// Release the unit from the spawner which spawned it. This allows the spawner to continue spawning additional units.
    fn release_from_spawner();
    /// Remove fluid from this entity.
    fn remove_fluid(amount: f64, maximum_temperature: f64, minimum_temperature: f64, name: String, temperature: f64) -> f64;
    /// Remove an offer from a market.
    fn remove_market_item(offer: u32) -> bool;
    fn request_to_close(force: ForceIdentification);
    fn request_to_open(extra_time: u32, force: ForceIdentification);
    /// Revive a ghost. I.e. turn it from a ghost to a real entity or tile.
    fn revive(raise_revive: bool, return_item_request_proxy: bool) -> (Option<HashMap<String, u32>>, Option<LuaEntity>, Option<LuaEntity>);
    /// Rotates this entity as if the player rotated it.
    fn rotate(by_player: PlayerIdentification, enable_looted: bool, force: LuaEntityMethodsRotateForceUnion, reverse: bool, spill_items: bool) -> (bool, Option<HashMap<String, u32>>);
    /// Set the source of this beam.
    fn set_beam_source(source: LuaEntityMethodsSetBeamSourceSourceUnion);
    /// Set the target of this beam.
    fn set_beam_target(target: LuaEntityMethodsSetBeamTargetTargetUnion);
    /// Give the entity a command.
    fn set_command(command: Command);
    /// Give the entity a distraction command.
    fn set_distraction_command(command: Command);
    /// Sets the driver of this vehicle.
    fn set_driver(driver: LuaEntityMethodsSetDriverDriverUnion);
    /// Set the filter for a slot in an inserter, loader, or logistic storage container.
    fn set_filter(item: LuaEntityMethodsSetFilterItemUnion, slot_index: u32);
    /// Sets the heat setting for this heat interface.
    fn set_heat_setting(filter: HeatSetting);
    /// Sets the filter for this infinity container at the given index.
    fn set_infinity_container_filter(filter: LuaEntityMethodsSetInfinityContainerFilterFilterUnion, index: u32);
    /// Sets the filter for this infinity pipe.
    fn set_infinity_pipe_filter(filter: LuaEntityMethodsSetInfinityPipeFilterFilterUnion);
    /// Sets the passenger of this car or spidertron.
    fn set_passenger(passenger: LuaEntityMethodsSetPassengerPassengerUnion);
    /// Sets the given recipe in this assembly machine.
    fn set_recipe(recipe: LuaEntityMethodsSetRecipeRecipeUnion) -> HashMap<String, u32>;
    /// Set a logistic requester slot.
    fn set_request_slot(request: ItemStackIdentification, slot: u32) -> bool;
    /// Revives a ghost silently.
    fn silent_revive(raise_revive: bool, return_item_request_proxy: bool) -> (Option<HashMap<String, u32>>, Option<LuaEntity>, Option<LuaEntity>);
    /// Triggers spawn_decoration actions defined in the entity prototype or does nothing if entity is not "turret" or "unit-spawner".
    fn spawn_decorations();
    /// Only works if the entity is a speech-bubble, with an "effect" defined in its wrapper_flow_style. Starts animating the opacity of the speech bubble towards zero, and destroys the entity when it hits zero.
    fn start_fading_out();
    /// Sets the [speed](LuaEntity::speed) of the given SpiderVehicle to zero. Notably does not clear its [autopilot_destination](LuaEntity::autopilot_destination), which it will continue moving towards if set.
    fn stop_spider();
    /// Whether this entity supports a backer name.
    fn supports_backer_name() -> bool;
    /// Is this entity marked for deconstruction?
    fn to_be_deconstructed() -> bool;
    /// Is this entity marked for upgrade?
    fn to_be_upgraded() -> bool;
    /// Toggle this entity's equipment movement bonus. Does nothing if the entity does not have an equipment grid.
    fn toggle_equipment_movement_bonus();
    /// Reconnect loader, beacon, cliff and mining drill connections to entities that might have been teleported out or in by the script. The game doesn't do this automatically as we don't want to loose performance by checking this in normal games.
    fn update_connections();
}

pub struct LuaEntityPrototypeCreatedSmoke {
    pub initial_height: f32,
    pub max_radius: Option<f32>,
    pub offset_deviation: BoundingBox,
    pub offsets: Vec<Vector>,
    pub smoke_name: String,
    pub speed: Vector,
    pub speed_from_center: f32,
    pub speed_from_center_deviation: f32,
    pub speed_multiplier: f32,
    pub speed_multiplier_deviation: f32,
    pub starting_frame: f32,
    pub starting_frame_deviation: f32,
    pub starting_frame_speed: f32,
    pub starting_frame_speed_deviation: f32,
}

pub struct LuaEntityPrototypeLogisticParameters {
    pub charge_approach_distance: f32,
    pub charging_distance: f32,
    pub charging_energy: f64,
    pub charging_station_count: u32,
    pub charging_station_shift: Vector,
    pub charging_threshold_distance: f32,
    pub construction_radius: f32,
    pub logistic_radius: f32,
    pub logistics_connection_distance: f32,
    pub robot_limit: u32,
    pub robot_vertical_acceleration: f32,
    pub robots_shrink_when_entering_and_exiting: bool,
    pub spawn_and_station_height: f32,
    pub spawn_and_station_shadow_height_offset: f32,
    pub stationing_offset: Vector,
}

pub struct LuaEntityPrototypeMineableProperties {
    /// The required fluid amount if any.
    pub fluid_amount: Option<f64>,
    /// Is this entity mineable at all?
    pub minable: bool,
    /// Prototype name of the particle produced when mining this entity. Will only be present if this entity produces any particle during mining.
    pub mining_particle: Option<String>,
    /// Energy required to mine an entity.
    pub mining_time: f64,
    /// The mining trigger if any.
    pub mining_trigger: Option<Vec<TriggerItem>>,
    /// Products obtained by mining this entity.
    pub products: Option<Vec<Product>>,
    /// The prototype name of the required fluid if any.
    pub required_fluid: Option<String>,
}

pub struct LuaEntityPrototypeSpawnCooldown {
    pub max: f64,
    pub min: f64,
}

/// Prototype of an entity.
pub struct LuaEntityPrototype {
    /// The active energy usage of this rocket silo or combinator prototype.
    pub active_energy_usage: Option<f64>,
    /// Entities this entity can be pasted onto in addition to the normal allowed ones.
    pub additional_pastable_entities: Vec<LuaEntityPrototype>,
    /// The bounding box that specifies which tiles adjacent to the offshore pump should be checked.
    pub adjacent_tile_collision_box: Option<BoundingBox>,
    /// Tiles adjacent to the offshore pump must not collide with this collision mask.
    pub adjacent_tile_collision_mask: Option<CollisionMask>,
    /// If this mask is not empty, tiles adjacent to the offshore pump must not collide with this collision mask.
    pub adjacent_tile_collision_test: Option<CollisionMask>,
    /// Whether this unit prototype is affected by tile walking speed modifiers.
    pub affected_by_tiles: Option<bool>,
    /// The air resistance of this rolling stock prototype.
    pub air_resistance: Option<f64>,
    /// The alert icon scale of this entity prototype.
    pub alert_icon_scale: f32,
    /// The alert icon shift of this entity prototype.
    pub alert_icon_shift: Vector,
    /// Whether this turret raises an alert when attacking
    pub alert_when_attacking: Option<bool>,
    /// Whether this entity raises an alert when damaged.
    pub alert_when_damaged: Option<bool>,
    /// Whether this market allows access to all forces or just friendly ones.
    pub allow_access_to_all_forces: Option<bool>,
    /// Whether this inserter allows burner leeching.
    pub allow_burner_leech: Option<bool>,
    /// When false copy-paste is not allowed for this entity.
    pub allow_copy_paste: bool,
    /// Whether this inserter allows custom pickup and drop vectors.
    pub allow_custom_vectors: Option<bool>,
    /// Whether this vehicle allows passengers.
    pub allow_passengers: Option<bool>,
    /// True if this entity-with-owner's is_military_target can be changed run-time (on the entity, not on the prototype itself)
    pub allow_run_time_change_of_is_military_target: Option<bool>,
    /// The allowed module effects for this entity, if any.
    pub allowed_effects: Option<HashMap<String, bool>>,
    /// Whether the lamp is always on (except when out of power or turned off by the circuit network).
    pub always_on: Option<bool>,
    /// Name of the ammo category of this land mine.
    pub ammo_category: Option<String>,
    /// The animation speed coefficient of this belt connectable prototype.
    pub animation_speed_coefficient: Option<f64>,
    /// The attack parameters for this entity, if any.
    pub attack_parameters: Option<AttackParameters>,
    /// The attack result of this entity, if any.
    pub attack_result: Option<Vec<TriggerItem>>,
    /// The amount of ammo that inserters automatically insert into this ammo-turret or artillery-turret.
    pub automated_ammo_count: Option<u32>,
    /// Whether this spider vehicle prototype automatically cycles weapons.
    pub automatic_weapon_cycling: Option<bool>,
    /// Autoplace specification for this entity prototype, if any.
    pub autoplace_specification: Option<AutoplaceSpecification>,
    /// The base productivity of this crafting machine, lab, or mining drill.
    pub base_productivity: Option<f64>,
    pub belt_distance: Option<f64>,
    pub belt_length: Option<f64>,
    /// The speed of this transport belt.
    pub belt_speed: Option<f64>,
    /// The braking force of this vehicle prototype.
    pub braking_force: Option<f64>,
    /// The evolution requirement to build this entity as a base when expanding enemy bases.
    pub build_base_evolution_requirement: f64,
    pub build_distance: Option<u32>,
    /// The log2 of grid size of the building
    pub building_grid_bit_shift: u32,
    /// The burner energy source prototype this entity uses, if any.
    pub burner_prototype: Option<LuaBurnerPrototype>,
    /// Whether this generator prototype burns fluid.
    pub burns_fluid: Option<bool>,
    pub call_for_help_radius: Option<f64>,
    /// Whether this unit prototype can open gates.
    pub can_open_gates: Option<bool>,
    /// The collision mask used only for collision test with tile directly at offshore pump position.
    pub center_collision_mask: Option<CollisionMask>,
    /// The chain shooting cooldown modifier of this spider vehicle prototype.
    pub chain_shooting_cooldown_modifier: Option<f64>,
    pub character_corpse: Option<Box<LuaEntityPrototype>>,
    /// The chunk exploration radius of this spider vehicle prototype.
    pub chunk_exploration_radius: Option<f64>,
    /// The item prototype name used to destroy this cliff.
    pub cliff_explosive_prototype: Option<String>,
    /// The bounding box used for collision checking.
    pub collision_box: BoundingBox,
    /// The collision masks this entity uses
    pub collision_mask: CollisionMask,
    /// Does this prototype collision mask collide with itself?
    pub collision_mask_collides_with_self: bool,
    /// Does this prototype collision mask collide with tiles only?
    pub collision_mask_collides_with_tiles_only: bool,
    /// Does this prototype collision mask consider tile transitions?
    pub collision_mask_considers_tile_transitions: bool,
    pub collision_mask_with_flags: CollisionMaskWithFlags,
    /// The color of the prototype, if any.
    pub color: Option<Color>,
    /// The construction radius for this roboport prototype.
    pub construction_radius: Option<f64>,
    /// The energy consumption of this car prototype.
    pub consumption: Option<f64>,
    pub container_distance: Option<f64>,
    /// Corpses used when this entity is destroyed. It is a dictionary indexed by the corpse's prototype name.
    pub corpses: Option<HashMap<String, LuaEntityPrototype>>,
    /// If this simple-entity is counted as a rock for the deconstruction planner "trees and rocks only" filter.
    pub count_as_rock_for_filtered_deconstruction: Option<bool>,
    /// The crafting categories this entity prototype supports.
    pub crafting_categories: Option<HashMap<String, bool>>,
    /// The crafting speed..
    pub crafting_speed: Option<f64>,
    /// If this prototype will attempt to create a ghost of itself on death.
    pub create_ghost_on_death: bool,
    /// The trigger to run when this entity is created, if any.
    pub created_effect: Option<Vec<TriggerItem>>,
    /// The smoke trigger run when this entity is built, if any.
    pub created_smoke: Option<LuaEntityPrototypeCreatedSmoke>,
    pub damage_hit_tint: Option<Color>,
    /// Value between 0 and 1 darkness where all lamps of this lamp prototype are off.
    pub darkness_for_all_lamps_off: Option<f32>,
    /// Value between 0 and 1 darkness where all lamps of this lamp prototype are on.
    pub darkness_for_all_lamps_on: Option<f32>,
    /// The hardcoded default collision mask (with flags) for this entity prototype type.
    pub default_collision_mask_with_flags: CollisionMaskWithFlags,
    /// Whether this generator prototype destroys non-fuel fluids.
    pub destroy_non_fuel_fluid: Option<bool>,
    /// The distraction cooldown of this unit prototype.
    pub distraction_cooldown: Option<u32>,
    /// The distribution effectivity for this beacon prototype.
    pub distribution_effectivity: Option<f64>,
    /// The door opening speed for this rocket silo prototype.
    pub door_opening_speed: Option<f64>,
    /// Whether this logistics or construction robot renders its cargo when flying.
    pub draw_cargo: Option<bool>,
    /// The bounding box used for drawing the entity icon.
    pub drawing_box: BoundingBox,
    pub drop_item_distance: Option<u32>,
    /// The dying time of this corpse prototype.
    pub dying_speed: Option<f32>,
    /// The effectivity of this car prototype, generator prototype.
    pub effectivity: Option<f64>,
    /// The electric energy source prototype this entity uses, if any.
    pub electric_energy_source_prototype: Option<LuaElectricEnergySourcePrototype>,
    /// Amount of pollution emissions per second this entity will create.
    pub emissions_per_second: f64,
    /// The enemy map color used when charting this entity.
    pub enemy_map_color: Color,
    /// The energy used per hitpoint taken for this vehicle during collisions.
    pub energy_per_hit_point: Option<f64>,
    /// The energy consumed per tile moved for this flying robot.
    pub energy_per_move: Option<f64>,
    /// The energy consumed per tick for this flying robot.
    pub energy_per_tick: Option<f64>,
    /// The direct energy usage of this entity, if any.
    pub energy_usage: Option<f64>,
    /// The engine starting speed for this rocket silo rocket prototype.
    pub engine_starting_speed: Option<f64>,
    pub enter_vehicle_distance: Option<f64>,
    /// Whether this explosion has a beam.
    pub explosion_beam: Option<f64>,
    /// Whether this explosion rotates.
    pub explosion_rotate: Option<f64>,
    /// The group of mutually fast-replaceable entities, if any.
    pub fast_replaceable_group: Option<String>,
    /// The filter count of this inserter, loader, or logistic chest. For logistic containers, `nil` means no limit.
    pub filter_count: Option<u32>,
    /// The final attack result for this projectile.
    pub final_attack_result: Option<Vec<TriggerItem>>,
    /// The fixed recipe name for this assembling machine prototype, if any.
    pub fixed_recipe: Option<String>,
    /// The flags for this entity prototype.
    pub flags: EntityPrototypeFlags,
    /// The fluid this offshore pump produces.
    pub fluid: Option<LuaFluidPrototype>,
    /// The fluid capacity of this entity or 0 if this entity doesn't support fluids.
    pub fluid_capacity: f64,
    /// The fluid energy source prototype this entity uses, if any.
    pub fluid_energy_source_prototype: Option<LuaFluidEnergySourcePrototype>,
    /// The fluid usage of this generator prototype.
    pub fluid_usage_per_tick: Option<f64>,
    /// The fluidbox prototypes for this entity.
    pub fluidbox_prototypes: Vec<LuaFluidBoxPrototype>,
    /// The flying acceleration for this rocket silo rocket prototype.
    pub flying_acceleration: Option<f64>,
    /// The flying speed for this rocket silo rocket prototype.
    pub flying_speed: Option<f64>,
    /// The friction of this vehicle prototype.
    pub friction_force: Option<f64>,
    /// The friendly map color used when charting this entity.
    pub friendly_map_color: Color,
    /// The equipment grid prototype for this entity, if any.
    pub grid_prototype: Option<LuaEquipmentGridPrototype>,
    /// Group of this entity.
    pub group: LuaGroup,
    /// A mapping of the gun name to the gun prototype this prototype uses. `nil` if this entity prototype doesn't use guns.
    pub guns: Option<HashMap<String, LuaItemPrototype>>,
    /// Whether this unit, car, or character prototype has belt immunity.
    pub has_belt_immunity: Option<bool>,
    /// Amount this entity can heal per tick, if any.
    pub healing_per_tick: Option<f32>,
    /// The heat buffer prototype this entity uses, if any.
    pub heat_buffer_prototype: Option<LuaHeatBufferPrototype>,
    /// The heat energy source prototype this entity uses, if any.
    pub heat_energy_source_prototype: Option<LuaHeatEnergySourcePrototype>,
    /// The height of this spider vehicle prototype.
    pub height: Option<f64>,
    /// The idle energy usage of this rocket silo prototype.
    pub idle_energy_usage: Option<f64>,
    /// A vector of the gun prototypes of this car, spider vehicle, artillery wagon, or turret.
    pub indexed_guns: Option<Vec<LuaItemPrototype>>,
    /// Every time this infinite resource 'ticks' down, it is reduced by this amount. Meaningless if this isn't an infinite resource.
    pub infinite_depletion_resource_amount: Option<u32>,
    /// Whether this resource is infinite.
    pub infinite_resource: Option<bool>,
    /// The max number of ingredients this crafting machine prototype supports.
    pub ingredient_count: Option<u32>,
    /// True if this inserter chases items on belts for pickup.
    pub inserter_chases_belt_items: Option<bool>,
    /// The drop position for this inserter.
    pub inserter_drop_position: Option<Vector>,
    /// The extension speed of this inserter.
    pub inserter_extension_speed: Option<f64>,
    /// The pickup position for this inserter.
    pub inserter_pickup_position: Option<Vector>,
    /// The rotation speed of this inserter.
    pub inserter_rotation_speed: Option<f64>,
    /// The built-in stack size bonus of this inserter prototype.
    pub inserter_stack_size_bonus: Option<f64>,
    /// The instruments for this programmable speaker.
    pub instruments: Option<Vec<ProgrammableSpeakerInstrument>>,
    /// Everything in the following list is considered a building.
    ///
    /// - AccumulatorPrototype
    /// - ArtilleryTurretPrototype
    /// - BeaconPrototype
    /// - BoilerPrototype
    /// - BurnerGeneratorPrototype
    /// - CombinatorPrototype → ArithmeticCombinator, DeciderCombinator
    /// - ConstantCombinatorPrototype
    /// - ContainerPrototype → LogisticContainer, InfinityContainer
    /// - CraftingMachinePrototype → AssemblingMachine, RocketSilo, Furnace
    /// - ElectricEnergyInterfacePrototype
    /// - ElectricPolePrototype
    /// - EnemySpawnerPrototype
    /// - GatePrototype
    /// - GeneratorPrototype
    /// - HeatInterfacePrototype
    /// - HeatPipePrototype
    /// - InserterPrototype
    /// - LabPrototype
    /// - LampPrototype
    /// - LinkedContainerPrototype
    /// - MarketPrototype
    /// - MiningDrillPrototype
    /// - OffshorePumpPrototype
    /// - PipePrototype → InfinityPipe
    /// - PipeToGroundPrototype
    /// - PlayerPortPrototype
    /// - PowerSwitchPrototype
    /// - ProgrammableSpeakerPrototype
    /// - PumpPrototype
    /// - RadarPrototype
    /// - RailPrototype → CurvedRail, StraightRail
    /// - RailSignalBasePrototype → RailChainSignal, RailSignal
    /// - ReactorPrototype
    /// - RoboportPrototype
    /// - SimpleEntityPrototype
    /// - SimpleEntityWithOwnerPrototype → SimpleEntityWithForce
    /// - SolarPanelPrototype
    /// - StorageTankPrototype
    /// - TrainStopPrototype
    /// - TransportBeltConnectablePrototype → LinkedBelt, Loader1x1, Loader1x2, Splitter, TransportBelt, UndergroundBelt
    /// - TurretPrototype → AmmoTurret, ElectricTurret, FluidTurret
    /// - WallPrototype
    pub is_building: bool,
    /// True if this is entity-with-owner
    pub is_entity_with_owner: bool,
    /// True if this entity-with-owner is military target
    pub is_military_target: Option<bool>,
    pub item_pickup_distance: Option<f64>,
    /// The item slot count of this constant combinator prototype.
    pub item_slot_count: Option<u32>,
    /// Items that when placed will produce this entity, if any. Construction bots will choose the first item in the list to build this entity.
    pub items_to_place_this: Option<Vec<ItemStackDefinition>>,
    /// The item prototype names that are the inputs of this lab prototype.
    pub lab_inputs: Option<Vec<String>>,
    /// The lamp energy usage of this rocket silo prototype.
    pub lamp_energy_usage: Option<f64>,
    /// The rocket launch delay for this rocket silo prototype.
    pub launch_wait_time: Option<u8>,
    /// The light blinking speed for this rocket silo prototype.
    pub light_blinking_speed: Option<f64>,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    /// The logistic mode of this logistic container. One of `"requester"`, `"active-provider"`, `"passive-provider"`, `"buffer"`, `"storage"`, `"none"`.
    pub logistic_mode: Option<String>,
    /// The logistic parameters for this roboport.
    pub logistic_parameters: Option<LuaEntityPrototypeLogisticParameters>,
    /// The logistic radius for this roboport prototype.
    pub logistic_radius: Option<f64>,
    /// Loot that will be dropped when this entity is killed, if any.
    pub loot: Option<Vec<Loot>>,
    pub loot_pickup_distance: Option<f64>,
    /// The manual range modifier for this artillery turret or wagon prototype.
    ///
    /// subclass(ArtilleryWagon, ArtilleryTurret)
    pub manual_range_modifier: Option<f64>,
    /// The map color used when charting this entity if a friendly or enemy color isn't defined, if any.
    pub map_color: Option<Color>,
    /// The bounding box used for map generator collision checking.
    pub map_generator_bounding_box: BoundingBox,
    /// The maximum circuit wire distance for this entity. 0 if the entity doesn't support circuit wires.
    pub max_circuit_wire_distance: f64,
    /// Count of enemies this spawner can sustain.
    pub max_count_of_owned_units: Option<f64>,
    /// The maximum darkness at which this unit spawner can spawn entities.
    pub max_darkness_to_spawn: Option<f32>,
    /// The radius of the area constantly revealed by this radar, in chunks.
    pub max_distance_of_nearby_sector_revealed: Option<u32>,
    /// The radius of the area this radar can chart, in chunks.
    pub max_distance_of_sector_revealed: Option<u32>,
    /// The max energy for this flying robot.
    pub max_energy: Option<f64>,
    /// The theoretical maximum energy production for this this entity.
    pub max_energy_production: f64,
    /// The theoretical maximum energy usage for this entity.
    pub max_energy_usage: f64,
    /// How many friendly units are required within the spawning_radius of this spawner for it to stop producing more units.
    pub max_friends_around_to_spawn: Option<f64>,
    /// Max health of this entity. Will be `0` if this is not an entity with health.
    pub max_health: f32,
    /// The max payload size of this logistics or construction robot.
    pub max_payload_size: Option<u32>,
    /// The maximum polyphony for this programmable speaker.
    pub max_polyphony: Option<u32>,
    /// The default maximum power output of this generator prototype.
    pub max_power_output: Option<f64>,
    /// The maximum pursue distance of this unit prototype.
    pub max_pursue_distance: Option<f64>,
    /// The max speed of this projectile or flying robot prototype.
    pub max_speed: Option<f64>,
    /// The maximum energy for this flying robot above which it won't try to recharge when stationing.
    pub max_to_charge: Option<f32>,
    /// The max underground distance for underground belts and underground pipes.
    pub max_underground_distance: Option<u8>,
    /// The maximum wire distance for this entity. 0 if the entity doesn't support wires.
    pub max_wire_distance: f64,
    pub maximum_corner_sliding_distance: Option<f64>,
    /// The maximum fluid temperature of this generator prototype.
    pub maximum_temperature: Option<f64>,
    /// The minimum darkness at which this unit spawner can spawn entities.
    pub min_darkness_to_spawn: Option<f32>,
    /// The minimum pursue time of this unit prototype.
    pub min_pursue_time: Option<u32>,
    /// The minimum energy for this flying robot before it tries to recharge.
    pub min_to_charge: Option<f32>,
    /// Whether this entity is minable and what can be obtained by mining it.
    pub mineable_properties: LuaEntityPrototypeMineableProperties,
    /// Minimum amount of this resource.
    pub minimum_resource_amount: Option<u32>,
    /// The mining radius of this mining drill prototype.
    pub mining_drill_radius: Option<f64>,
    /// The mining speed of this mining drill/character prototype.
    pub mining_speed: Option<f64>,
    /// The module inventory size. `nil` if this entity doesn't support modules.
    pub module_inventory_size: Option<u32>,
    /// Whether this unit prototype can move while shooting.
    pub move_while_shooting: Option<bool>,
    /// Name of this prototype.
    pub name: String,
    pub neighbour_bonus: Option<f64>,
    /// The next upgrade for this entity, if any.
    pub next_upgrade: Option<Box<LuaEntityPrototype>>,
    /// The normal amount for this resource.
    pub normal_resource_amount: Option<u32>,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    /// The amount of pollution that has to be absorbed by the unit's spawner before the unit will leave the spawner and attack the source of the pollution.
    pub pollution_to_join_attack: Option<f32>,
    /// True if this entity prototype should be included during tile collision checks with [LuaTilePrototype::check_collision_with_entities](LuaTilePrototype::check_collision_with_entities) enabled.
    pub protected_from_tile_building: bool,
    /// The pumping speed of this offshore or normal pump.
    pub pumping_speed: Option<f64>,
    /// The radar range of this unit prototype.
    pub radar_range: Option<u32>,
    /// The radius of this entity prototype.
    pub radius: f64,
    pub reach_distance: Option<u32>,
    pub reach_resource_distance: Option<f64>,
    pub related_underground_belt: Option<Box<LuaEntityPrototype>>,
    /// The remains left behind when this entity is mined.
    pub remains_when_mined: Vec<LuaEntityPrototype>,
    pub remove_decoratives: String,
    /// Repair-speed modifier for this entity, if any. Actual repair speed will be `tool_repair_speed * entity_repair_speed_modifier`.
    pub repair_speed_modifier: Option<u32>,
    /// The base researching speed of this lab prototype.
    pub researching_speed: Option<f64>,
    /// List of resistances towards each damage type. It is a dictionary indexed by damage type names (see `data/base/prototypes/damage-type.lua`).
    pub resistances: Option<HashMap<String, Resistance>>,
    /// The resource categories this character or mining drill supports.
    pub resource_categories: Option<HashMap<String, bool>>,
    /// Name of the category of this resource.
    pub resource_category: Option<String>,
    pub respawn_time: Option<u32>,
    /// The result units and spawn points with weight and evolution factor for a biter spawner entity.
    pub result_units: Option<Vec<UnitSpawnDefinition>>,
    /// The rising speed for this rocket silo rocket prototype.
    pub rising_speed: Option<f64>,
    /// The rocket entity prototype associated with this rocket silo prototype.
    pub rocket_entity_prototype: Option<Box<LuaEntityPrototype>>,
    /// The rocket parts required for this rocket silo prototype.
    pub rocket_parts_required: Option<u32>,
    /// The rocket rising delay for this rocket silo prototype.
    pub rocket_rising_delay: Option<u8>,
    /// The rotation speed of this car prototype.
    pub rotation_speed: Option<f64>,
    /// The current movement speed of this character, including effects from exoskeletons, tiles, stickers and shooting.
    pub running_speed: Option<f64>,
    /// Whether this generator prototype scales fluid usage.
    pub scale_fluid_usage: Option<bool>,
    /// The secondary bounding box used for collision checking, if any. This is only used in rails and rail remnants.
    pub secondary_collision_box: Option<BoundingBox>,
    /// Is this entity selectable?
    pub selectable_in_game: bool,
    /// The bounding box used for drawing selection.
    pub selection_box: BoundingBox,
    /// The selection priority of this entity - a value between 0 and 255
    pub selection_priority: u32,
    /// The cursor size used when shooting at this entity.
    pub shooting_cursor_size: f64,
    /// The spawning cooldown for this enemy spawner prototype.
    pub spawn_cooldown: Option<LuaEntityPrototypeSpawnCooldown>,
    /// How far from the spawner can the units be spawned.
    pub spawning_radius: Option<f64>,
    /// What spaces should be between the spawned units.
    pub spawning_spacing: Option<f64>,
    /// The spawning time modifier of this unit prototype.
    pub spawning_time_modifier: Option<f64>,
    /// The default speed of this flying robot, rolling stock or unit. For rolling stocks, this is their `max_speed`.
    pub speed: Option<f64>,
    /// The speed multiplier when this flying robot is out of energy.
    pub speed_multiplier_when_out_of_energy: Option<f32>,
    /// Whether this inserter is a stack-type.
    pub stack: Option<bool>,
    /// The bounding box used to attach sticker type entities.
    pub sticker_box: BoundingBox,
    /// Subgroup of this entity.
    pub subgroup: LuaGroup,
    /// The supply area of this electric pole or beacon prototype.
    pub supply_area_distance: Option<f64>,
    /// Whether this entity prototype could possibly ever be rotated.
    pub supports_direction: bool,
    /// If this car prototype uses tank controls to drive.
    pub tank_driving: Option<bool>,
    /// The target temperature of this boiler prototype.
    pub target_temperature: Option<f64>,
    /// The terrain friction modifier for this vehicle.
    pub terrain_friction_modifier: Option<f32>,
    pub ticks_to_keep_aiming_direction: Option<u32>,
    pub ticks_to_keep_gun: Option<u32>,
    pub ticks_to_stay_in_combat: Option<u32>,
    /// Specifies the tiling size of the entity, is used to decide, if the center should be in the center of the tile (odd tile size dimension) or on the tile border (even tile size dimension)
    pub tile_height: u32,
    /// Specifies the tiling size of the entity, is used to decide, if the center should be in the center of the tile (odd tile size dimension) or on the tile border (even tile size dimension)
    pub tile_width: u32,
    /// The time to live for this prototype or `0` if prototype doesn't have time_to_live or time_before_removed.
    pub time_to_live: u32,
    /// The time it takes this land mine to arm.
    pub timeout: Option<u32>,
    /// The torso bob speed of this spider vehicle prototype.
    pub torso_bob_speed: Option<f64>,
    /// The torso rotation speed of this spider vehicle prototype.
    pub torso_rotation_speed: Option<f64>,
    /// If it is a tree, return the number of colors it supports.
    pub tree_color_count: Option<u8>,
    /// The collision mask entities must collide with to make this landmine blow up.
    pub trigger_collision_mask: Option<CollisionMaskWithFlags>,
    /// The range of this turret.
    pub turret_range: Option<u32>,
    /// The turret rotation speed of this car prototype.
    pub turret_rotation_speed: Option<f64>,
    /// Type of this prototype.
    pub typ: String,
    /// Whether this logistic container prototype uses exact mode
    pub use_exact_mode: Option<bool>,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
    /// The vision distance of this unit prototype.
    pub vision_distance: Option<f64>,
    /// The void energy source prototype this entity uses, if any.
    pub void_energy_source_prototype: Option<LuaVoidEnergySourcePrototype>,
    /// The weight of this vehicle prototype.
    pub weight: Option<f64>,
}

/// Prototype of an entity.
pub trait LuaEntityPrototypeMethods {
    /// Gets the base size of the given inventory on this entity or `nil` if the given inventory doesn't exist.
    fn get_inventory_size(index: Inventory) -> Option<u32>;
    /// Test whether this entity prototype has a certain flag set.
    fn has_flag(flag: String) -> bool;
    /// All methods and properties that this object supports.
    fn help() -> String;
}

pub struct LuaEquipmentShape {
    pub height: u32,
    pub width: u32,
}

/// An item in a [LuaEquipmentGrid](LuaEquipmentGrid), for example a fusion reactor placed in one's power armor.
///
/// An equipment reference becomes invalid once the equipment is removed or the equipment grid it resides in is destroyed.
pub struct LuaEquipment {
    /// The burner energy source for this equipment, if any.
    pub burner: Option<Box<LuaBurner>>,
    /// Current available energy.
    pub energy: f64,
    /// Energy generated per tick.
    pub generator_power: f64,
    /// Maximum amount of energy that can be stored in this equipment.
    pub max_energy: f64,
    /// Maximum shield value.
    pub max_shield: f64,
    /// Maximum solar power generated.
    pub max_solar_power: f64,
    /// Movement speed bonus.
    pub movement_bonus: f64,
    /// Name of this equipment.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// Position of this equipment in the equipment grid.
    pub position: EquipmentPosition,
    pub prototype: LuaEquipmentPrototype,
    /// Shape of this equipment.
    pub shape: LuaEquipmentShape,
    /// Current shield value of the equipment.
    pub shield: f64,
    /// Type of this equipment.
    pub typ: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// An item in a [LuaEquipmentGrid](LuaEquipmentGrid), for example a fusion reactor placed in one's power armor.
///
/// An equipment reference becomes invalid once the equipment is removed or the equipment grid it resides in is destroyed.
pub trait LuaEquipmentMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Prototype of an equipment category.
pub struct LuaEquipmentCategoryPrototype {
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    /// Name of this prototype.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of an equipment category.
pub trait LuaEquipmentCategoryPrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// An equipment grid is for example the inside of a power armor.
pub struct LuaEquipmentGrid {
    /// The total energy stored in all batteries in the equipment grid.
    pub available_in_batteries: f64,
    /// Total energy storage capacity of all batteries in the equipment grid.
    pub battery_capacity: f64,
    /// All the equipment in this grid.
    pub equipment: Vec<LuaEquipment>,
    /// Total energy per tick generated by the equipment inside this grid.
    pub generator_energy: f64,
    /// Height of the equipment grid.
    pub height: u32,
    /// Whether this grid's equipment movement bonus is active.
    pub inhibit_movement_bonus: bool,
    /// The maximum amount of shields this equipment grid has.
    pub max_shield: f32,
    /// Maximum energy per tick that can be created by any solar panels in the equipment grid. Actual generated energy varies depending on the daylight levels.
    pub max_solar_energy: f64,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    pub prototype: LuaEquipmentGridPrototype,
    /// The amount of shields this equipment grid has.
    pub shield: f32,
    /// Unique identifier of this equipment grid.
    pub unique_id: u32,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
    /// Width of the equipment grid.
    pub width: u32,
}

/// An equipment grid is for example the inside of a power armor.
pub trait LuaEquipmentGridMethods {
    /// Check whether moving an equipment would succeed.
    fn can_move(equipment: LuaEquipment, position: EquipmentPosition) -> bool;
    /// Clear all equipment from the grid, removing it without actually returning it.
    fn clear(by_player: PlayerIdentification);
    /// Get the number of all or some equipment in this grid.
    fn count(equipment: String) -> u32;
    /// Find equipment by name.
    fn find(equipment: String) -> Option<LuaEquipment>;
    /// Find equipment in the Equipment Grid based off a position.
    fn get(position: EquipmentPosition) -> Option<LuaEquipment>;
    /// Get counts of all equipment in this grid.
    fn get_contents() -> HashMap<String, u32>;
    /// All methods and properties that this object supports.
    fn help() -> String;
    /// Move an equipment within this grid.
    fn mov(equipment: LuaEquipment, position: EquipmentPosition) -> bool;
    /// Insert an equipment into the grid.
    fn put(by_player: PlayerIdentification, name: String, position: EquipmentPosition) -> Option<LuaEquipment>;
    /// Remove an equipment from the grid.
    fn take(by_player: PlayerIdentification, equipment: LuaEquipment, position: EquipmentPosition) -> Option<SimpleItemStack>;
    /// Remove all equipment from the grid.
    fn take_all(by_player: PlayerIdentification) -> HashMap<String, u32>;
}

/// Prototype of an equipment grid.
pub struct LuaEquipmentGridPrototype {
    /// Equipment category names for the [categories](LuaEquipmentPrototype::equipment_categories) that may be inserted into this equipment grid. The grid will accept any equipment that has at least one category in this list.
    pub equipment_categories: Vec<String>,
    pub height: u32,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    /// If the player can move equipment into or out of this grid.
    pub locked: bool,
    /// Name of this prototype.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
    pub width: u32,
}

/// Prototype of an equipment grid.
pub trait LuaEquipmentGridPrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

pub struct LuaEquipmentPrototypeLogisticParameters {
    pub charge_approach_distance: f32,
    pub charging_distance: f32,
    pub charging_energy: f64,
    pub charging_station_count: u32,
    pub charging_station_shift: Vector,
    pub charging_threshold_distance: f32,
    pub construction_radius: f32,
    pub logistic_radius: f32,
    pub logistics_connection_distance: f32,
    pub robot_limit: u32,
    pub robot_vertical_acceleration: f32,
    pub robots_shrink_when_entering_and_exiting: bool,
    pub spawn_and_station_height: f32,
    pub spawn_and_station_shadow_height_offset: f32,
    pub stationing_offset: Vector,
}

pub struct LuaEquipmentPrototypeShape {
    pub height: u32,
    /// Only set when the shape is "manual"
    pub points: Option<Vec<EquipmentPoint>>,
    pub width: u32,
}

/// Prototype of a modular equipment.
pub struct LuaEquipmentPrototype {
    /// The equipment attack parameters.
    pub attack_parameters: Option<AttackParameters>,
    /// Whether this active defense equipment is automatic. Returns false if not active defense equipment.
    pub automatic: bool,
    /// The background color of this equipment prototype.
    pub background_color: Color,
    /// The burner energy source prototype this equipment uses, if any.
    pub burner_prototype: Option<LuaBurnerPrototype>,
    /// The electric energy source prototype this equipment uses, if any.
    pub electric_energy_source_prototype: Option<LuaElectricEnergySourcePrototype>,
    pub energy_consumption: f64,
    /// The energy per shield point restored. 0 for non-shield equipment.
    pub energy_per_shield: f64,
    /// The max power generated by this equipment.
    pub energy_production: f64,
    /// The energy source prototype for the equipment.
    pub energy_source: LuaElectricEnergySourcePrototype,
    /// Category names for this equipment. These [categories](LuaEquipmentGridPrototype::equipment_categories) will be used to determine whether this equipment is allowed in a particular equipment grid.
    pub equipment_categories: Vec<String>,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    /// The logistic parameters for this roboport equipment.
    pub logistic_parameters: Option<LuaEquipmentPrototypeLogisticParameters>,
    pub movement_bonus: Option<f32>,
    /// Name of this prototype.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    /// Shape of this equipment prototype.
    pub shape: LuaEquipmentPrototypeShape,
    /// The shield value of this equipment. 0 for non-shield equipment.
    pub shield: f32,
    /// The result item when taking this equipment out of an equipment grid, if any.
    pub take_result: Option<Box<LuaItemPrototype>>,
    /// Type of this equipment prototype.
    pub typ: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of a modular equipment.
pub trait LuaEquipmentPrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

pub enum LuaFlowStatisticsInputCountsUnion {
    Uint64(u64),
    Double(f64),
}

pub enum LuaFlowStatisticsOutputCountsUnion {
    Uint64(u64),
    Double(f64),
}

/// Encapsulates statistic data for different parts of the game. In the context of flow statistics, `input` and `output` describe on which side of the associated GUI the values are shown. Input values are shown on the left side, output values on the right side.
///
/// Examples:
/// - The item production GUI shows "consumption" on the right, thus `output` describes the item consumption numbers. The same goes for fluid consumption.
/// - The kills GUI shows "losses" on the right, so `output` describes how many of the force's entities were killed by enemies.
/// - The electric network GUI shows "power consumption" on the left side, so in this case `input` describes the power consumption numbers.
pub struct LuaFlowStatistics {
    /// The force these statistics belong to. `nil` for pollution statistics.
    pub force: Option<Box<LuaForce>>,
    /// List of input counts indexed by prototype name. Represents the data that is shown on the left side of the GUI for the given statistics.
    pub input_counts: HashMap<String, LuaFlowStatisticsInputCountsUnion>,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// List of output counts indexed by prototype name. Represents the data that is shown on the right side of the GUI for the given statistics.
    pub output_counts: HashMap<String, LuaFlowStatisticsOutputCountsUnion>,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

pub enum LuaFlowStatisticsMethodsGetInputCountUnion {
    Uint64(u64),
    Double(f64),
}

pub enum LuaFlowStatisticsMethodsGetOutputCountUnion {
    Uint64(u64),
    Double(f64),
}

pub enum LuaFlowStatisticsMethodsSetInputCountCountUnion {
    Uint64(u64),
    Double(f64),
}

pub enum LuaFlowStatisticsMethodsSetOutputCountCountUnion {
    Uint64(u64),
    Double(f64),
}

/// Encapsulates statistic data for different parts of the game. In the context of flow statistics, `input` and `output` describe on which side of the associated GUI the values are shown. Input values are shown on the left side, output values on the right side.
///
/// Examples:
/// - The item production GUI shows "consumption" on the right, thus `output` describes the item consumption numbers. The same goes for fluid consumption.
/// - The kills GUI shows "losses" on the right, so `output` describes how many of the force's entities were killed by enemies.
/// - The electric network GUI shows "power consumption" on the left side, so in this case `input` describes the power consumption numbers.
pub trait LuaFlowStatisticsMethods {
    /// Reset all the statistics data to 0.
    fn clear();
    /// Gets the flow count value for the given time frame. If `sample_index` is not provided, then the value returned is the average across the provided precision time period. These are the values shown in the bottom section of the statistics GUIs.
    ///
    /// Use `sample_index` to access the data used to generate the statistics graphs. Each precision level contains 300 samples of data so at a precision of 1 minute, each sample contains data averaged across 60s / 300 = 0.2s = 12 ticks.
    ///
    /// All return values are normalized to be per-tick for electric networks and per-minute for all other types.
    fn get_flow_count(count: bool, input: bool, name: String, precision_index: FlowPrecisionIndex, sample_index: u16) -> f64;
    /// Gets the total input count for a given prototype.
    fn get_input_count(name: String) -> LuaFlowStatisticsMethodsGetInputCountUnion;
    /// Gets the total output count for a given prototype.
    fn get_output_count(name: String) -> LuaFlowStatisticsMethodsGetOutputCountUnion;
    /// All methods and properties that this object supports.
    fn help() -> String;
    /// Adds a value to this flow statistics.
    fn on_flow(count: f32, name: String);
    /// Sets the total input count for a given prototype.
    fn set_input_count(count: LuaFlowStatisticsMethodsSetInputCountCountUnion, name: String);
    /// Sets the total output count for a given prototype.
    fn set_output_count(count: LuaFlowStatisticsMethodsSetOutputCountCountUnion, name: String);
}

/// An array of fluid boxes of an entity. Entities may contain more than one fluid box, and some can change the number of fluid boxes -- for instance, an assembling machine will change its number of fluid boxes depending on its active recipe. See [Fluid](Fluid).
///
/// Do note that reading from a [LuaFluidBox](LuaFluidBox) creates a new table and writing will copy the given fields from the table into the engine's own fluid box structure. Therefore, the correct way to update a fluidbox of an entity is to read it first, modify the table, then write the modified table back. Directly accessing the returned table's attributes won't have the desired effect.
pub struct LuaFluidBox {
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The entity that owns this fluidbox.
    pub owner: Box<LuaEntity>,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

pub enum LuaFluidBoxMethodsGetPrototypeUnion {
    LuaFluidBoxPrototype(LuaFluidBoxPrototype),
    Array(Vec<LuaFluidBoxPrototype>),
}

pub enum LuaFluidBoxMethodsSetFilterFilterUnion {
    FluidBoxFilterSpec(FluidBoxFilterSpec),
    Nil,
}

/// An array of fluid boxes of an entity. Entities may contain more than one fluid box, and some can change the number of fluid boxes -- for instance, an assembling machine will change its number of fluid boxes depending on its active recipe. See [Fluid](Fluid).
///
/// Do note that reading from a [LuaFluidBox](LuaFluidBox) creates a new table and writing will copy the given fields from the table into the engine's own fluid box structure. Therefore, the correct way to update a fluidbox of an entity is to read it first, modify the table, then write the modified table back. Directly accessing the returned table's attributes won't have the desired effect.
pub trait LuaFluidBoxMethods {
    /// Flushes all fluid from this fluidbox and its fluid system.
    fn flush(fluid: FluidIdentification, index: u32) -> HashMap<String, f32>;
    /// The capacity of the given fluidbox index.
    fn get_capacity(index: u32) -> f64;
    /// The fluidboxes to which the fluidbox at the given index is connected.
    fn get_connections(index: u32) -> Vec<LuaFluidBox>;
    /// Get a fluid box filter
    fn get_filter(index: u32) -> Option<FluidBoxFilter>;
    /// Flow through the fluidbox in the last tick. It is the larger of in-flow and out-flow.
    fn get_flow(index: u32) -> f64;
    /// Gets unique fluid system identifier of selected fluid box. May return nil for fluid wagon, fluid turret's internal buffer or a fluidbox which does not belong to a fluid system
    fn get_fluid_system_id(index: u32) -> u32;
    /// Returns the fluid the fluidbox is locked onto
    fn get_locked_fluid(index: u32) -> Option<String>;
    /// The prototype of this fluidbox index. If this is used on a fluidbox of a crafting machine which due to recipe was created by merging multiple prototypes, a table of prototypes that were merged will be returned instead
    fn get_prototype(index: u32) -> LuaFluidBoxMethodsGetPrototypeUnion;
    /// All methods and properties that this object supports.
    fn help() -> String;
    /// Set a fluid box filter.
    fn set_filter(filter: LuaFluidBoxMethodsSetFilterFilterUnion, index: u32) -> bool;
}

/// A prototype of a fluidbox owned by some [LuaEntityPrototype](LuaEntityPrototype).
pub struct LuaFluidBoxPrototype {
    pub base_area: f64,
    pub base_level: f64,
    /// The entity that this belongs to.
    pub entity: Box<LuaEntityPrototype>,
    /// The filter, if any is set.
    pub filter: Option<LuaFluidPrototype>,
    pub height: f64,
    /// The index of this fluidbox prototype in the owning entity.
    pub index: u32,
    /// The maximum temperature, if any is set.
    pub maximum_temperature: Option<f64>,
    /// The minimum temperature, if any is set.
    pub minimum_temperature: Option<f64>,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The pipe connection points.
    pub pipe_connections: Vec<FluidBoxConnection>,
    /// The production type. "input", "output", "input-output", or "none".
    pub production_type: String,
    /// The render layer.
    pub render_layer: String,
    /// The secondary draw orders for the 4 possible connection directions.
    pub secondary_draw_orders: Vec<i32>,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
    pub volume: f64,
}

/// A prototype of a fluidbox owned by some [LuaEntityPrototype](LuaEntityPrototype).
pub trait LuaFluidBoxPrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Prototype of a fluid energy source.
pub struct LuaFluidEnergySourcePrototype {
    pub burns_fluid: bool,
    pub destroy_non_fuel_fluid: bool,
    pub effectivity: f64,
    /// The emissions of this energy source in `pollution/Joule`. Multiplying it by energy consumption in `Watt` gives `pollution/second`.
    pub emissions: f64,
    /// The fluid box for this energy source.
    pub fluid_box: LuaFluidBoxPrototype,
    pub fluid_usage_per_tick: f64,
    pub maximum_temperature: f64,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    pub render_no_network_icon: bool,
    pub render_no_power_icon: bool,
    pub scale_fluid_usage: bool,
    /// The smoke sources for this prototype, if any.
    pub smoke: Vec<SmokeSource>,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of a fluid energy source.
pub trait LuaFluidEnergySourcePrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Prototype of a fluid.
pub struct LuaFluidPrototype {
    pub base_color: Color,
    /// Default temperature of this fluid.
    pub default_temperature: f64,
    /// A multiplier on the amount of emissions produced when this fluid is burnt in a generator. A value above `1.0` increases emissions and vice versa. The multiplier can't be negative.
    pub emissions_multiplier: f64,
    pub flow_color: Color,
    /// The amount of energy in Joules one unit of this fluid will produce when burnt in a generator. A value of `0` means this fluid can't be used for energy generation. The value can't be negative.
    pub fuel_value: f64,
    /// The temperature above which this fluid will be shown as gaseous inside tanks and pipes.
    pub gas_temperature: f64,
    /// Group of this prototype.
    pub group: LuaGroup,
    /// The amount of energy in Joules required to heat one unit of this fluid by 1°C.
    pub heat_capacity: f64,
    /// Whether this fluid is hidden from the fluid and signal selectors.
    pub hidden: bool,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    /// Maximum temperature this fluid can reach.
    pub max_temperature: f64,
    /// Name of this prototype.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    /// Subgroup of this prototype.
    pub subgroup: LuaGroup,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of a fluid.
pub trait LuaFluidPrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Prototype of a font.
pub struct LuaFontPrototype {
    pub border: bool,
    /// The border color, if any.
    pub border_color: Option<Color>,
    pub filtered: bool,
    pub from: String,
    /// Name of this prototype.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    pub size: i32,
    pub spacing: f32,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of a font.
pub trait LuaFontPrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// `LuaForce` encapsulates data local to each "force" or "faction" of the game. Default forces are player, enemy and neutral. Players and mods can create additional forces (up to 64 total).
pub struct LuaForce {
    /// Enables some higher-level AI behaviour for this force. When set to `true`, biters belonging to this force will automatically expand into new territories, build new spawners, and form unit groups. By default, this value is `true` for the enemy force and `false` for all others.
    pub ai_controllable: bool,
    pub artillery_range_modifier: f64,
    pub character_build_distance_bonus: u32,
    pub character_health_bonus: f64,
    /// the number of additional inventory slots the character main inventory has.
    pub character_inventory_slots_bonus: u32,
    pub character_item_drop_distance_bonus: u32,
    pub character_item_pickup_distance_bonus: f64,
    /// `true` if character requester logistics is enabled.
    pub character_logistic_requests: bool,
    pub character_loot_pickup_distance_bonus: f64,
    pub character_reach_distance_bonus: u32,
    pub character_resource_reach_distance_bonus: f64,
    /// Modifies the running speed of all characters in this force by the given value as a percentage. Setting the running modifier to `0.5` makes the character run 50% faster. The minimum value of `-1` reduces the movement speed by 100%, resulting in a speed of `0`.
    pub character_running_speed_modifier: f64,
    /// Number of character trash slots.
    pub character_trash_slot_count: f64,
    /// Effective color of this force.
    pub color: Color,
    /// The connected players belonging to this force.
    ///
    /// This is primarily useful when you want to do some action against all online players of this force.
    pub connected_players: Vec<LuaPlayer>,
    /// The currently ongoing technology research, if any.
    pub current_research: Option<LuaTechnology>,
    /// Custom color for this force. If specified, will take priority over other sources of the force color. Writing nil clears custom color. Will return nil if it was not specified or if was set to {0,0,0,0}
    pub custom_color: Option<Color>,
    /// The time, in ticks, before a deconstruction order is removed.
    pub deconstruction_time_to_live: u32,
    /// The entity build statistics for this force (built and mined)
    pub entity_build_count_statistics: LuaFlowStatistics,
    /// Evolution factor of this force.
    pub evolution_factor: f64,
    pub evolution_factor_by_killing_spawners: f64,
    pub evolution_factor_by_pollution: f64,
    pub evolution_factor_by_time: f64,
    /// The fluid production statistics for this force.
    pub fluid_production_statistics: LuaFlowStatistics,
    /// Additional lifetime for following robots.
    pub following_robots_lifetime_modifier: f64,
    /// If friendly fire is enabled for this force.
    pub friendly_fire: bool,
    /// The time, in ticks, before a placed ghost disappears.
    pub ghost_time_to_live: u32,
    /// This force's index in [LuaGameScript::forces](LuaGameScript::forces) (unique ID). It is assigned when a force is created, and remains so until it is [merged](on_forces_merged) (ie. deleted). Indexes of merged forces can be reused.
    pub index: u32,
    /// The inserter stack size bonus for non stack inserters
    pub inserter_stack_size_bonus: f64,
    /// The item production statistics for this force.
    pub item_production_statistics: LuaFlowStatistics,
    /// All of the items that have been launched in rockets. The attribute is a dictionary mapping the item prototype names to the launched amounts.
    pub items_launched: HashMap<String, u32>,
    /// The kill counter statistics for this force.
    pub kill_count_statistics: LuaFlowStatistics,
    pub laboratory_productivity_bonus: f64,
    pub laboratory_speed_modifier: f64,
    /// List of logistic networks, grouped by surface.
    pub logistic_networks: HashMap<String, Vec<LuaLogisticNetwork>>,
    /// Multiplier of the manual crafting speed. Default value is `0`. The actual crafting speed will be multiplied by `1 + manual_crafting_speed_modifier`.
    pub manual_crafting_speed_modifier: f64,
    /// Multiplier of the manual mining speed. Default value is `0`. The actual mining speed will be multiplied by `1 + manual_mining_speed_modifier`.
    pub manual_mining_speed_modifier: f64,
    pub max_failed_attempts_per_tick_per_construction_queue: u32,
    pub max_successful_attempts_per_tick_per_construction_queue: u32,
    /// Maximum number of follower robots.
    pub maximum_following_robot_count: u32,
    pub mining_drill_productivity_bonus: f64,
    /// Name of the force.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// Players belonging to this force.
    pub players: Vec<LuaPlayer>,
    /// The previous research, if any.
    pub previous_research: Option<LuaTechnology>,
    /// Recipes available to this force, indexed by `name`.
    pub recipes: HashMap<String, LuaRecipe>,
    /// Whether research is enabled for this force, see [LuaForce::enable_research](LuaForce::enable_research) and [LuaForce::disable_research](LuaForce::disable_research)
    pub research_enabled: bool,
    /// Progress of current research, as a number in range [0, 1].
    pub research_progress: f64,
    /// The research queue of this force. The first technology in the array is the currently active one. Reading this attribute gives an array of [LuaTechnology](LuaTechnology).
    ///
    /// To write to this, the entire table must be written. Providing an empty table or `nil` will empty the research queue and cancel the current research. Writing to this when the research queue is disabled will simply set the last research in the table as the current research.
    pub research_queue: Vec<TechnologyIdentification>,
    /// Whether the research queue is available for this force.
    pub research_queue_enabled: bool,
    /// The number of rockets launched.
    pub rockets_launched: u32,
    /// If sharing chart data is enabled for this force.
    pub share_chart: bool,
    /// Number of items that can be transferred by stack inserters. When writing to this value, it must be >= 0 and <= 254.
    pub stack_inserter_capacity_bonus: u32,
    /// Technologies owned by this force, indexed by `name`.
    pub technologies: HashMap<String, LuaTechnology>,
    pub train_braking_force_bonus: f64,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
    pub worker_robots_battery_modifier: f64,
    pub worker_robots_speed_modifier: f64,
    pub worker_robots_storage_bonus: f64,
    /// Ability to create new blueprints using empty blueprint item when using zoom-to-world.
    pub zoom_to_world_blueprint_enabled: bool,
    /// Ability to use deconstruction planner when using zoom-to-world.
    pub zoom_to_world_deconstruction_planner_enabled: bool,
    /// Ability to use zoom-to-world on map.
    pub zoom_to_world_enabled: bool,
    /// Ability to build ghosts through blueprint or direct ghost placement, or "mine" ghosts when using zoom-to-world.
    pub zoom_to_world_ghost_building_enabled: bool,
    /// Ability to use custom selection tools when using zoom-to-world.
    pub zoom_to_world_selection_tool_enabled: bool,
}

pub enum LuaForceMethodsGetHandCraftingDisabledForRecipeRecipeUnion {
    String(String),
    LuaRecipe(LuaRecipe),
}

pub enum LuaForceMethodsGetTrainStopsNameUnion {
    String(String),
    Array(Vec<String>),
}

pub enum LuaForceMethodsSetHandCraftingDisabledForRecipeRecipeUnion {
    String(String),
    LuaRecipe(LuaRecipe),
}

/// `LuaForce` encapsulates data local to each "force" or "faction" of the game. Default forces are player, enemy and neutral. Players and mods can create additional forces (up to 64 total).
pub trait LuaForceMethods {
    /// Adds a custom chart tag to the given surface and returns the new tag or `nil` if the given position isn't valid for a chart tag.
    fn add_chart_tag(surface: SurfaceIdentification, tag: ChartTagSpec) -> Option<LuaCustomChartTag>;
    /// Add this technology to the back of the research queue if the queue is enabled. Otherwise, set this technology to be researched now.
    fn add_research(technology: TechnologyIdentification) -> bool;
    /// Cancels pending chart requests for the given surface or all surfaces.
    fn cancel_charting(surface: SurfaceIdentification);
    /// Stop the research currently in progress. This will remove any dependent technologies from the research queue.
    fn cancel_current_research();
    /// Chart a portion of the map. The chart for the given area is refreshed; it creates chart for any parts of the given area that haven't been charted yet.
    fn chart(area: BoundingBox, surface: SurfaceIdentification);
    /// Chart all generated chunks.
    fn chart_all(surface: SurfaceIdentification);
    /// Erases chart data for this force.
    fn clear_chart(surface: SurfaceIdentification);
    /// Disable all recipes and technologies. Only recipes and technologies enabled explicitly will be useable from this point.
    fn disable_all_prototypes();
    /// Disable research for this force.
    fn disable_research();
    /// Enables all recipes and technologies. The opposite of [LuaForce::disable_all_prototypes](LuaForce::disable_all_prototypes)
    fn enable_all_prototypes();
    /// Unlock all recipes.
    fn enable_all_recipes();
    /// Unlock all technologies.
    fn enable_all_technologies();
    /// Enable research for this force.
    fn enable_research();
    /// Finds all custom chart tags within the given bounding box on the given surface.
    fn find_chart_tags(area: BoundingBox, surface: SurfaceIdentification) -> Vec<LuaCustomChartTag>;
    fn find_logistic_network_by_position(position: MapPosition, surface: SurfaceIdentification) -> Option<LuaLogisticNetwork>;
    fn get_ammo_damage_modifier(ammo: String) -> f64;
    /// Is `other` force in this force's cease fire list?
    fn get_cease_fire(other: ForceIdentification) -> bool;
    /// Count entities of given type.
    fn get_entity_count(name: String) -> u32;
    /// Is `other` force in this force's friends list.
    fn get_friend(other: ForceIdentification) -> bool;
    fn get_gun_speed_modifier(ammo: String) -> f64;
    /// Gets if the given recipe is explicitly disabled from being hand crafted.
    fn get_hand_crafting_disabled_for_recipe(recipe: LuaForceMethodsGetHandCraftingDisabledForRecipeRecipeUnion) -> bool;
    /// Gets the count of a given item launched in rockets.
    fn get_item_launched(item: String) -> u32;
    /// Gets the linked inventory for the given prototype and link ID if it exists or `nil`.
    fn get_linked_inventory(link_id: u32, prototype: EntityPrototypeIdentification) -> Option<LuaInventory>;
    /// Gets the saved progress for the given technology or `nil` if there is no saved progress.
    fn get_saved_technology_progress(technology: TechnologyIdentification) -> Option<f64>;
    fn get_spawn_position(surface: SurfaceIdentification) -> MapPosition;
    /// Gets train stops matching the given filters.
    fn get_train_stops(name: LuaForceMethodsGetTrainStopsNameUnion, surface: SurfaceIdentification) -> Vec<LuaEntity>;
    fn get_trains(surface: SurfaceIdentification) -> Vec<LuaTrain>;
    fn get_turret_attack_modifier(turret: String) -> f64;
    /// All methods and properties that this object supports.
    fn help() -> String;
    /// Has a chunk been charted?
    fn is_chunk_charted(position: ChunkPosition, surface: SurfaceIdentification) -> bool;
    /// Is the given chunk currently charted and visible (not covered by fog of war) on the map.
    fn is_chunk_visible(position: ChunkPosition, surface: SurfaceIdentification) -> bool;
    /// Is this force an enemy? This differs from `get_cease_fire` in that it is always false for neutral force. This is equivalent to checking the `enemy` ForceCondition.
    fn is_enemy(other: ForceIdentification) -> bool;
    /// Is this force a friend? This differs from `get_friend` in that it is always true for neutral force. This is equivalent to checking the `friend` ForceCondition.
    fn is_friend(other: ForceIdentification) -> bool;
    /// Is pathfinder busy? When the pathfinder is busy, it won't accept any more pathfinding requests.
    fn is_pathfinder_busy() -> bool;
    /// Kill all units and flush the pathfinder.
    fn kill_all_units();
    /// Play a sound for every player in this force.
    fn play_sound(override_sound_type: SoundType, path: SoundPath, position: MapPosition, volume_modifier: f64);
    /// Print text to the chat console of all players on this force.
    fn print(color: Color, message: LocalisedString);
    /// Force a rechart of the whole chart.
    fn rechart();
    /// Research all technologies.
    fn research_all_technologies(include_disabled_prototypes: bool);
    /// Reset everything. All technologies are set to not researched, all modifiers are set to default values.
    fn reset();
    /// Resets evolution for this force to zero.
    fn reset_evolution();
    /// Load the original version of all recipes from the prototypes.
    fn reset_recipes();
    /// Load the original versions of technologies from prototypes. Preserves research state of technologies.
    fn reset_technologies();
    /// Reapplies all possible research effects, including unlocked recipes. Any custom changes are lost. Preserves research state of technologies.
    fn reset_technology_effects();
    fn set_ammo_damage_modifier(ammo: String, modifier: f64);
    /// Add `other` force to this force's cease fire list. Forces on the cease fire list won't be targeted for attack.
    fn set_cease_fire(cease_fire: bool, other: ForceIdentification);
    /// Add `other` force to this force's friends list. Friends have unrestricted access to buildings and turrets won't fire at them.
    fn set_friend(friend: bool, other: ForceIdentification);
    fn set_gun_speed_modifier(ammo: String, modifier: f64);
    /// Sets if the given recipe can be hand-crafted. This is used to explicitly disable hand crafting a recipe - it won't allow hand-crafting otherwise not hand-craftable recipes.
    fn set_hand_crafting_disabled_for_recipe(hand_crafting_disabled: bool, recipe: LuaForceMethodsSetHandCraftingDisabledForRecipeRecipeUnion);
    /// Sets the count of a given item launched in rockets.
    fn set_item_launched(count: u32, item: String);
    /// Sets the saved progress for the given technology. The technology must not be in progress, must not be completed, and the new progress must be < 100%.
    fn set_saved_technology_progress(progress: f64, technology: TechnologyIdentification);
    fn set_spawn_position(position: MapPosition, surface: SurfaceIdentification);
    fn set_turret_attack_modifier(modifier: f64, turret: String);
    fn unchart_chunk(position: ChunkPosition, surface: SurfaceIdentification);
}

/// Prototype of a fuel category.
pub struct LuaFuelCategoryPrototype {
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    /// Name of this prototype.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of a fuel category.
pub trait LuaFuelCategoryPrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

pub enum LuaGameScriptForcesUnion {
    Uint(u32),
    String(String),
}

pub enum LuaGameScriptPlayersUnion {
    Uint(u32),
    String(String),
}

pub enum LuaGameScriptSurfacesUnion {
    Uint(u32),
    String(String),
}

/// Main toplevel type, provides access to most of the API though its members. An instance of LuaGameScript is available as the global object named `game`.
pub struct LuaGameScript {
    /// A dictionary containing every LuaAchievementPrototype indexed by `name`.
    pub achievement_prototypes: HashMap<String, LuaAchievementPrototype>,
    /// The active mods versions. The keys are mod names, the values are the versions.
    pub active_mods: HashMap<String, String>,
    /// A dictionary containing every LuaAmmoCategoryPrototype indexed by `name`.
    pub ammo_category_prototypes: HashMap<String, LuaAmmoCategoryPrototype>,
    /// A dictionary containing every LuaAutoplaceControlPrototype indexed by `name`.
    pub autoplace_control_prototypes: HashMap<String, LuaAutoplaceControlPrototype>,
    /// True by default. Can be used to disable autosaving. Make sure to turn it back on soon after.
    pub autosave_enabled: bool,
    /// Array of the names of all the backers that supported the game development early on. These are used as names for labs, locomotives, radars, roboports, and train stops.
    pub backer_names: HashMap<u32, String>,
    /// The players that are currently online.
    ///
    /// This is primarily useful when you want to do some action against all online players.
    pub connected_players: Vec<LuaPlayer>,
    /// Whether a console command has been used.
    pub console_command_used: bool,
    /// A dictionary containing every LuaCustomInputPrototype indexed by `name`.
    pub custom_input_prototypes: HashMap<String, LuaCustomInputPrototype>,
    /// A dictionary containing every LuaDamagePrototype indexed by `name`.
    pub damage_prototypes: HashMap<String, LuaDamagePrototype>,
    /// A dictionary containing every LuaDecorativePrototype indexed by `name`.
    pub decorative_prototypes: HashMap<String, LuaDecorativePrototype>,
    /// The default map gen settings for this save.
    pub default_map_gen_settings: MapGenSettings,
    /// Current scenario difficulty.
    pub difficulty: Difficulty,
    /// The currently active set of difficulty settings. Even though this property is marked as read-only, the members of the dictionary that is returned can be modified mid-game. This is however not recommended as different difficulties can have differing technology and recipe trees, which can cause problems for players.
    pub difficulty_settings: DifficultySettings,
    /// True by default. Can be used to disable the highlighting of resource patches when they are hovered on the map.
    pub draw_resource_selection: bool,
    /// Determines if enemy land mines are completely invisible or not.
    pub enemy_has_vision_on_land_mines: bool,
    /// A dictionary containing every LuaEntityPrototype indexed by `name`.
    pub entity_prototypes: HashMap<String, LuaEntityPrototype>,
    /// A dictionary containing every LuaEquipmentCategoryPrototype indexed by `name`.
    pub equipment_category_prototypes: HashMap<String, LuaEquipmentCategoryPrototype>,
    /// A dictionary containing every LuaEquipmentGridPrototype indexed by `name`.
    pub equipment_grid_prototypes: HashMap<String, LuaEquipmentGridPrototype>,
    /// A dictionary containing every LuaEquipmentPrototype indexed by `name`.
    pub equipment_prototypes: HashMap<String, LuaEquipmentPrototype>,
    /// True while the victory screen is shown.
    pub finished: bool,
    /// True after players finished the game and clicked "continue".
    pub finished_but_continuing: bool,
    /// A dictionary containing every LuaFluidPrototype indexed by `name`.
    pub fluid_prototypes: HashMap<String, LuaFluidPrototype>,
    /// A dictionary containing every LuaFontPrototype indexed by `name`.
    pub font_prototypes: HashMap<String, LuaFontPrototype>,
    /// Get a table of all the forces that currently exist. This sparse table allows you to find forces by indexing it with either their `name` or `index`. Iterating this table with `pairs()` will only iterate the array part of the table. Iterating with `ipairs()` will not work at all.
    pub forces: HashMap<LuaGameScriptForcesUnion, LuaForce>,
    /// A dictionary containing every LuaFuelCategoryPrototype indexed by `name`.
    pub fuel_category_prototypes: HashMap<String, LuaFuelCategoryPrototype>,
    /// A dictionary containing every ItemGroup indexed by `name`.
    pub item_group_prototypes: HashMap<String, LuaGroup>,
    /// A dictionary containing every LuaItemPrototype indexed by `name`.
    pub item_prototypes: HashMap<String, LuaItemPrototype>,
    /// A dictionary containing every ItemSubgroup indexed by `name`.
    pub item_subgroup_prototypes: HashMap<String, LuaGroup>,
    /// A dictionary containing every MapGenPreset indexed by `name`.
    pub map_gen_presets: HashMap<String, MapGenPreset>,
    /// The currently active set of map settings. Even though this property is marked as read-only, the members of the dictionary that is returned can be modified mid-game.
    pub map_settings: MapSettings,
    pub max_beacon_supply_area_distance: f64,
    pub max_electric_pole_connection_distance: f64,
    pub max_electric_pole_supply_area_distance: f32,
    pub max_force_distraction_chunk_distance: u32,
    pub max_force_distraction_distance: f64,
    pub max_gate_activation_distance: f64,
    pub max_inserter_reach_distance: f64,
    pub max_pipe_to_ground_distance: u8,
    pub max_underground_belt_distance: u8,
    /// A dictionary containing every LuaModSettingPrototype indexed by `name`.
    pub mod_setting_prototypes: HashMap<String, LuaModSettingPrototype>,
    /// A dictionary containing every LuaModuleCategoryPrototype indexed by `name`.
    pub module_category_prototypes: HashMap<String, LuaModuleCategoryPrototype>,
    /// A dictionary containing every LuaNamedNoiseExpression indexed by `name`.
    pub named_noise_expressions: HashMap<String, LuaNamedNoiseExpression>,
    /// A dictionary containing every LuaNoiseLayerPrototype indexed by `name`.
    pub noise_layer_prototypes: HashMap<String, LuaNoiseLayerPrototype>,
    /// This object's name.
    pub object_name: String,
    /// A dictionary containing every LuaParticlePrototype indexed by `name`.
    pub particle_prototypes: HashMap<String, LuaParticlePrototype>,
    pub permissions: LuaPermissionGroups,
    /// This property is only populated inside [custom command](LuaCommandProcessor) handlers and when writing [Lua console commands](https://wiki.factorio.com/Console#Scripting_and_cheat_commands). Returns the player that is typing the command, `nil` in all other instances.
    ///
    /// See [LuaGameScript::players](LuaGameScript::players) for accessing all players.
    pub player: Option<LuaPlayer>,
    /// Get a table of all the players that currently exist. This sparse table allows you to find players by indexing it with either their `name` or `index`. Iterating this table with `pairs()` will only iterate the array part of the table. Iterating with `ipairs()` will not work at all.
    ///
    /// If only a single player is required, [LuaGameScript::get_player](LuaGameScript::get_player) should be used instead, as it avoids the unnecessary overhead of passing the whole table to Lua.
    pub players: HashMap<LuaGameScriptPlayersUnion, LuaPlayer>,
    /// The pollution statistics for this map.
    pub pollution_statistics: LuaFlowStatistics,
    /// A dictionary containing every LuaRecipeCategoryPrototype indexed by `name`.
    pub recipe_category_prototypes: HashMap<String, LuaRecipeCategoryPrototype>,
    /// A dictionary containing every LuaRecipePrototype indexed by `name`.
    pub recipe_prototypes: HashMap<String, LuaRecipePrototype>,
    /// A dictionary containing every LuaResourceCategoryPrototype indexed by `name`.
    pub resource_category_prototypes: HashMap<String, LuaResourceCategoryPrototype>,
    /// A dictionary containing every LuaShortcutPrototype indexed by `name`.
    pub shortcut_prototypes: HashMap<String, LuaShortcutPrototype>,
    /// Speed to update the map at. 1.0 is normal speed -- 60 UPS.
    pub speed: f32,
    /// The styles that [LuaGuiElement](LuaGuiElement) can use, indexed by `name`.
    pub styles: HashMap<String, String>,
    /// Get a table of all the surfaces that currently exist. This sparse table allows you to find surfaces by indexing it with either their `name` or `index`. Iterating this table with `pairs()` will only iterate the array part of the table. Iterating with `ipairs()` will not work at all.
    pub surfaces: HashMap<LuaGameScriptSurfacesUnion, LuaSurface>,
    /// A dictionary containing every [LuaTechnologyPrototype](LuaTechnologyPrototype) indexed by `name`.
    pub technology_prototypes: HashMap<String, LuaTechnologyPrototype>,
    /// Current map tick.
    pub tick: u32,
    /// If the tick has been paused. This means that entity update has been paused.
    pub tick_paused: bool,
    /// The number of ticks since this game was 'created'. A game is 'created' either by using "new game" or "new game from scenario".
    pub ticks_played: u32,
    /// The number of ticks to be run while the tick is paused. When [LuaGameScript::tick_paused](LuaGameScript::tick_paused) is true, ticks_to_run behaves the following way: While this is > 0, the entity update is running normally and this value is decremented every tick. When this reaches 0, the game will pause again.
    pub ticks_to_run: u32,
    /// A dictionary containing every LuaTilePrototype indexed by `name`.
    pub tile_prototypes: HashMap<String, LuaTilePrototype>,
    /// A dictionary containing every LuaTrivialSmokePrototype indexed by `name`.
    pub trivial_smoke_prototypes: HashMap<String, LuaTrivialSmokePrototype>,
    /// A dictionary containing every LuaVirtualSignalPrototype indexed by `name`.
    pub virtual_signal_prototypes: HashMap<String, LuaVirtualSignalPrototype>,
}

pub enum LuaGameScriptMethodsDeleteSurfaceSurfaceUnion {
    String(String),
    LuaSurface(LuaSurface),
}

pub enum LuaGameScriptMethodsGetPlayerPlayerUnion {
    Uint(u32),
    String(String),
}

pub enum LuaGameScriptMethodsGetSurfaceSurfaceUnion {
    Uint(u32),
    String(String),
}

pub enum LuaGameScriptMethodsGetTrainStopsNameUnion {
    String(String),
    Array(Vec<String>),
}

pub enum LuaGameScriptMethodsRegenerateEntityEntitiesUnion {
    String(String),
    Array(Vec<String>),
}

pub enum LuaGameScriptMethodsRemoveOfflinePlayersPlayersUnion {
    LuaPlayer(LuaPlayer),
    String(String),
}

/// Main toplevel type, provides access to most of the API though its members. An instance of LuaGameScript is available as the global object named `game`.
pub trait LuaGameScriptMethods {
    /// Instruct the game to perform an auto-save.
    fn auto_save(name: String);
    /// Bans the given player from this multiplayer game. Does nothing if this is a single player game of if the player running this isn't an admin.
    fn ban_player(player: PlayerIdentification, reason: LocalisedString);
    /// Run internal consistency checks. Allegedly prints any errors it finds.
    fn check_consistency();
    /// Goes over all items, entities, tiles, recipes, technologies among other things and logs if the locale is incorrect.
    fn check_prototype_translations();
    /// Counts how many distinct groups of pipes exist in the world.
    fn count_pipe_groups();
    /// Create a new force.
    fn create_force(force: String) -> LuaForce;
    /// Creates an inventory that is not owned by any game object. It can be resized later with [LuaInventory::resize](LuaInventory::resize).
    fn create_inventory(size: u16) -> LuaInventory;
    /// Creates a [LuaProfiler](LuaProfiler), which is used for measuring script performance.
    fn create_profiler(stopped: bool) -> LuaProfiler;
    /// Creates a deterministic standalone random generator with the given seed or if a seed is not provided the initial map seed is used.
    fn create_random_generator(seed: u32) -> LuaRandomGenerator;
    /// Create a new surface.
    fn create_surface(name: String, settings: MapGenSettings) -> LuaSurface;
    /// Base64 decodes and inflates the given string.
    fn decode_string(string: String) -> Option<String>;
    /// Deletes the given surface and all entities on it.
    fn delete_surface(surface: LuaGameScriptMethodsDeleteSurfaceSurfaceUnion);
    /// Converts the given direction into the string version of the direction.
    fn direction_to_string(direction: Direction);
    /// Disables replay saving for the current save file. Once done there's no way to re-enable replay saving for the save file without loading an old save.
    fn disable_replay();
    /// Disables tutorial triggers, that unlock new tutorials and show notices about unlocked tutorials.
    fn disable_tutorial_triggers();
    /// Deflates and base64 encodes the given string.
    fn encode_string(string: String) -> Option<String>;
    /// Evaluate an expression, substituting variables as provided. For details on the formula, see the relevant page on the [Factorio wiki](https://wiki.factorio.com/Prototype/Technology#unit).
    fn evaluate_expression(expression: String, variables: HashMap<String, f64>) -> f64;
    /// Force a CRC check. Tells all peers to calculate their current CRC, which are then compared to each other. If a mismatch is detected, the game desyncs and some peers are forced to reconnect.
    fn force_crc();
    /// Gets the number of entities that are active (updated each tick).
    fn get_active_entities_count(surface: SurfaceIdentification) -> u32;
    fn get_entity_by_tag(tag: String) -> Option<LuaEntity>;
    /// Returns a dictionary of all LuaAchievementPrototypes that fit the given filters. The prototypes are indexed by `name`.
    fn get_filtered_achievement_prototypes(filters: Vec<AchievementPrototypeFilter>) -> HashMap<String, LuaAchievementPrototype>;
    /// Returns a dictionary of all LuaDecorativePrototypes that fit the given filters. The prototypes are indexed by `name`.
    fn get_filtered_decorative_prototypes(filters: Vec<DecorativePrototypeFilter>) -> HashMap<String, LuaDecorativePrototype>;
    /// Returns a dictionary of all LuaEntityPrototypes that fit the given filters. The prototypes are indexed by `name`.
    fn get_filtered_entity_prototypes(filters: Vec<EntityPrototypeFilter>) -> HashMap<String, LuaEntityPrototype>;
    /// Returns a dictionary of all LuaEquipmentPrototypes that fit the given filters. The prototypes are indexed by `name`.
    fn get_filtered_equipment_prototypes(filters: Vec<EquipmentPrototypeFilter>) -> HashMap<String, LuaEquipmentPrototype>;
    /// Returns a dictionary of all LuaFluidPrototypes that fit the given filters. The prototypes are indexed by `name`.
    fn get_filtered_fluid_prototypes(filters: Vec<FluidPrototypeFilter>) -> HashMap<String, LuaFluidPrototype>;
    /// Returns a dictionary of all LuaItemPrototypes that fit the given filters. The prototypes are indexed by `name`.
    fn get_filtered_item_prototypes(filters: Vec<ItemPrototypeFilter>) -> HashMap<String, LuaItemPrototype>;
    /// Returns a dictionary of all LuaModSettingPrototypes that fit the given filters. The prototypes are indexed by `name`.
    fn get_filtered_mod_setting_prototypes(filters: Vec<ModSettingPrototypeFilter>) -> HashMap<String, LuaModSettingPrototype>;
    /// Returns a dictionary of all LuaRecipePrototypes that fit the given filters. The prototypes are indexed by `name`.
    fn get_filtered_recipe_prototypes(filters: Vec<RecipePrototypeFilter>) -> HashMap<String, LuaRecipePrototype>;
    /// Returns a dictionary of all LuaTechnologyPrototypes that fit the given filters. The prototypes are indexed by `name`.
    fn get_filtered_technology_prototypes(filters: Vec<TechnologyPrototypeFilter>) -> HashMap<String, LuaTechnologyPrototype>;
    /// Returns a dictionary of all LuaTilePrototypes that fit the given filters. The prototypes are indexed by `name`.
    fn get_filtered_tile_prototypes(filters: Vec<TilePrototypeFilter>) -> HashMap<String, LuaTilePrototype>;
    /// Gets the map exchange string for the map generation settings that were used to create this map.
    fn get_map_exchange_string() -> String;
    /// Gets the given player or returns `nil` if no player is found.
    fn get_player(player: LuaGameScriptMethodsGetPlayerPlayerUnion) -> Option<LuaPlayer>;
    /// Gets the inventories created through [LuaGameScript::create_inventory](LuaGameScript::create_inventory)
    fn get_script_inventories(mod_name: String) -> HashMap<String, Vec<LuaInventory>>;
    /// Gets the given surface or returns `nil` if no surface is found.
    fn get_surface(surface: LuaGameScriptMethodsGetSurfaceSurfaceUnion) -> Option<LuaSurface>;
    /// Gets train stops matching the given filters.
    fn get_train_stops(force: ForceIdentification, name: LuaGameScriptMethodsGetTrainStopsNameUnion, surface: SurfaceIdentification) -> Vec<LuaEntity>;
    /// Is this the demo version of Factorio?
    fn is_demo() -> bool;
    /// Is the map loaded is multiplayer?
    fn is_multiplayer() -> bool;
    /// Checks if the given SoundPath is valid.
    fn is_valid_sound_path(sound_path: SoundPath) -> bool;
    /// Checks if the given SpritePath is valid and contains a loaded sprite. The existence of the image is not checked for paths of type `file`.
    fn is_valid_sprite_path(sprite_path: SpritePath) -> bool;
    /// Convert a JSON string to a table.
    fn json_to_table(json: String) -> Option<AnyBasic>;
    /// Kicks the given player from this multiplayer game. Does nothing if this is a single player game or if the player running this isn't an admin.
    fn kick_player(player: PlayerIdentification, reason: LocalisedString);
    /// Marks two forces to be merged together. All players and entities in the source force will be reassigned to the target force. The source force will then be destroyed. Importantly, this does not merge technologies or bonuses, which are instead retained from the target force.
    fn merge_forces(destination: ForceIdentification, source: ForceIdentification);
    /// Mutes the given player. Does nothing if the player running this isn't an admin.
    fn mute_player(player: PlayerIdentification);
    /// Convert a map exchange string to map gen settings and map settings.
    fn parse_map_exchange_string(map_exchange_string: String) -> MapExchangeStringData;
    /// Play a sound for every player in the game.
    fn play_sound(override_sound_type: SoundType, path: SoundPath, position: MapPosition, volume_modifier: f64);
    /// Print text to the chat console all players.
    fn print(color: Color, message: LocalisedString);
    /// Purges the given players messages from the game. Does nothing if the player running this isn't an admin.
    fn purge_player(player: PlayerIdentification);
    /// Regenerate autoplacement of some entities on all surfaces. This can be used to autoplace newly-added entities.
    fn regenerate_entity(entities: LuaGameScriptMethodsRegenerateEntityEntitiesUnion);
    /// Forces a reload of all mods.
    fn reload_mods();
    /// Forces a reload of the scenario script from the original scenario location.
    fn reload_script();
    /// Remove players who are currently not connected from the map.
    fn remove_offline_players(players: Vec<LuaGameScriptMethodsRemoveOfflinePlayersPlayersUnion>);
    /// Remove a file or directory in the `script-output` folder, located in the game's [user data directory](https://wiki.factorio.com/User_data_directory). Can be used to remove files created by [LuaGameScript::write_file](LuaGameScript::write_file).
    fn remove_path(path: String);
    /// Reset scenario state (game_finished, player_won, etc.).
    fn reset_game_state();
    /// Resets the amount of time played for this map.
    fn reset_time_played();
    /// Saves the current configuration of Atlas to a file. This will result in huge file containing all of the game graphics moved to as small space as possible.
    fn save_atlas();
    /// Instruct the server to save the map.
    fn server_save(name: String);
    /// Set scenario state. Any parameters not provided do not change the current state.
    fn set_game_state(can_continue: bool, game_finished: bool, next_level: String, player_won: bool, victorious_force: ForceIdentification);
    /// Forces the screenshot saving system to wait until all queued screenshots have been written to disk.
    fn set_wait_for_screenshots_to_finish();
    /// Show an in-game message dialog.
    fn show_message_dialog(image: String, point_to: GuiArrowSpecification, style: String, text: LocalisedString, wrapper_frame_style: String);
    /// Convert a table to a JSON string
    fn table_to_json(data: table) -> String;
    /// Take a screenshot of the game and save it to the `script-output` folder, located in the game's [user data directory](https://wiki.factorio.com/User_data_directory). The name of the image file can be specified via the `path` parameter.
    fn take_screenshot(allow_in_replay: bool, anti_alias: bool, by_player: PlayerIdentification, daytime: f64, force_render: bool, path: String, player: PlayerIdentification, position: MapPosition, quality: i32, resolution: TilePosition, show_cursor_building_preview: bool, show_entity_info: bool, show_gui: bool, surface: SurfaceIdentification, water_tick: u32, zoom: f64);
    /// Take a screenshot of the technology screen and save it to the `script-output` folder, located in the game's [user data directory](https://wiki.factorio.com/User_data_directory). The name of the image file can be specified via the `path` parameter.
    fn take_technology_screenshot(by_player: PlayerIdentification, force: ForceIdentification, path: String, quality: i32, selected_technology: TechnologyIdentification, skip_disabled: bool);
    /// Unbans the given player from this multiplayer game. Does nothing if this is a single player game of if the player running this isn't an admin.
    fn unban_player(player: PlayerIdentification);
    /// Unmutes the given player. Does nothing if the player running this isn't an admin.
    fn unmute_player(player: PlayerIdentification);
    /// Write a file to the `script-output` folder, located in the game's [user data directory](https://wiki.factorio.com/User_data_directory). The name and file extension of the file can be specified via the `filename` parameter.
    fn write_file(append: bool, data: LocalisedString, filename: String, for_player: u32);
}

/// An abstract base class for behaviors that support switching the entity on or off based on some condition.
pub struct LuaGenericOnOffControlBehavior {
    pub lua_control_behavior: Box<LuaControlBehavior>,
    /// The circuit condition. Writing `nil` clears the circuit condition.
    pub circuit_condition: CircuitConditionDefinition,
    /// `true` if this should connect to the logistic network.
    pub connect_to_logistic_network: bool,
    /// If the entity is currently disabled because of the control behavior.
    pub disabled: bool,
    /// The logistic condition. Writing `nil` clears the logistic condition.
    pub logistic_condition: CircuitConditionDefinition,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// An abstract base class for behaviors that support switching the entity on or off based on some condition.
pub trait LuaGenericOnOffControlBehaviorMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Item group or subgroup.
pub struct LuaGroup {
    /// The parent group, if any.
    pub group: Option<Box<LuaGroup>>,
    /// Localised name of the group.
    pub localised_name: Option<LocalisedString>,
    pub name: Option<String>,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    /// The additional order value used in recipe ordering. Can only be used on groups, not on subgroups.
    pub order_in_recipe: Option<String>,
    /// Subgroups of this group. Can only be used on groups, not on subgroups.
    pub subgroups: Option<Vec<LuaGroup>>,
    pub typ: Option<String>,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Item group or subgroup.
pub trait LuaGroupMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// The root of the GUI. This type houses the root elements, `top`, `left`, `center`, `goal`, and `screen`, to which other elements can be added to be displayed on screen.
pub struct LuaGui {
    /// The center part of the GUI. It is a flow element.
    pub center: LuaGuiElement,
    /// The children GUI elements mapped by name <> element.
    pub children: HashMap<String, LuaGuiElement>,
    /// The flow used in the objectives window. It is a flow element. The objectives window is only visible when the flow is not empty or the objective text is set.
    pub goal: LuaGuiElement,
    /// The left part of the GUI. It is a flow element inside a scroll pane element.
    pub left: LuaGuiElement,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The player who owns this gui.
    pub player: LuaPlayer,
    /// For showing a GUI somewhere relative to one of the game GUIs. It is an empty-widget element.
    pub relative: LuaGuiElement,
    /// For showing a GUI somewhere on the entire screen. It is an empty-widget element.
    pub screen: LuaGuiElement,
    /// The top part of the GUI. It is a flow element inside a scroll pane element.
    pub top: LuaGuiElement,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// The root of the GUI. This type houses the root elements, `top`, `left`, `center`, `goal`, and `screen`, to which other elements can be added to be displayed on screen.
pub trait LuaGuiMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
    /// Returns `true` if sprite_path is valid and contains loaded sprite, otherwise `false`. Sprite path of type `file` doesn't validate if file exists.
    ///
    /// If you want to avoid needing a LuaGui object, [LuaGameScript::is_valid_sprite_path](LuaGameScript::is_valid_sprite_path) can be used instead.
    fn is_valid_sprite_path(sprite_path: SpritePath) -> bool;
}

pub enum LuaGuiElementElemValueUnion {
    String(String),
    SignalID(SignalID),
}

pub enum LuaGuiElementStyleUnion {
    LuaStyle(LuaStyle),
    String(String),
}

/// An element of a custom GUI. This type is used to represent any kind of a GUI element - labels, buttons and frames are all instances of this type. Just like [LuaEntity](LuaEntity), different kinds of elements support different attributes; attempting to access an attribute on an element that doesn't support it (for instance, trying to access the `column_count` of a `textfield`) will result in a runtime error.
///
/// The following types of GUI element are supported:
///
/// - `"button"`: A clickable element. Relevant event: [on_gui_click](on_gui_click)
/// - `"sprite-button"`: A `button` that displays a sprite rather than text. Relevant event: [on_gui_click](on_gui_click)
/// - `"checkbox"`: A clickable element with a check mark that can be turned off or on. Relevant event: [on_gui_checked_state_changed](on_gui_checked_state_changed)
/// - `"flow"`: An invisible container that lays out its children either horizontally or vertically.
/// - `"frame"`: A non-transparent box that contains other elements. It can have a title (set via the `caption` attribute). Just like a `flow`, it lays out its children either horizontally or vertically. Relevant event: [on_gui_location_changed](on_gui_location_changed)
/// - `"label"`: A piece of text.
/// - `"line"`: A horizontal or vertical separation line.
/// - `"progressbar"`: A partially filled bar that can be used to indicate progress.
/// - `"table"`: An invisible container that lays out its children in a specific number of columns. The width of each column is determined by the widest element it contains.
/// - `"textfield"`: A single-line box the user can type into. Relevant events: [on_gui_text_changed](on_gui_text_changed), [on_gui_confirmed](on_gui_confirmed)
/// - `"radiobutton"`: An element that is similar to a `checkbox`, but with a circular appearance. Clicking a selected radio button will not deselect it. Radio buttons are not linked to each other in any way. Relevant event: [on_gui_checked_state_changed](on_gui_checked_state_changed)
/// - `"sprite"`: An element that shows an image.
/// - `"scroll-pane"`: An invisible element that is similar to a `flow`, but has the ability to show and use scroll bars.
/// - `"drop-down"`: A drop-down containing strings of text. Relevant event: [on_gui_selection_state_changed](on_gui_selection_state_changed)
/// - `"list-box"`: A list of strings, only one of which can be selected at a time. Shows a scroll bar if necessary. Relevant event: [on_gui_selection_state_changed](on_gui_selection_state_changed)
/// - `"camera"`: A camera that shows the game at the given position on the given surface. It can visually track an [entity](LuaGuiElement::entity) that is set after the element has been created.
/// - `"choose-elem-button"`: A button that lets the player pick from a certain kind of prototype, with optional filtering. Relevant event: [on_gui_elem_changed](on_gui_elem_changed)
/// - `"text-box"`: A multi-line `textfield`. Relevant event: [on_gui_text_changed](on_gui_text_changed)
/// - `"slider"`: A horizontal number line which can be used to choose a number. Relevant event: [on_gui_value_changed](on_gui_value_changed)
/// - `"minimap"`: A minimap preview, similar to the normal player minimap. It can visually track an [entity](LuaGuiElement::entity) that is set after the element has been created.
/// - `"entity-preview"`: A preview of an entity. The [entity](LuaGuiElement::entity) has to be set after the element has been created.
/// - `"empty-widget"`: An empty element that just exists. The root GUI elements `screen` and `relative` are `empty-widget`s.
/// - `"tabbed-pane"`: A collection of `tab`s and their contents. Relevant event: [on_gui_selected_tab_changed](on_gui_selected_tab_changed)
/// - `"tab"`: A tab for use in a `tabbed-pane`.
/// - `"switch"`: A switch with three possible states. Can have labels attached to either side. Relevant event: [on_gui_switch_state_changed](on_gui_switch_state_changed)
///
/// Each GUI element allows access to its children by having them as attributes. Thus, one can use the `parent.child` syntax to refer to children. Lua also supports the `parent["child"]` syntax to refer to the same element. This can be used in cases where the child has a name that isn't a valid Lua identifier.
pub struct LuaGuiElement {
    /// Whether this textfield (when in numeric mode) allows decimal numbers.
    pub allow_decimal: bool,
    /// Whether this textfield (when in numeric mode) allows negative numbers.
    pub allow_negative: bool,
    /// Whether the `"none"` state is allowed for this switch.
    pub allow_none_state: bool,
    /// The anchor for this relative widget, if any. Setting `nil` clears the anchor.
    pub anchor: Option<GuiAnchor>,
    /// Whether this frame auto-centers on window resize when stored in [LuaGui::screen](LuaGui::screen).
    pub auto_center: bool,
    /// The text to display after the normal tab text (designed to work with numbers)
    pub badge_text: LocalisedString,
    /// The text displayed on this element. For frames, this is the "heading". For other elements, like buttons or labels, this is the content.
    pub caption: LocalisedString,
    /// The child-elements of this GUI element.
    pub children: Vec<LuaGuiElement>,
    /// Names of all the children of this element. These are the identifiers that can be used to access the child as an attribute of this element.
    pub children_names: Vec<String>,
    /// Makes it so right-clicking on this textfield clears and focuses it.
    pub clear_and_focus_on_right_click: bool,
    /// The sprite to display on this sprite-button when it is clicked.
    pub clicked_sprite: SpritePath,
    /// The number of columns in this table.
    pub column_count: u32,
    /// Direction of this element's layout. May be either `"horizontal"` or `"vertical"`.
    pub direction: String,
    /// The `frame` that is being moved when dragging this GUI element, if any. This element needs to be a child of the `drag_target` at some level.
    pub drag_target: Option<Box<LuaGuiElement>>,
    /// Whether this table should draw a horizontal grid line below the first table row.
    pub draw_horizontal_line_after_headers: bool,
    /// Whether this table should draw horizontal grid lines.
    pub draw_horizontal_lines: bool,
    /// Whether this table should draw vertical grid lines.
    pub draw_vertical_lines: bool,
    /// The elem filters of this choose-elem-button, if any. The compatible type of filter is determined by `elem_type`.
    pub elem_filters: Option<PrototypeFilter>,
    /// The elem type of this choose-elem-button.
    pub elem_type: String,
    /// The elem value of this choose-elem-button, if any.
    pub elem_value: Option<LuaGuiElementElemValueUnion>,
    /// Whether this GUI element is enabled. Disabled GUI elements don't trigger events when clicked.
    pub enabled: bool,
    /// The entity associated with this entity-preview, camera, minimap, if any.
    pub entity: Option<LuaEntity>,
    /// The force this minimap is using, if any.
    pub force: Option<String>,
    /// The GUI this element is a child of.
    pub gui: Box<LuaGui>,
    /// Policy of the horizontal scroll bar. Possible values are `"auto"`, `"never"`, `"always"`, `"auto-and-reserve-space"`, `"dont-show-but-allow-scrolling"`.
    pub horizontal_scroll_policy: String,
    /// The sprite to display on this sprite-button when it is hovered.
    pub hovered_sprite: SpritePath,
    /// Whether this GUI element is ignored by interaction. This makes clicks on this element 'go through' to the GUI element or even the game surface below it.
    pub ignored_by_interaction: bool,
    /// The index of this GUI element (unique amongst the GUI elements of a LuaPlayer).
    pub index: u32,
    /// Whether this textfield displays as a password field, which renders all characters as `*`.
    pub is_password: bool,
    /// The items in this dropdown or listbox.
    pub items: Vec<LocalisedString>,
    /// The text shown for the left switch label.
    pub left_label_caption: LocalisedString,
    /// The tooltip shown on the left switch label.
    pub left_label_tooltip: LocalisedString,
    /// The location of this widget when stored in [LuaGui::screen](LuaGui::screen). `nil` if not set or not in [LuaGui::screen](LuaGui::screen).
    pub location: Option<GuiLocation>,
    /// Whether this choose-elem-button can be changed by the player.
    pub locked: bool,
    /// Whether this textfield loses focus after [defines.events.on_gui_confirmed](defines.events.on_gui_confirmed) is fired.
    pub lose_focus_on_confirm: bool,
    /// The player index this minimap is using.
    pub minimap_player_index: u32,
    /// The mouse button filters for this button or sprite-button.
    pub mouse_button_filter: MouseButtonFlags,
    /// The name of this element. `""` if no name was set.
    pub name: String,
    /// The number to be shown in the bottom right corner of this sprite-button. Set this to `nil` to show nothing.
    pub number: f64,
    /// Whether this textfield is limited to only numberic characters.
    pub numeric: bool,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The direct parent of this element. `nil` if this is a top-level element.
    pub parent: Option<Box<LuaGuiElement>>,
    /// Index into [LuaGameScript::players](LuaGameScript::players) specifying the player who owns this element.
    pub player_index: u32,
    /// The position this camera or minimap is focused on, if any.
    pub position: MapPosition,
    /// Whether this element will raise [on_gui_hover](on_gui_hover) and [on_gui_leave](on_gui_leave).
    pub raise_hover_events: bool,
    /// Whether this text-box is read-only. Defaults to `false`.
    pub read_only: bool,
    /// Whether the sprite widget should resize according to the sprite in it. Defaults to `true`.
    pub resize_to_sprite: bool,
    /// The text shown for the right switch label.
    pub right_label_caption: LocalisedString,
    /// The tooltip shown on the right switch label.
    pub right_label_tooltip: LocalisedString,
    /// Whether the contents of this text-box are selectable. Defaults to `true`.
    pub selectable: bool,
    /// The selected index for this dropdown or listbox. Returns `0` if none is selected.
    pub selected_index: u32,
    /// The selected tab index for this tabbed pane, if any.
    pub selected_tab_index: Option<u32>,
    /// Related to the number to be shown in the bottom right corner of this sprite-button. When set to `true`, numbers that are non-zero and smaller than one are shown as a percentage rather than the value. For example, `0.5` will be shown as `50%` instead.
    pub show_percent_for_small_numbers: bool,
    /// The value of this slider element.
    pub slider_value: f64,
    /// The sprite to display on this sprite-button or sprite in the default state.
    pub sprite: SpritePath,
    /// Is this checkbox or radiobutton checked?
    pub state: bool,
    /// The style of this element. When read, this evaluates to a [LuaStyle](LuaStyle). For writing, it only accepts a string that specifies the textual identifier (prototype name) of the desired style.
    pub style: LuaGuiElementStyleUnion,
    /// The surface index this camera or minimap is using.
    pub surface_index: u32,
    /// The switch state (left, none, right) for this switch.
    pub switch_state: String,
    /// The tabs and contents being shown in this tabbed-pane.
    pub tabs: Vec<TabAndContent>,
    /// The tags associated with this LuaGuiElement.
    pub tags: Tags,
    /// The text contained in this textfield or text-box.
    pub text: String,
    pub tooltip: LocalisedString,
    /// The type of this GUI element.
    pub typ: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
    /// How much this progress bar is filled. It is a value in the range [0, 1].
    pub value: f64,
    /// Whether the content of this table should be vertically centered. Overrides [LuaStyle::column_alignments](LuaStyle::column_alignments). Defaults to `true`.
    pub vertical_centering: bool,
    /// Policy of the vertical scroll bar. Possible values are `"auto"`, `"never"`, `"always"`, `"auto-and-reserve-space"`, `"dont-show-but-allow-scrolling"`.
    pub vertical_scroll_policy: String,
    /// Sets whether this GUI element is visible or completely hidden, taking no space in the layout.
    pub visible: bool,
    /// Whether this text-box will word-wrap automatically. Defaults to `false`.
    pub word_wrap: bool,
    /// The zoom this camera or minimap is using. This value must be positive.
    pub zoom: f64,
}

/// An element of a custom GUI. This type is used to represent any kind of a GUI element - labels, buttons and frames are all instances of this type. Just like [LuaEntity](LuaEntity), different kinds of elements support different attributes; attempting to access an attribute on an element that doesn't support it (for instance, trying to access the `column_count` of a `textfield`) will result in a runtime error.
///
/// The following types of GUI element are supported:
///
/// - `"button"`: A clickable element. Relevant event: [on_gui_click](on_gui_click)
/// - `"sprite-button"`: A `button` that displays a sprite rather than text. Relevant event: [on_gui_click](on_gui_click)
/// - `"checkbox"`: A clickable element with a check mark that can be turned off or on. Relevant event: [on_gui_checked_state_changed](on_gui_checked_state_changed)
/// - `"flow"`: An invisible container that lays out its children either horizontally or vertically.
/// - `"frame"`: A non-transparent box that contains other elements. It can have a title (set via the `caption` attribute). Just like a `flow`, it lays out its children either horizontally or vertically. Relevant event: [on_gui_location_changed](on_gui_location_changed)
/// - `"label"`: A piece of text.
/// - `"line"`: A horizontal or vertical separation line.
/// - `"progressbar"`: A partially filled bar that can be used to indicate progress.
/// - `"table"`: An invisible container that lays out its children in a specific number of columns. The width of each column is determined by the widest element it contains.
/// - `"textfield"`: A single-line box the user can type into. Relevant events: [on_gui_text_changed](on_gui_text_changed), [on_gui_confirmed](on_gui_confirmed)
/// - `"radiobutton"`: An element that is similar to a `checkbox`, but with a circular appearance. Clicking a selected radio button will not deselect it. Radio buttons are not linked to each other in any way. Relevant event: [on_gui_checked_state_changed](on_gui_checked_state_changed)
/// - `"sprite"`: An element that shows an image.
/// - `"scroll-pane"`: An invisible element that is similar to a `flow`, but has the ability to show and use scroll bars.
/// - `"drop-down"`: A drop-down containing strings of text. Relevant event: [on_gui_selection_state_changed](on_gui_selection_state_changed)
/// - `"list-box"`: A list of strings, only one of which can be selected at a time. Shows a scroll bar if necessary. Relevant event: [on_gui_selection_state_changed](on_gui_selection_state_changed)
/// - `"camera"`: A camera that shows the game at the given position on the given surface. It can visually track an [entity](LuaGuiElement::entity) that is set after the element has been created.
/// - `"choose-elem-button"`: A button that lets the player pick from a certain kind of prototype, with optional filtering. Relevant event: [on_gui_elem_changed](on_gui_elem_changed)
/// - `"text-box"`: A multi-line `textfield`. Relevant event: [on_gui_text_changed](on_gui_text_changed)
/// - `"slider"`: A horizontal number line which can be used to choose a number. Relevant event: [on_gui_value_changed](on_gui_value_changed)
/// - `"minimap"`: A minimap preview, similar to the normal player minimap. It can visually track an [entity](LuaGuiElement::entity) that is set after the element has been created.
/// - `"entity-preview"`: A preview of an entity. The [entity](LuaGuiElement::entity) has to be set after the element has been created.
/// - `"empty-widget"`: An empty element that just exists. The root GUI elements `screen` and `relative` are `empty-widget`s.
/// - `"tabbed-pane"`: A collection of `tab`s and their contents. Relevant event: [on_gui_selected_tab_changed](on_gui_selected_tab_changed)
/// - `"tab"`: A tab for use in a `tabbed-pane`.
/// - `"switch"`: A switch with three possible states. Can have labels attached to either side. Relevant event: [on_gui_switch_state_changed](on_gui_switch_state_changed)
///
/// Each GUI element allows access to its children by having them as attributes. Thus, one can use the `parent.child` syntax to refer to children. Lua also supports the `parent["child"]` syntax to refer to the same element. This can be used in cases where the child has a name that isn't a valid Lua identifier.
pub trait LuaGuiElementMethods {
    /// Add a new child element to this GuiElement.
    fn add(anchor: GuiAnchor, caption: LocalisedString, enabled: bool, ignored_by_interaction: bool, index: u32, name: String, style: String, tags: Tags, tooltip: LocalisedString, typ: String, visible: bool) -> LuaGuiElement;
    /// Inserts a string at the end or at the given index of this dropdown or listbox.
    fn add_item(index: u32, string: LocalisedString);
    /// Adds the given tab and content widgets to this tabbed pane as a new tab.
    fn add_tab(content: LuaGuiElement, tab: LuaGuiElement);
    /// Moves this GUI element to the "front" so it will draw over other elements.
    fn bring_to_front();
    /// Remove children of this element. Any [LuaGuiElement](LuaGuiElement) objects referring to the destroyed elements become invalid after this operation.
    fn clear();
    /// Removes the items in this dropdown or listbox.
    fn clear_items();
    /// Closes the dropdown list if this is a dropdown and it is open.
    fn close_dropdown();
    /// Remove this element, along with its children. Any [LuaGuiElement](LuaGuiElement) objects referring to the destroyed elements become invalid after this operation.
    fn destroy();
    /// Focuses this GUI element if possible.
    fn focus();
    /// Forces this frame to re-auto-center. Only works on frames stored directly in [LuaGui::screen](LuaGui::screen).
    fn force_auto_center();
    /// Gets the index that this element has in its parent element.
    fn get_index_in_parent() -> u32;
    /// Gets the item at the given index from this dropdown or listbox.
    fn get_item(index: u32) -> LocalisedString;
    /// The mod that owns this Gui element or `nil` if it's owned by the scenario script.
    fn get_mod() -> Option<String>;
    /// Returns whether this slider only allows being moved to discrete positions.
    fn get_slider_discrete_slider() -> bool;
    /// Returns whether this slider only allows discrete values.
    fn get_slider_discrete_values() -> bool;
    /// Gets this sliders maximum value.
    fn get_slider_maximum() -> f64;
    /// Gets this sliders minimum value.
    fn get_slider_minimum() -> f64;
    /// Gets the minimum distance this slider can move.
    fn get_slider_value_step() -> f64;
    /// All methods and properties that this object supports.
    fn help() -> String;
    /// Removes the item at the given index from this dropdown or listbox.
    fn remove_item(index: u32);
    /// Removes the given tab and its associated content from this tabbed pane.
    fn remove_tab(tab: LuaGuiElement);
    /// Scrolls this scroll bar to the bottom.
    fn scroll_to_bottom();
    /// Scrolls this scroll bar such that the specified GUI element is visible to the player.
    fn scroll_to_element(element: LuaGuiElement, scroll_mode: String);
    /// Scrolls the scroll bar such that the specified listbox item is visible to the player.
    fn scroll_to_item(index: i32, scroll_mode: String);
    /// Scrolls this scroll bar to the left.
    fn scroll_to_left();
    /// Scrolls this scroll bar to the right.
    fn scroll_to_right();
    /// Scrolls this scroll bar to the top.
    fn scroll_to_top();
    /// Selects a range of text in this textbox.
    fn select(end: i32, start: i32);
    /// Selects all the text in this textbox.
    fn select_all();
    /// Sets the given string at the given index in this dropdown or listbox.
    fn set_item(index: u32, string: LocalisedString);
    /// Sets whether this slider only allows being moved to discrete positions.
    fn set_slider_discrete_slider(value: bool);
    /// Sets whether this slider only allows discrete values.
    fn set_slider_discrete_values(value: bool);
    /// Sets this sliders minimum and maximum values.
    fn set_slider_minimum_maximum(maximum: f64, minimum: f64);
    /// Sets the minimum distance this slider can move.
    fn set_slider_value_step(value: f64);
    /// Swaps the children at the given indices in this element.
    fn swap_children(index_1: u32, index_2: u32);
}

/// Prototype of a heat buffer.
pub struct LuaHeatBufferPrototype {
    pub connections: Vec<HeatConnection>,
    pub default_temperature: f64,
    pub max_temperature: f64,
    pub max_transfer: f64,
    pub min_temperature_gradient: f64,
    pub min_working_temperature: f64,
    pub minimum_glow_temperature: f64,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    pub specific_heat: f64,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of a heat buffer.
pub trait LuaHeatBufferPrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Prototype of a heat energy source.
pub struct LuaHeatEnergySourcePrototype {
    pub connections: Vec<HeatConnection>,
    pub default_temperature: f64,
    /// The emissions of this energy source in `pollution/Joule`. Multiplying it by energy consumption in `Watt` gives `pollution/second`.
    pub emissions: f64,
    pub heat_buffer_prototype: LuaHeatBufferPrototype,
    pub max_temperature: f64,
    pub max_transfer: f64,
    pub min_temperature_gradient: f64,
    pub min_working_temperature: f64,
    pub minimum_glow_temperature: f64,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    pub render_no_network_icon: bool,
    pub render_no_power_icon: bool,
    pub specific_heat: f64,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of a heat energy source.
pub trait LuaHeatEnergySourcePrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Control behavior for inserters.
pub struct LuaInserterControlBehavior {
    pub lua_generic_on_off_control_behavior: Box<LuaGenericOnOffControlBehavior>,
    /// The hand read mode for the inserter.
    pub circuit_hand_read_mode: ControlBehaviorInserterHandReadMode,
    /// The circuit mode of operations for the inserter.
    pub circuit_mode_of_operation: ControlBehaviorInserterCircuitModeOfOperation,
    /// `true` if the contents of the inserter hand should be sent to the circuit network
    pub circuit_read_hand_contents: bool,
    /// If the stack size of the inserter is set through the circuit network or not.
    pub circuit_set_stack_size: bool,
    /// The signal used to set the stack size of the inserter.
    pub circuit_stack_control_signal: SignalID,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Control behavior for inserters.
pub trait LuaInserterControlBehaviorMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// A storage of item stacks.
pub struct LuaInventory {
    /// The entity that owns this inventory, if any.
    pub entity_owner: Option<Box<LuaEntity>>,
    /// The equipment that owns this inventory, if any.
    pub equipment_owner: Option<Box<LuaEquipment>>,
    /// The inventory index this inventory uses, if any.
    pub index: Option<Inventory>,
    /// The mod that owns this inventory, if any.
    pub mod_owner: Option<String>,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The player that owns this inventory, if any.
    pub player_owner: Option<LuaPlayer>,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

pub enum LuaInventoryMethodsSetFilterFilterUnion {
    String(String),
    Nil,
}

/// A storage of item stacks.
pub trait LuaInventoryMethods {
    /// Can at least some items be inserted?
    fn can_insert(items: ItemStackIdentification) -> bool;
    /// If the given inventory slot filter can be set to the given filter.
    fn can_set_filter(filter: String, index: u32) -> bool;
    /// Make this inventory empty.
    fn clear();
    /// Counts the number of empty stacks.
    fn count_empty_stacks(include_bar: bool, include_filtered: bool) -> u32;
    /// Destroys this inventory.
    fn destroy();
    /// Finds the first empty stack. Filtered slots are excluded unless a filter item is given.
    fn find_empty_stack(item: String) -> (Option<LuaItemStack>, Option<u32>);
    /// Finds the first LuaItemStack in the inventory that matches the given item name.
    fn find_item_stack(item: String) -> (Option<LuaItemStack>, Option<u32>);
    /// Get the current bar. This is the index at which the red area starts.
    fn get_bar() -> u32;
    /// Get counts of all items in this inventory.
    fn get_contents() -> HashMap<String, u32>;
    /// Gets the filter for the given item stack index.
    fn get_filter(index: u32) -> Option<String>;
    /// Gets the number of the given item that can be inserted into this inventory.
    fn get_insertable_count(item: String);
    /// Get the number of all or some items in this inventory.
    fn get_item_count(item: String) -> u32;
    /// All methods and properties that this object supports.
    fn help() -> String;
    /// Insert items into this inventory.
    fn insert(items: ItemStackIdentification) -> u32;
    /// Does this inventory contain nothing?
    fn is_empty() -> bool;
    /// If this inventory supports filters and has at least 1 filter set.
    fn is_filtered() -> bool;
    /// Is every stack in this inventory full? Ignores stacks blocked by the current bar.
    fn is_full() -> bool;
    /// Remove items from this inventory.
    fn remove(items: ItemStackIdentification) -> u32;
    /// Resizes the inventory.
    fn resize(size: u16);
    /// Set the current bar.
    fn set_bar(bar: u32);
    /// Sets the filter for the given item stack index.
    fn set_filter(filter: LuaInventoryMethodsSetFilterFilterUnion, index: u32) -> bool;
    /// Sorts and merges the items in this inventory.
    fn sort_and_merge();
    /// Does this inventory support a bar? Bar is the draggable red thing, found for example on chests, that limits the portion of the inventory that may be manipulated by machines.
    fn supports_bar() -> bool;
    /// If this inventory supports filters.
    fn supports_filters() -> bool;
}

/// Prototype of an item.
pub struct LuaItemPrototype {
    /// The alt entity filter mode used by this selection tool.
    pub alt_entity_filter_mode: Option<String>,
    /// The alt entity filters used by this selection tool indexed by entity name.
    pub alt_entity_filters: Option<HashMap<String, LuaEntityPrototype>>,
    /// The alt entity type filters used by this selection tool indexed by entity type.
    pub alt_entity_type_filters: Option<HashMap<String, bool>>,
    /// The alt reverse entity filter mode used by this selection tool.
    pub alt_reverse_alt_entity_filter_mode: Option<String>,
    /// The alt reverse entity filters used by this selection tool indexed by entity name.
    pub alt_reverse_entity_filters: Option<HashMap<String, LuaEntityPrototype>>,
    /// The alt reverse entity type filters used by this selection tool indexed by entity type.
    pub alt_reverse_entity_type_filters: Option<HashMap<String, bool>>,
    /// The color used when doing alt reverse selection with this selection tool prototype.
    pub alt_reverse_selection_border_color: Option<Color>,
    pub alt_reverse_selection_cursor_box_type: Option<String>,
    /// Flags that affect which entities will be selected during alt reverse selection.
    pub alt_reverse_selection_mode_flags: Option<SelectionModeFlags>,
    /// The alt reverse tile filter mode used by this selection tool.
    pub alt_reverse_tile_filter_mode: Option<String>,
    /// The alt reverse tile filters used by this selection tool indexed by tile name.
    pub alt_reverse_tile_filters: Option<HashMap<String, LuaTilePrototype>>,
    /// The color used when doing alt selection with this selection tool prototype.
    pub alt_selection_border_color: Option<Color>,
    pub alt_selection_cursor_box_type: Option<String>,
    /// Flags that affect which entities will be selected during alternate selection.
    pub alt_selection_mode_flags: Option<SelectionModeFlags>,
    /// The alt tile filter mode used by this selection tool.
    pub alt_tile_filter_mode: Option<String>,
    /// The alt tile filters used by this selection tool indexed by tile name.
    pub alt_tile_filters: Option<HashMap<String, LuaTilePrototype>>,
    /// If tiles area always included when doing selection with this selection tool prototype.
    pub always_include_tiles: Option<bool>,
    /// The gun attack parameters.
    pub attack_parameters: Option<AttackParameters>,
    /// The result of burning this item as fuel, if any.
    pub burnt_result: Option<Box<LuaItemPrototype>>,
    /// If this item can be mod-opened.
    pub can_be_mod_opened: bool,
    /// The capsule action for this capsule item prototype.
    pub capsule_action: Option<CapsuleAction>,
    /// The name of a [LuaModuleCategoryPrototype](LuaModuleCategoryPrototype). Used when upgrading modules: Ctrl + click modules into an entity and it will replace lower tier modules of the same category with higher tier modules.
    pub category: Option<String>,
    /// The curved rail prototype used for this rail planner prototype.
    pub curved_rail: Option<LuaEntityPrototype>,
    /// The default label color used for this item with label, if any.
    pub default_label_color: Option<Color>,
    /// The default request value.
    pub default_request_amount: u32,
    /// If true, and this item with label has a label it is drawn in place of the normal number when held in the cursor.
    pub draw_label_for_cursor_render: Option<bool>,
    /// The durability of this tool item.
    pub durability: Option<f64>,
    /// The durability message key used when displaying the durability of this tool.
    pub durability_description_key: Option<String>,
    /// The entity filter mode used by this selection tool.
    pub entity_filter_mode: Option<String>,
    /// The number of entity filters this deconstruction item has.
    pub entity_filter_slots: Option<u32>,
    /// The entity filters used by this selection tool indexed by entity name.
    pub entity_filters: Option<HashMap<String, LuaEntityPrototype>>,
    /// The entity type filters used by this selection tool indexed by entity type.
    pub entity_type_filters: Option<HashMap<String, bool>>,
    /// The prototype of this armor equipment grid, if any.
    pub equipment_grid: Option<LuaEquipmentGridPrototype>,
    /// If this item with inventory extends the inventory it resides in by default.
    pub extend_inventory_by_default: Option<bool>,
    /// The filter mode used by this item with inventory.
    pub filter_mode: Option<String>,
    /// The flags for this item prototype.
    pub flags: ItemPrototypeFlags,
    /// The acceleration multiplier when this item is used as fuel in a vehicle.
    pub fuel_acceleration_multiplier: f64,
    /// The fuel category of this item prototype, if any.
    pub fuel_category: Option<String>,
    /// The emissions multiplier if this is used as fuel.
    pub fuel_emissions_multiplier: f64,
    /// The fuel top speed multiplier when this item is used as fuel in a vehicle.
    pub fuel_top_speed_multiplier: f64,
    /// Fuel value when burned.
    pub fuel_value: f32,
    /// The group this prototype belongs to.
    pub group: LuaGroup,
    /// If this tool item has infinite durability.
    pub infinite: Option<bool>,
    /// The insertion priority mode used by this item with inventory.
    pub insertion_priority_mode: Option<String>,
    /// The main inventory size for item-with-inventory-prototype.
    pub inventory_size: Option<u32>,
    /// The inventory size bonus for this armor prototype.
    pub inventory_size_bonus: Option<u32>,
    pub item_filters: Option<HashMap<String, LuaItemPrototype>>,
    pub item_group_filters: Option<HashMap<String, LuaGroup>>,
    pub item_subgroup_filters: Option<HashMap<String, LuaGroup>>,
    /// The limitation message key used when the player attempts to use this modules in some place it's not allowed.
    pub limitation_message_key: Option<String>,
    /// An array of recipe names this module is allowed to work with. Empty when all recipes are allowed.
    pub limitations: Option<Vec<String>>,
    pub localised_description: LocalisedString,
    /// The localised string used when the player attempts to put items into this item with inventory that aren't allowed.
    pub localised_filter_message: Option<LocalisedString>,
    pub localised_name: LocalisedString,
    /// Size of full magazine.
    pub magazine_size: Option<f32>,
    /// How many filters an upgrade item has.
    pub mapper_count: Option<u32>,
    /// Effects of this module.
    pub module_effects: Option<ModuleEffects>,
    /// Name of this prototype.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    /// Prototype of the equipment that will be created by placing this item in an equipment grid, if any.
    pub place_as_equipment_result: Option<LuaEquipmentPrototype>,
    /// The place-as-tile result if one is defined, if any.
    pub place_as_tile_result: Option<PlaceAsTileResult>,
    /// Prototype of the entity that will be created by placing this item, if any.
    pub place_result: Option<LuaEntityPrototype>,
    /// Amount of extra time (in ticks) it takes to reload the weapon after depleting the magazine.
    pub reload_time: Option<f32>,
    /// The repair result of this repair tool prototype.
    pub repair_result: Option<Vec<TriggerItem>>,
    /// Resistances of this armor item, if any, indexed by damage type name.
    pub resistances: Option<HashMap<String, Resistance>>,
    /// The reverse entity filter mode used by this selection tool.
    pub reverse_alt_entity_filter_mode: Option<String>,
    /// The reverse entity filters used by this selection tool indexed by entity name.
    pub reverse_entity_filters: Option<HashMap<String, LuaEntityPrototype>>,
    /// The reverse entity type filters used by this selection tool indexed by entity type.
    pub reverse_entity_type_filters: Option<HashMap<String, bool>>,
    /// The color used when doing reverse selection with this selection tool prototype.
    pub reverse_selection_border_color: Option<Color>,
    pub reverse_selection_cursor_box_type: Option<String>,
    /// Flags that affect which entities will be selected during reverse selection.
    pub reverse_selection_mode_flags: Option<SelectionModeFlags>,
    /// The reverse tile filter mode used by this selection tool.
    pub reverse_tile_filter_mode: Option<String>,
    /// The reverse tile filters used by this selection tool indexed by tile name.
    pub reverse_tile_filters: Option<HashMap<String, LuaTilePrototype>>,
    /// The results of launching this item in a rocket.
    pub rocket_launch_products: Vec<Product>,
    /// The color used when doing normal selection with this selection tool prototype.
    pub selection_border_color: Option<Color>,
    pub selection_cursor_box_type: Option<String>,
    /// Flags that affect which entities will be selected.
    pub selection_mode_flags: Option<SelectionModeFlags>,
    /// The repairing speed if this is a repairing tool.
    pub speed: Option<f32>,
    /// Maximum stack size of the item specified by this prototype.
    pub stack_size: u32,
    /// Is this item allowed to stack at all?
    pub stackable: bool,
    /// The straight rail prototype used for this rail planner prototype.
    pub straight_rail: Option<LuaEntityPrototype>,
    /// The subgroup this prototype belongs to.
    pub subgroup: LuaGroup,
    /// Tier of the module inside its category. Used when upgrading modules: Ctrl + click modules into an entity and it will replace lower tier modules with higher tier modules if they have the same category.
    pub tier: Option<u32>,
    /// The tile filter mode used by this selection tool.
    pub tile_filter_mode: Option<String>,
    /// The number of tile filters this deconstruction item has.
    pub tile_filter_slots: Option<u32>,
    /// The tile filters used by this selection tool indexed by tile name.
    pub tile_filters: Option<HashMap<String, LuaTilePrototype>>,
    /// Type of this prototype. E.g. `"gun"` or `"mining-tool"`.
    pub typ: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
    /// The number of items needed to connect two entities with this as wire.
    pub wire_count: u32,
}

/// Prototype of an item.
pub trait LuaItemPrototypeMethods {
    /// The type of this ammo prototype.
    fn get_ammo_type(ammo_source_type: String) -> Option<AmmoType>;
    /// Does this prototype have a flag enabled?
    fn has_flag(flag: String) -> bool;
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// A reference to an item and count owned by some external entity.
pub struct LuaItemStack {
    /// The active blueprint index for this blueprint book. `nil` if this blueprint book is empty.
    pub active_index: Option<u32>,
    /// Whether the label for this item can be manually changed. When false the label can only be changed through the API.
    pub allow_manual_label_change: bool,
    /// Number of bullets left in the magazine.
    pub ammo: u32,
    /// If absolute snapping is enabled on this blueprint item.
    pub blueprint_absolute_snapping: bool,
    /// Icons of this blueprint item, blueprint book, deconstruction item or upgrade planner. An item that doesn't have icons returns `nil` on read and throws error on write.
    pub blueprint_icons: Option<Vec<BlueprintSignalIcon>>,
    /// The offset from the absolute grid. `nil` if absolute snapping is not enabled.
    pub blueprint_position_relative_to_grid: Option<TilePosition>,
    /// The snapping grid size in this blueprint item. `nil` if snapping is not enabled.
    pub blueprint_snap_to_grid: Option<TilePosition>,
    /// If this item is a spidertron remote that has a spidertron bound to it, it returns the connected spider-vehicle entity.
    pub connected_entity: Option<Box<LuaEntity>>,
    /// Raw materials required to build this blueprint. Result is a dictionary mapping each item prototype name to the required count.
    pub cost_to_build: HashMap<String, u32>,
    /// Number of items in this stack.
    pub count: u32,
    /// The custom description this item-with-tags. This is shown over the normal item description if this is set to a non-empty value.
    pub custom_description: LocalisedString,
    /// The default icons for a blueprint item.
    pub default_icons: Vec<BlueprintSignalIcon>,
    /// Durability of the contained item. Automatically capped at the item's maximum durability.
    pub durability: Option<f64>,
    /// If this is an item with entity data, get the stored entity color.
    pub entity_color: Option<Color>,
    /// The number of entity filters this deconstruction item supports.
    pub entity_filter_count: u32,
    /// The blacklist/whitelist entity filter mode for this deconstruction item.
    pub entity_filter_mode: DeconstructionItemEntityFilterMode,
    /// The entity filters for this deconstruction item. The attribute is a sparse array with the keys representing the index of the filter. All strings in this array must be entity prototype names that don't have the `"not-deconstructable"` flag set and are either a `cliff` or marked as `minable`.
    pub entity_filters: Vec<String>,
    /// If this is an item with entity data, get the stored entity label.
    pub entity_label: Option<String>,
    /// If this item extends the inventory it resides in (provides its contents for counts, crafting, insertion). Only callable on items with inventories.
    pub extends_inventory: bool,
    /// The equipment grid of this item, if any.
    pub grid: Option<LuaEquipmentGrid>,
    /// How much health the item has, as a number in range [0, 1].
    pub health: f32,
    /// If this is an armor item.
    pub is_armor: bool,
    /// If this is a blueprint item.
    pub is_blueprint: bool,
    /// If this is a blueprint book item.
    pub is_blueprint_book: bool,
    /// If this is a deconstruction tool item.
    pub is_deconstruction_item: bool,
    /// If this is an item with entity data item.
    pub is_item_with_entity_data: bool,
    /// If this is an item with inventory item.
    pub is_item_with_inventory: bool,
    /// If this is an item with label item.
    pub is_item_with_label: bool,
    /// If this is an item with tags item.
    pub is_item_with_tags: bool,
    /// If this is a mining tool item.
    pub is_mining_tool: bool,
    /// If this is a module item.
    pub is_module: bool,
    /// If this is a repair tool item.
    pub is_repair_tool: bool,
    /// If this is a selection tool item.
    pub is_selection_tool: bool,
    /// If this is a tool item.
    pub is_tool: bool,
    /// If this is a upgrade item.
    pub is_upgrade_item: bool,
    /// The unique identifier for this item , if any. Note that this ID stays the same no matter where the item is moved to.
    ///
    /// Only these types of items have unique IDs:
    /// - `"armor"`
    /// - `"spidertron-remote"`
    /// - `"selection-tool"`
    /// - `"copy-paste-tool"`
    /// - `"upgrade-item"`
    /// - `"deconstruction-item"`
    /// - `"blueprint"`
    /// - `"blueprint-book"`
    /// - `"item-with-entity-data"`
    /// - `"item-with-inventory"`
    /// - `"item-with-tags"`
    pub item_number: Option<u32>,
    /// The current label for this item, if any.
    pub label: Option<String>,
    /// The current label color for this item, if any.
    pub label_color: Option<Color>,
    /// Prototype name of the item held in this stack.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The insertion mode priority this ItemWithInventory uses when items are inserted into an inventory it resides in. Only callable on items with inventories.
    pub prioritize_insertion_mode: String,
    /// Prototype of the item held in this stack.
    pub prototype: LuaItemPrototype,
    pub tags: Tags,
    /// The number of tile filters this deconstruction item supports.
    pub tile_filter_count: u32,
    /// The blacklist/whitelist tile filter mode for this deconstruction item.
    pub tile_filter_mode: DeconstructionItemTileFilterMode,
    /// The tile filters for this deconstruction item. The attribute is a sparse array with the keys representing the index of the filter. All strings in this array must be tile prototype names.
    pub tile_filters: Vec<String>,
    /// The tile selection mode for this deconstruction item.
    pub tile_selection_mode: DeconstructionItemTileSelectionMode,
    /// If this deconstruction item is set to allow trees and rocks only.
    pub trees_and_rocks_only: bool,
    /// Type of the item prototype.
    pub typ: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
    /// Is this valid for reading? Differs from the usual `valid` in that `valid` will be `true` even if the item stack is blank but the entity that holds it is still valid.
    pub valid_for_read: bool,
}

pub enum LuaItemStackMethodsSetEntityFilterFilterUnion {
    String(String),
    LuaEntityPrototype(LuaEntityPrototype),
    LuaEntity(LuaEntity),
    Nil,
}

pub enum LuaItemStackMethodsSetTileFilterFilterUnion {
    String(String),
    LuaTilePrototype(LuaTilePrototype),
    LuaTile(LuaTile),
}

/// A reference to an item and count owned by some external entity.
pub trait LuaItemStackMethods {
    /// Add ammo to this ammo item.
    fn add_ammo(amount: f32);
    /// Add durability to this tool item.
    fn add_durability(amount: f64);
    fn build_blueprint(by_player: PlayerIdentification, direction: Direction, force: ForceIdentification, force_build: bool, position: MapPosition, raise_built: bool, skip_fog_of_war: bool, surface: SurfaceIdentification) -> Vec<LuaEntity>;
    /// Would a call to [LuaItemStack::set_stack](LuaItemStack::set_stack) succeed?
    fn can_set_stack(stack: ItemStackIdentification) -> bool;
    /// Cancel deconstruct the given area with this deconstruction item.
    fn cancel_deconstruct_area(area: BoundingBox, by_player: PlayerIdentification, force: ForceIdentification, skip_fog_of_war: bool, surface: SurfaceIdentification);
    /// Clear this item stack.
    fn clear();
    /// Clears this blueprint item.
    fn clear_blueprint();
    /// Clears all settings/filters on this deconstruction item resetting it to default values.
    fn clear_deconstruction_item();
    /// Clears all settings/filters on this upgrade item resetting it to default values.
    fn clear_upgrade_item();
    /// Sets up this blueprint using the found blueprintable entities/tiles on the surface.
    fn create_blueprint(always_include_tiles: bool, area: BoundingBox, force: ForceIdentification, include_entities: bool, include_fuel: bool, include_modules: bool, include_station_names: bool, include_trains: bool, surface: SurfaceIdentification) -> HashMap<u32, LuaEntity>;
    /// Creates the equipment grid for this item if it doesn't exist and this is an item-with-entity-data that supports equipment grids.
    fn create_grid() -> LuaEquipmentGrid;
    /// Deconstruct the given area with this deconstruction item.
    fn deconstruct_area(area: BoundingBox, by_player: PlayerIdentification, force: ForceIdentification, skip_fog_of_war: bool, surface: SurfaceIdentification);
    /// Remove ammo from this ammo item.
    fn drain_ammo(amount: f32);
    /// Remove durability from this tool item.
    fn drain_durability(amount: f64);
    /// Export a supported item (blueprint, blueprint-book, deconstruction-planner, upgrade-planner, item-with-tags) to a string.
    fn export_stack() -> String;
    /// The entities in this blueprint.
    fn get_blueprint_entities() -> Option<Vec<BlueprintEntity>>;
    /// Gets the number of entities in this blueprint item.
    fn get_blueprint_entity_count() -> u32;
    /// Gets the given tag on the given blueprint entity index in this blueprint item.
    fn get_blueprint_entity_tag(index: u32, tag: String) -> Option<AnyBasic>;
    /// Gets the tags for the given blueprint entity index in this blueprint item.
    fn get_blueprint_entity_tags(index: u32) -> Tags;
    /// A list of the tiles in this blueprint.
    fn get_blueprint_tiles() -> Option<Vec<Tile>>;
    /// Gets the entity filter at the given index for this deconstruction item.
    fn get_entity_filter(index: u32) -> Option<String>;
    /// Access the inner inventory of an item.
    fn get_inventory(inventory: Inventory) -> Option<LuaInventory>;
    /// Gets the filter at the given index for this upgrade item.
    fn get_mapper(index: u32, typ: String) -> UpgradeFilter;
    /// Gets the tag with the given name or returns `nil` if it doesn't exist.
    fn get_tag(tag_name: String) -> Option<AnyBasic>;
    /// Gets the tile filter at the given index for this deconstruction item.
    fn get_tile_filter(index: u32) -> Option<String>;
    /// All methods and properties that this object supports.
    fn help() -> String;
    /// Import a supported item (blueprint, blueprint-book, deconstruction-planner, upgrade-planner, item-with-tags) from a string.
    fn import_stack(data: String) -> i32;
    /// Is this blueprint item setup? I.e. is it a non-empty blueprint?
    fn is_blueprint_setup() -> bool;
    /// Removes a tag with the given name.
    fn remove_tag(tag: String) -> bool;
    /// Set new entities to be a part of this blueprint.
    fn set_blueprint_entities(entities: Vec<BlueprintEntity>);
    /// Sets the given tag on the given blueprint entity index in this blueprint item.
    fn set_blueprint_entity_tag(index: u32, tag: String, value: AnyBasic);
    /// Sets the tags on the given blueprint entity index in this blueprint item.
    fn set_blueprint_entity_tags(index: u32, tags: Tags);
    /// Set specific tiles in this blueprint.
    fn set_blueprint_tiles(tiles: Vec<Tile>);
    /// Sets the entity filter at the given index for this deconstruction item.
    fn set_entity_filter(filter: LuaItemStackMethodsSetEntityFilterFilterUnion, index: u32) -> bool;
    /// Sets the module filter at the given index for this upgrade item.
    fn set_mapper(filter: UpgradeFilter, index: u32, typ: String);
    /// Set this item stack to another item stack.
    fn set_stack(stack: ItemStackIdentification) -> bool;
    /// Sets the tag with the given name and value.
    fn set_tag(tag: AnyBasic, tag_name: String);
    /// Sets the tile filter at the given index for this deconstruction item.
    fn set_tile_filter(filter: LuaItemStackMethodsSetTileFilterFilterUnion, index: u32) -> bool;
    /// Swaps this item stack with the given item stack if allowed.
    fn swap_stack(stack: LuaItemStack) -> bool;
    /// Transfers the given item stack into this item stack.
    fn transfer_stack(stack: ItemStackIdentification) -> bool;
    /// Use the capsule item with the entity as the source, targeting the given position.
    fn use_capsule(entity: LuaEntity, target_position: MapPosition) -> Vec<LuaEntity>;
}

/// Control behavior for lamps.
pub struct LuaLampControlBehavior {
    pub lua_generic_on_off_control_behavior: Box<LuaGenericOnOffControlBehavior>,
    /// The color the lamp is showing, if any.
    pub color: Option<Color>,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// `true` if the lamp should set the color from the circuit network signals.
    pub use_colors: bool,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Control behavior for lamps.
pub trait LuaLampControlBehaviorMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// A lazily loaded value. For performance reasons, we sometimes return a custom lazily-loaded value type instead of the native Lua value. This custom type lazily constructs the necessary value when [LuaLazyLoadedValue::get](LuaLazyLoadedValue::get) is called, therefore preventing its unnecessary construction in some cases.
///
/// An instance of LuaLazyLoadedValue is only valid during the event it was created from and cannot be saved.
pub struct LuaLazyLoadedValue {
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// A lazily loaded value. For performance reasons, we sometimes return a custom lazily-loaded value type instead of the native Lua value. This custom type lazily constructs the necessary value when [LuaLazyLoadedValue::get](LuaLazyLoadedValue::get) is called, therefore preventing its unnecessary construction in some cases.
///
/// An instance of LuaLazyLoadedValue is only valid during the event it was created from and cannot be saved.
pub trait LuaLazyLoadedValueMethods {
    /// Gets the value of this lazy loaded value.
    fn get() -> Any;
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Logistic cell of a particular [LuaEntity](LuaEntity). A "Logistic Cell" is the given name for settings and properties used by what would normally be seen as a "Roboport". A logistic cell however doesn't have to be attached to the roboport entity (the character has one for the personal roboport).
pub struct LuaLogisticCell {
    /// Radius at which the robots hover when waiting to be charged.
    pub charge_approach_distance: f32,
    /// Number of robots currently charging.
    pub charging_robot_count: u32,
    /// Robots currently being charged.
    pub charging_robots: Vec<LuaEntity>,
    /// Construction radius of this cell.
    pub construction_radius: f32,
    /// The network that owns this cell, if any.
    pub logistic_network: Option<LuaLogisticNetwork>,
    /// Logistic radius of this cell.
    pub logistic_radius: f32,
    /// Logistic connection distance of this cell.
    pub logistics_connection_distance: f32,
    /// `true` if this is a mobile cell. In vanilla, only the logistic cell created by a character's personal roboport is mobile.
    pub mobile: bool,
    /// Neighbouring cells.
    pub neighbours: Vec<LuaLogisticCell>,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// This cell's owner.
    pub owner: Box<LuaEntity>,
    /// Number of stationed construction robots in this cell.
    pub stationed_construction_robot_count: u32,
    /// Number of stationed logistic robots in this cell.
    pub stationed_logistic_robot_count: u32,
    /// Number of robots waiting to charge.
    pub to_charge_robot_count: u32,
    /// Robots waiting to charge.
    pub to_charge_robots: Vec<LuaEntity>,
    /// `true` if this cell is active.
    pub transmitting: bool,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Logistic cell of a particular [LuaEntity](LuaEntity). A "Logistic Cell" is the given name for settings and properties used by what would normally be seen as a "Roboport". A logistic cell however doesn't have to be attached to the roboport entity (the character has one for the personal roboport).
pub trait LuaLogisticCellMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
    /// Is a given position within the construction range of this cell?
    fn is_in_construction_range(position: MapPosition) -> bool;
    /// Is a given position within the logistic range of this cell?
    fn is_in_logistic_range(position: MapPosition) -> bool;
    /// Are two cells neighbours?
    fn is_neighbour_with(other: LuaLogisticCell) -> bool;
}

/// Control behavior for logistic chests.
pub struct LuaLogisticContainerControlBehavior {
    pub lua_control_behavior: Box<LuaControlBehavior>,
    /// The circuit mode of operations for the logistic container. Can only be set on containers whose [logistic_mode](LuaEntityPrototype::logistic_mode) is set to "requester".
    pub circuit_mode_of_operation: ControlBehaviorLogisticContainerCircuitModeOfOperation,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Control behavior for logistic chests.
pub trait LuaLogisticContainerControlBehaviorMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// A single logistic network of a given force on a given surface.
pub struct LuaLogisticNetwork {
    /// All active provider points in this network.
    pub active_provider_points: Vec<LuaLogisticPoint>,
    /// The total number of construction robots in the network (idle and active + in roboports).
    pub all_construction_robots: u32,
    /// The total number of logistic robots in the network (idle and active + in roboports).
    pub all_logistic_robots: u32,
    /// Number of construction robots available for a job.
    pub available_construction_robots: u32,
    /// Number of logistic robots available for a job.
    pub available_logistic_robots: u32,
    /// All cells in this network.
    pub cells: Vec<LuaLogisticCell>,
    /// All construction robots in this logistic network.
    pub construction_robots: Vec<LuaEntity>,
    /// All things that have empty provider points in this network.
    pub empty_provider_points: Vec<LuaLogisticPoint>,
    /// All entities that have empty logistic provider points in this network.
    pub empty_providers: Vec<LuaEntity>,
    /// The force this logistic network belongs to.
    pub force: LuaForce,
    /// All other entities that have logistic points in this network (inserters mostly).
    pub logistic_members: Vec<LuaEntity>,
    /// All logistic robots in this logistic network.
    pub logistic_robots: Vec<LuaEntity>,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// All passive provider points in this network.
    pub passive_provider_points: Vec<LuaLogisticPoint>,
    /// All things that have provider points in this network.
    pub provider_points: Vec<LuaLogisticPoint>,
    /// All entities that have logistic provider points in this network.
    pub providers: Vec<LuaEntity>,
    /// All things that have requester points in this network.
    pub requester_points: Vec<LuaLogisticPoint>,
    /// All entities that have logistic requester points in this network.
    pub requesters: Vec<LuaEntity>,
    /// Maximum number of robots the network can work with. Currently only used for the personal roboport.
    pub robot_limit: u32,
    /// All robots in this logistic network.
    pub robots: Vec<LuaEntity>,
    /// All things that have storage points in this network.
    pub storage_points: Vec<LuaLogisticPoint>,
    /// All entities that have logistic storage points in this network.
    pub storages: Vec<LuaEntity>,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// A single logistic network of a given force on a given surface.
pub trait LuaLogisticNetworkMethods {
    /// Can the network satisfy a request for a given item and count.
    fn can_satisfy_request(count: u32, include_buffers: bool, item: String) -> bool;
    /// Find logistic cell closest to a given position.
    fn find_cell_closest_to(position: MapPosition) -> Option<LuaLogisticCell>;
    /// Get item counts for the entire network, similar to how [LuaInventory::get_contents](LuaInventory::get_contents) does.
    fn get_contents() -> HashMap<String, u32>;
    /// Count given or all items in the network or given members.
    fn get_item_count(item: String, member: String) -> i32;
    /// Get the amount of items of the given type indexed by the storage member.
    fn get_supply_counts(item: String) -> HashMap<String, u32>;
    /// Gets the logistic points with of the given type indexed by the storage member.
    fn get_supply_points(item: String) -> HashMap<String, Vec<LuaLogisticPoint>>;
    /// All methods and properties that this object supports.
    fn help() -> String;
    /// Insert items into the logistic network. This will actually insert the items into some logistic chests.
    fn insert(item: ItemStackIdentification, members: String) -> u32;
    /// Remove items from the logistic network. This will actually remove the items from some logistic chests.
    fn remove_item(item: ItemStackIdentification, members: String) -> u32;
    /// Find a logistic point to drop the specific item stack.
    fn select_drop_point(members: String, stack: ItemStackIdentification) -> Option<LuaLogisticPoint>;
    /// Find the 'best' logistic point with this item ID and from the given position or from given chest type.
    fn select_pickup_point(include_buffers: bool, members: String, name: String, position: MapPosition) -> Option<LuaLogisticPoint>;
}

/// Logistic point of a particular [LuaEntity](LuaEntity). A "Logistic point" is the name given for settings and properties used by requester, provider, and storage points in a given logistic network. These "points" don't have to be a logistic container but often are. One other entity that can own several points is the "character" character type entity.
pub struct LuaLogisticPoint {
    /// If this logistic point is using the exact mode. In exact mode robots never over-deliver requests.
    pub exact: bool,
    /// The logistic filters for this logistic point, if this uses any.
    pub filters: Option<Vec<LogisticFilter>>,
    /// The force of this logistic point.
    pub force: LuaForce,
    /// The Logistic member index of this logistic point.
    pub logistic_member_index: u32,
    pub logistic_network: LuaLogisticNetwork,
    /// The logistic mode.
    pub mode: LogisticMode,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The [LuaEntity](LuaEntity) owner of this [LuaLogisticPoint](LuaLogisticPoint).
    pub owner: LuaEntity,
    /// Items targeted to be dropped off into this logistic point by robots. The attribute is a dictionary mapping the item prototype names to their item counts.
    pub targeted_items_deliver: HashMap<String, u32>,
    /// Items targeted to be picked up from this logistic point by robots. The attribute is a dictionary mapping the item prototype names to their item counts.
    pub targeted_items_pickup: HashMap<String, u32>,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Logistic point of a particular [LuaEntity](LuaEntity). A "Logistic point" is the name given for settings and properties used by requester, provider, and storage points in a given logistic network. These "points" don't have to be a logistic container but often are. One other entity that can own several points is the "character" character type entity.
pub trait LuaLogisticPointMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Control behavior for mining drills.
pub struct LuaMiningDrillControlBehavior {
    pub lua_generic_on_off_control_behavior: Box<LuaGenericOnOffControlBehavior>,
    /// `true` if this drill is enabled or disabled using the logistics or circuit condition.
    pub circuit_enable_disable: bool,
    /// `true` if this drill should send the resources in the field to the circuit network. Which resources depends on [LuaMiningDrillControlBehavior::resource_read_mode](LuaMiningDrillControlBehavior::resource_read_mode)
    pub circuit_read_resources: bool,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// If the mining drill should send just the resources in its area or the entire field it's on to the circuit network.
    pub resource_read_mode: ControlBehaviorMiningDrillResourceReadMode,
    /// The resource entities that the mining drill will send information about to the circuit network or an empty array.
    pub resource_read_targets: Vec<LuaEntity>,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Control behavior for mining drills.
pub trait LuaMiningDrillControlBehaviorMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

pub enum LuaModSettingPrototypeAllowedValuesUnion {
    ArrayString(Vec<String>),
    ArrayI32(Vec<i32>),
    ArrayF64(Vec<f64>),
}

pub enum LuaModSettingPrototypeDefaultValueUnion {
    Boolean(bool),
    Double(f64),
    Int(i32),
    String(String),
}

pub enum LuaModSettingPrototypeMaximumValueUnion {
    Double(f64),
    Int(i32),
}

pub enum LuaModSettingPrototypeMinimumValueUnion {
    Double(f64),
    Int(i32),
}

/// Prototype of a mod setting.
pub struct LuaModSettingPrototype {
    /// Whether this string setting allows blank values. `nil` if not a string setting.
    pub allow_blank: Option<bool>,
    /// The allowed values for this setting. `nil` if this setting doesn't use the a fixed set of values.
    pub allowed_values: Option<LuaModSettingPrototypeAllowedValuesUnion>,
    /// Whether this string setting auto-trims values. `nil` if not a string setting
    pub auto_trim: Option<bool>,
    /// The default value of this setting.
    pub default_value: LuaModSettingPrototypeDefaultValueUnion,
    /// Whether this setting is hidden from the GUI.
    pub hidden: bool,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    /// The maximum value for this setting. `nil` if this setting type doesn't support a maximum.
    pub maximum_value: Option<LuaModSettingPrototypeMaximumValueUnion>,
    /// The minimum value for this setting. `nil` if this setting type doesn't support a minimum.
    pub minimum_value: Option<LuaModSettingPrototypeMinimumValueUnion>,
    /// The mod that owns this setting.
    pub mod_name: String,
    /// Name of this prototype.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    pub setting_type: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of a mod setting.
pub trait LuaModSettingPrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Prototype of a module category.
pub struct LuaModuleCategoryPrototype {
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    /// Name of this prototype.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of a module category.
pub trait LuaModuleCategoryPrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Prototype of a named noise expression.
pub struct LuaNamedNoiseExpression {
    /// The expression itself.
    pub expression: NoiseExpression,
    /// Name of the property that this expression is intended to provide a value for, if any.
    pub intended_property: String,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    /// Name of this prototype.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of a named noise expression.
pub trait LuaNamedNoiseExpressionMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Prototype of a noise layer.
pub struct LuaNoiseLayerPrototype {
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    /// Name of this prototype.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of a noise layer.
pub trait LuaNoiseLayerPrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Prototype of an optimized particle.
pub struct LuaParticlePrototype {
    pub ended_in_water_trigger_effect: TriggerEffectItem,
    pub life_time: u32,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub mining_particle_frame_speed: f32,
    pub movement_modifier: f32,
    pub movement_modifier_when_on_ground: f32,
    /// Name of this prototype.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    pub regular_trigger_effect: TriggerEffectItem,
    pub regular_trigger_effect_frequency: u32,
    pub render_layer: RenderLayer,
    pub render_layer_when_on_ground: RenderLayer,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of an optimized particle.
pub trait LuaParticlePrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// A permission group that defines what players in this group are allowed to do.
pub struct LuaPermissionGroup {
    /// The group ID
    pub group_id: u32,
    /// The name of this group.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The players in this group.
    pub players: Vec<LuaPlayer>,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// A permission group that defines what players in this group are allowed to do.
pub trait LuaPermissionGroupMethods {
    /// Adds the given player to this group.
    fn add_player(player: PlayerIdentification) -> bool;
    /// Whether this group allows the given action.
    fn allows_action(action: InputAction) -> bool;
    /// Destroys this group.
    fn destroy() -> bool;
    /// All methods and properties that this object supports.
    fn help() -> String;
    /// Removes the given player from this group.
    fn remove_player(player: PlayerIdentification) -> bool;
    /// Sets whether this group allows the performance the given action.
    fn set_allows_action(action: InputAction, allow_action: bool) -> bool;
}

/// All permission groups.
pub struct LuaPermissionGroups {
    /// All of the permission groups.
    pub groups: Vec<LuaPermissionGroup>,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

pub enum LuaPermissionGroupsMethodsGetGroupGroupUnion {
    String(String),
    Uint(u32),
}

/// All permission groups.
pub trait LuaPermissionGroupsMethods {
    /// Creates a new permission group.
    fn create_group(name: String) -> Option<LuaPermissionGroup>;
    /// Gets the permission group with the given name or group ID.
    fn get_group(group: LuaPermissionGroupsMethodsGetGroupGroupUnion) -> Option<LuaPermissionGroup>;
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// A player in the game. Pay attention that a player may or may not have a character, which is the [LuaEntity](LuaEntity) of the little guy running around the world doing things.
pub struct LuaPlayer {
    pub lua_control: Box<LuaControl>,
    /// `true` if the player is an admin.
    pub admin: bool,
    /// How many ticks since the last action of this player
    pub afk_time: u32,
    /// If the main inventory will be auto sorted.
    pub auto_sort_main_inventory: bool,
    /// The item stack containing a blueprint to be setup.
    pub blueprint_to_setup: LuaItemStack,
    /// The character attached to this player, if any. Returns `nil` when the player is disconnected (see [LuaPlayer::connected](LuaPlayer::connected)).
    pub character: Option<Box<LuaEntity>>,
    /// The color used when this player talks in game.
    pub chat_color: Color,
    /// The color associated with the player. This will be used to tint the player's character as well as their buildings and vehicles.
    pub color: Color,
    /// `true` if the player is currently connected to the game.
    pub connected: bool,
    pub controller_type: Controllers,
    /// Returns true if the current item stack in cursor will be destroyed after clearing the cursor. Manually putting it into inventory still preserves the item. If the cursor stack is not one of the supported types (blueprint, blueprint-book, deconstruction-planner, upgrade-planner), write operation will be silently ignored.
    pub cursor_stack_temporary: bool,
    /// When in a cutscene; the character this player would be using once the cutscene is over, if any. Returns `nil` when the player is disconnected (see [LuaPlayer::connected](LuaPlayer::connected)).
    pub cutscene_character: Option<Box<LuaEntity>>,
    /// The display resolution for this player.
    pub display_resolution: DisplayResolution,
    /// The display scale for this player.
    pub display_scale: f64,
    /// The wire drag target for this player, if any.
    pub drag_target: Option<Box<DragTarget>>,
    /// The source entity used during entity settings copy-paste, if any.
    pub entity_copy_source: Option<Box<LuaEntity>>,
    /// The player's game view settings.
    pub game_view_settings: GameViewSettings,
    pub gui: Box<LuaGui>,
    /// The original location of the item in the cursor, marked with a hand. `nil` if the cursor stack is empty. When writing, the specified inventory slot must be empty and the cursor stack must not be empty.
    pub hand_location: Option<ItemStackLocation>,
    /// This player's index in [LuaGameScript::players](LuaGameScript::players) (unique ID). It is assigned when a player is created, and remains so (even when the player is not [connected](LuaPlayer::connected)) until the player is irreversably [removed](on_player_removed). Indexes of removed players can be reused.
    pub index: u32,
    /// The filters for this map editor infinity inventory settings.
    pub infinity_inventory_filters: Vec<InfinityInventoryFilter>,
    /// At what tick this player was last online.
    pub last_online: u32,
    /// The player's map view settings. To write to this, use a table containing the fields that should be changed.
    pub map_view_settings: MapViewSettings,
    /// `true` if the minimap is visible.
    pub minimap_enabled: bool,
    /// The current per-player settings for the this player, indexed by prototype name. Returns the same structure as [LuaSettings::get_player_settings](LuaSettings::get_player_settings). This table becomes invalid if its associated player does.
    ///
    /// Even though this attribute is marked as read-only, individual settings can be changed by overwriting their [ModSetting](ModSetting) table. Mods can only change their own settings. Using the in-game console, all player settings can be changed.
    pub mod_settings: HashMap<String, ModSetting>,
    /// The player's username.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// How many ticks did this player spend playing this save (all sessions combined)
    pub online_time: u32,
    /// `true` if the player opened itself. I.e. if they opened the character or god-controller GUI.
    pub opened_self: bool,
    /// The permission group this player is part of, if any.
    pub permission_group: Option<LuaPermissionGroup>,
    /// If items not included in this map editor infinity inventory filters should be removed.
    pub remove_unfiltered_items: bool,
    /// The render mode of the player, like map or zoom to world. The render mode can be set using [LuaPlayer::open_map](LuaPlayer::open_map), [LuaPlayer::zoom_to_world](LuaPlayer::zoom_to_world) and [LuaPlayer::close_map](LuaPlayer::close_map).
    pub render_mode: RenderMode,
    /// If `true`, circle and name of given player is rendered on the map/chart.
    pub show_on_map: bool,
    /// If `true`, zoom-to-world noise effect will be disabled and environmental sounds will be based on zoom-to-world view instead of position of player's character.
    pub spectator: bool,
    /// The stashed controller type, if any.
    pub stashed_controller_type: Option<Controllers>,
    /// The tag that is shown after the player in chat and on the map.
    pub tag: String,
    /// The number of ticks until this player will respawn. `nil` if this player is not waiting to respawn.
    ///
    /// Set to `nil` to immediately respawn the player.
    pub ticks_to_respawn: Option<u32>,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
    /// The player's zoom-level.
    pub zoom: f64,
}

pub enum LuaPlayerMethodsPipetteEntityEntityUnion {
    String(String),
    LuaEntity(LuaEntity),
    LuaEntityPrototype(LuaEntityPrototype),
}

pub enum LuaPlayerMethodsRemoveAlertPrototypeUnion {
    LuaEntityPrototype(LuaEntityPrototype),
    String(String),
}

pub enum LuaPlayerMethodsSetInfinityInventoryFilterFilterUnion {
    InfinityInventoryFilter(InfinityInventoryFilter),
    Nil,
}

pub enum LuaPlayerMethodsSetQuickBarSlotFilterUnion {
    String(String),
    LuaItemPrototype(LuaItemPrototype),
    LuaItemStack(LuaItemStack),
}

/// A player in the game. Pay attention that a player may or may not have a character, which is the [LuaEntity](LuaEntity) of the little guy running around the world doing things.
pub trait LuaPlayerMethods {
    /// Gets a copy of the currently selected blueprint in the clipboard queue into the player's cursor, as if the player activated Paste.
    fn activate_paste();
    /// Adds an alert to this player for the given entity of the given alert type.
    fn add_alert(entity: LuaEntity, typ: AlertType);
    /// Adds a custom alert to this player.
    fn add_custom_alert(entity: LuaEntity, icon: SignalID, message: LocalisedString, show_on_map: bool);
    /// Adds the given recipe to the list of recipe notifications for this player.
    fn add_recipe_notification(recipe: String);
    /// Adds the given blueprint to this player's clipboard queue.
    fn add_to_clipboard(blueprint: LuaItemStack);
    /// Associates a character with this player.
    fn associate_character(character: LuaEntity);
    /// Builds whatever is in the cursor on the surface the player is on. The cursor stack will automatically be reduced as if the player built normally.
    fn build_from_cursor(alt: bool, direction: Direction, position: MapPosition, skip_fog_of_war: bool, terrain_building_size: u32);
    /// Checks if this player can build what ever is in the cursor on the surface the player is on.
    fn can_build_from_cursor(alt: bool, direction: Direction, position: MapPosition, skip_fog_of_war: bool, terrain_building_size: u32) -> bool;
    /// Checks if this player can build the given entity at the given location on the surface the player is on.
    fn can_place_entity(direction: Direction, name: String, position: MapPosition) -> bool;
    /// Clear the chat console.
    fn clear_console();
    /// Invokes the "clear cursor" action on the player as if the user pressed it.
    fn clear_cursor() -> bool;
    /// Clears all recipe notifications for this player.
    fn clear_recipe_notifications();
    /// Clears the players selection tool selection position.
    fn clear_selection();
    /// Queues request to switch to the normal game view from the map or zoom to world view. Render mode change requests are processed before rendering of the next frame.
    fn close_map();
    /// Asks the player if they would like to connect to the given server.
    fn connect_to_server(address: String, description: LocalisedString, name: LocalisedString, password: String);
    /// Creates and attaches a character entity to this player.
    fn create_character(character: String) -> bool;
    /// Spawn flying text that is only visible to this player. Either `position` or `create_at_cursor` are required. When `create_at_cursor` is `true`, all parameters other than `text` are ignored.
    fn create_local_flying_text(color: Color, create_at_cursor: bool, position: MapPosition, speed: f64, text: LocalisedString, time_to_live: u32);
    /// Disables alerts for the given alert category.
    fn disable_alert(alert_type: AlertType) -> bool;
    /// Disable recipe groups.
    fn disable_recipe_groups();
    /// Disable recipe subgroups.
    fn disable_recipe_subgroups();
    /// Disassociates a character from this player. This is functionally the same as setting [LuaEntity::associated_player](LuaEntity::associated_player) to `nil`.
    fn disassociate_character(character: LuaEntity);
    /// Start/end wire dragging at the specified location, wire type is based on the cursor contents
    fn drag_wire(position: MapPosition) -> bool;
    /// Enables alerts for the given alert category.
    fn enable_alert(alert_type: AlertType) -> bool;
    /// Enable recipe groups.
    fn enable_recipe_groups();
    /// Enable recipe subgroups.
    fn enable_recipe_subgroups();
    /// Exit the current cutscene. Errors if not in a cutscene.
    fn exit_cutscene();
    /// Gets which quick bar page is being used for the given screen page or `nil` if not known.
    fn get_active_quick_bar_page(index: u32) -> Option<u8>;
    /// Get all alerts matching the given filters, or all alerts if no filters are given.
    fn get_alerts(entity: LuaEntity, position: MapPosition, prototype: LuaEntityPrototype, surface: SurfaceIdentification, typ: AlertType) -> HashMap<u32, HashMap<AlertType, Vec<Alert>>>;
    /// The characters associated with this player.
    fn get_associated_characters() -> Vec<LuaEntity>;
    /// Get the current goal description, as a localised string.
    fn get_goal_description() -> LocalisedString;
    /// Gets the filter for this map editor infinity filters at the given index or `nil` if the filter index doesn't exist or is empty.
    fn get_infinity_inventory_filter(index: u32) -> Option<InfinityInventoryFilter>;
    /// Gets the quick bar filter for the given slot or `nil`.
    fn get_quick_bar_slot(index: u32) -> Option<LuaItemPrototype>;
    /// All methods and properties that this object supports.
    fn help() -> String;
    /// If the given alert type is currently enabled.
    fn is_alert_enabled(alert_type: AlertType) -> bool;
    /// If the given alert type is currently muted.
    fn is_alert_muted(alert_type: AlertType) -> bool;
    /// Is a custom Lua shortcut currently available?
    fn is_shortcut_available(prototype_name: String) -> bool;
    /// Is a custom Lua shortcut currently toggled?
    fn is_shortcut_toggled(prototype_name: String) -> bool;
    /// Jump to the specified cutscene waypoint. Only works when the player is viewing a cutscene.
    fn jump_to_cutscene_waypoint(waypoint_index: u32);
    /// Logs a dictionary of chunks -> active entities for the surface this player is on.
    fn log_active_entity_chunk_counts();
    /// Logs a dictionary of active entities -> count for the surface this player is on.
    fn log_active_entity_counts();
    /// Mutes alerts for the given alert category.
    fn mute_alert(alert_type: AlertType) -> bool;
    /// Queues a request to open the map at the specified position. If the map is already opened, the request will simply set the position, scale, and entity to follow. Render mode change requests are processed before rendering of the next frame.
    fn open_map(entity: LuaEntity, position: MapPosition, scale: f64);
    /// Invokes the "smart pipette" action on the player as if the user pressed it.
    fn pipette_entity(entity: LuaPlayerMethodsPipetteEntityEntityUnion) -> bool;
    /// Play a sound for this player.
    fn play_sound(override_sound_type: SoundType, path: SoundPath, position: MapPosition, volume_modifier: f64);
    /// Print text to the chat console.
    fn print(color: Color, message: LocalisedString);
    /// Print entity statistics to the player's console.
    fn print_entity_statistics(entities: Vec<String>);
    /// Print LuaObject counts per mod.
    fn print_lua_object_statistics();
    /// Print construction robot job counts to the players console.
    fn print_robot_jobs();
    /// Removes all alerts matching the given filters or if an empty filters table is given all alerts are removed.
    fn remove_alert(entity: LuaEntity, icon: SignalID, message: LocalisedString, position: MapPosition, prototype: LuaPlayerMethodsRemoveAlertPrototypeUnion, surface: SurfaceIdentification, typ: AlertType);
    /// Requests a translation for the given localised string. If the request is successful, the [on_string_translated](on_string_translated) event will be fired with the results.
    fn request_translation(localised_string: LocalisedString) -> Option<u32>;
    /// Requests translation for the given set of localised strings. If the request is successful, a [on_string_translated](on_string_translated) event will be fired for each string with the results.
    fn request_translations(localised_strings: Vec<LocalisedString>) -> Option<Vec<u32>>;
    /// Sets which quick bar page is being used for the given screen page.
    fn set_active_quick_bar_page(page_index: u32, screen_index: u32);
    /// Set the controller type of the player.
    fn set_controller(character: LuaEntity, chart_mode_cutoff: f64, final_transition_time: u32, start_position: MapPosition, start_zoom: f64, typ: Controllers, waypoints: CutsceneWaypoint);
    /// Setup the screen to be shown when the game is finished.
    fn set_ending_screen_data(file: String, message: LocalisedString);
    /// Set the text in the goal window (top left).
    fn set_goal_description(only_update: bool, text: LocalisedString);
    /// Sets the filter for this map editor infinity filters at the given index.
    fn set_infinity_inventory_filter(filter: LuaPlayerMethodsSetInfinityInventoryFilterFilterUnion, index: u32);
    /// Sets the quick bar filter for the given slot.
    fn set_quick_bar_slot(filter: LuaPlayerMethodsSetQuickBarSlotFilterUnion, index: u32);
    /// Make a custom Lua shortcut available or unavailable.
    fn set_shortcut_available(available: bool, prototype_name: String);
    /// Toggle or untoggle a custom Lua shortcut
    fn set_shortcut_toggled(prototype_name: String, toggled: bool);
    /// Starts selection with selection tool from the specified position. Does nothing if the players cursor is not a selection tool.
    fn start_selection(position: MapPosition, selection_mode: String);
    /// Toggles this player into or out of the map editor. Does nothing if this player isn't an admin or if the player doesn't have permission to use the map editor.
    fn toggle_map_editor();
    /// Unlock the achievements of the given player. This has any effect only when this is the local player, the achievement isn't unlocked so far and the achievement is of the type "achievement".
    fn unlock_achievement(name: String);
    /// Unmutes alerts for the given alert category.
    fn unmute_alert(alert_type: AlertType) -> bool;
    /// Uses the current item in the cursor if it's a capsule or does nothing if not.
    fn use_from_cursor(position: MapPosition);
    /// Queues a request to zoom to world at the specified position. If the player is already zooming to world, the request will simply set the position, scale, and entity to follow. Render mode change requests are processed before rendering of the next frame.
    fn zoom_to_world(entity: LuaEntity, position: MapPosition, scale: f64);
}

/// An object used to measure script performance.
pub struct LuaProfiler {
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// An object used to measure script performance.
pub trait LuaProfilerMethods {
    /// Add the duration of another timer to this timer. Useful to reduce start/stop overhead when accumulating time onto many timers at once.
    fn add(other: LuaProfiler);
    /// Divides the current duration by a set value. Useful for calculating the average of many iterations.
    fn divide(number: f64);
    /// All methods and properties that this object supports.
    fn help() -> String;
    /// Resets the clock, also restarting it.
    fn reset();
    /// Start the clock again, without resetting it.
    fn restart();
    /// Stops the clock.
    fn stop();
}

/// Control behavior for programmable speakers.
pub struct LuaProgrammableSpeakerControlBehavior {
    pub lua_control_behavior: Box<LuaControlBehavior>,
    pub circuit_condition: CircuitConditionDefinition,
    pub circuit_parameters: ProgrammableSpeakerCircuitParameters,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Control behavior for programmable speakers.
pub trait LuaProgrammableSpeakerControlBehaviorMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// An interface to send messages to the calling RCON interface through the global object named `rcon`.
pub struct LuaRCON {
    /// This object's name.
    pub object_name: String,
}

/// An interface to send messages to the calling RCON interface through the global object named `rcon`.
pub trait LuaRCONMethods {
    /// Print text to the calling RCON interface if any.
    fn print(message: LocalisedString);
}

/// Control behavior for rail chain signals.
pub struct LuaRailChainSignalControlBehavior {
    pub lua_control_behavior: Box<LuaControlBehavior>,
    pub blue_signal: SignalID,
    pub green_signal: SignalID,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    pub orange_signal: SignalID,
    pub red_signal: SignalID,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Control behavior for rail chain signals.
pub trait LuaRailChainSignalControlBehaviorMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// A rail path.
pub struct LuaRailPath {
    /// The current rail index.
    pub current: u32,
    /// If the path goes from the front of the train
    pub is_front: bool,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// Array of the rails that this path travels over.
    pub rails: HashMap<u32, LuaEntity>,
    /// The total number of rails in this path.
    pub size: u32,
    /// The total path distance.
    pub total_distance: f64,
    /// The total distance travelled.
    pub travelled_distance: f64,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// A rail path.
pub trait LuaRailPathMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Control behavior for rail signals.
pub struct LuaRailSignalControlBehavior {
    pub lua_control_behavior: Box<LuaControlBehavior>,
    /// The circuit condition when controlling the signal through the circuit network.
    pub circuit_condition: CircuitConditionDefinition,
    /// If this will close the rail signal based off the circuit condition.
    pub close_signal: bool,
    pub green_signal: SignalID,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    pub orange_signal: SignalID,
    /// If this will read the rail signal state.
    pub read_signal: bool,
    pub red_signal: SignalID,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Control behavior for rail signals.
pub trait LuaRailSignalControlBehaviorMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// A deterministic random generator independent from the core games random generator that can be seeded and re-seeded at will. This random generator can be saved and loaded and will maintain its state. Note this is entirely different from calling [math.random](Libraries.html#math.random)() and you should be sure you actually want to use this over calling `math.random()`. If you aren't sure if you need to use this over calling `math.random()` then you probably don't need to use this.
pub struct LuaRandomGenerator {
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// A deterministic random generator independent from the core games random generator that can be seeded and re-seeded at will. This random generator can be saved and loaded and will maintain its state. Note this is entirely different from calling [math.random](Libraries.html#math.random)() and you should be sure you actually want to use this over calling `math.random()`. If you aren't sure if you need to use this over calling `math.random()` then you probably don't need to use this.
pub trait LuaRandomGeneratorMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
    /// Re-seeds the random generator with the given value.
    fn re_seed(seed: u32);
}

/// A crafting recipe. Recipes belong to forces (see [LuaForce](LuaForce)) because some recipes are unlocked by research, and researches are per-force.
pub struct LuaRecipe {
    /// Category of the recipe.
    pub category: String,
    /// Can the recipe be used?
    pub enabled: bool,
    /// Energy required to execute this recipe. This directly affects the crafting time: Recipe's energy is exactly its crafting time in seconds, when crafted in an assembling machine with crafting speed exactly equal to one.
    pub energy: f64,
    /// The force that owns this recipe.
    pub force: LuaForce,
    /// Group of this recipe.
    pub group: LuaGroup,
    /// Is the recipe hidden? Hidden recipe don't show up in the crafting menu.
    pub hidden: bool,
    /// Is the recipe hidden from flow statistics?
    pub hidden_from_flow_stats: bool,
    /// The ingredients to this recipe.
    pub ingredients: Vec<Ingredient>,
    pub localised_description: LocalisedString,
    /// Localised name of the recipe.
    pub localised_name: LocalisedString,
    /// Name of the recipe. This can be different than the name of the result items as there could be more recipes to make the same item.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    /// The results/products of this recipe.
    pub products: Vec<Product>,
    /// The prototype for this recipe.
    pub prototype: LuaRecipePrototype,
    /// Subgroup of this recipe.
    pub subgroup: LuaGroup,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// A crafting recipe. Recipes belong to forces (see [LuaForce](LuaForce)) because some recipes are unlocked by research, and researches are per-force.
pub trait LuaRecipeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
    /// Reload the recipe from the prototype.
    fn reload();
}

/// Prototype of a recipe category.
pub struct LuaRecipeCategoryPrototype {
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    /// Name of this prototype.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of a recipe category.
pub trait LuaRecipeCategoryPrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// A crafting recipe prototype.
pub struct LuaRecipePrototype {
    /// If this recipe is enabled for the purpose of intermediate hand-crafting.
    pub allow_as_intermediate: bool,
    /// Is this recipe allowed to be broken down for the recipe tooltip "Total raw" calculations?
    pub allow_decomposition: bool,
    /// If the recipe is allowed to have the extra inserter overload bonus applied (4 * stack inserter stack size).
    pub allow_inserter_overload: bool,
    /// If this recipe is allowed to use intermediate recipes when hand-crafting.
    pub allow_intermediates: bool,
    /// Should this recipe always show "Made in" in the tooltip?
    pub always_show_made_in: bool,
    /// If the products are always shown in the recipe tooltip.
    pub always_show_products: bool,
    /// Category of the recipe.
    pub category: String,
    /// The emissions multiplier for this recipe.
    pub emissions_multiplier: f64,
    /// If this recipe prototype is enabled by default (enabled at the beginning of a game).
    pub enabled: bool,
    /// Energy required to execute this recipe. This directly affects the crafting time: Recipe's energy is exactly its crafting time in seconds, when crafted in an assembling machine with crafting speed exactly equal to one.
    pub energy: f64,
    /// Group of this recipe.
    pub group: LuaGroup,
    /// Is the recipe hidden? Hidden recipe don't show up in the crafting menu.
    pub hidden: bool,
    /// Is the recipe hidden from flow statistics (item/fluid production statistics)?
    pub hidden_from_flow_stats: bool,
    /// Is the recipe hidden from player crafting? The recipe will still show up for selection in machines.
    pub hidden_from_player_crafting: bool,
    /// The ingredients to this recipe.
    pub ingredients: Vec<Ingredient>,
    pub localised_description: LocalisedString,
    /// Localised name of the recipe.
    pub localised_name: LocalisedString,
    /// The main product of this recipe, if any.
    pub main_product: Option<Product>,
    /// Name of the recipe. This can be different than the name of the result items as there could be more recipes to make the same item.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    /// Used to determine how many extra items are put into an assembling machine before it's considered "full enough".
    pub overload_multiplier: u32,
    /// The results/products of this recipe.
    pub products: Vec<Product>,
    /// The multiplier used when this recipe is copied from an assembling machine to a requester chest. For each item in the recipe the item count * this value is set in the requester chest.
    pub request_paste_multiplier: u32,
    /// If the amount is shown in the recipe tooltip title when the recipe produces more than 1 product.
    pub show_amount_in_title: bool,
    /// Subgroup of this recipe.
    pub subgroup: LuaGroup,
    /// Is this recipe unlocks the result item(s) so they're shown in filter-select GUIs.
    pub unlock_results: bool,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// A crafting recipe prototype.
pub trait LuaRecipePrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Registry of interfaces between scripts. An interface is simply a dictionary mapping names to functions. A script or mod can then register an interface with [LuaRemote](LuaRemote), after that any script can call the registered functions, provided it knows the interface name and the desired function name. An instance of LuaRemote is available through the global object named `remote`.
pub struct LuaRemote {
    /// List of all registered interfaces. For each interface name, `remote.interfaces[name]` is a dictionary mapping the interface's registered functions to `true`.
    pub interfaces: HashMap<String, HashSet<String>>,
    /// This object's name.
    pub object_name: String,
}

/// Registry of interfaces between scripts. An interface is simply a dictionary mapping names to functions. A script or mod can then register an interface with [LuaRemote](LuaRemote), after that any script can call the registered functions, provided it knows the interface name and the desired function name. An instance of LuaRemote is available through the global object named `remote`.
pub trait LuaRemoteMethods {
    /// Add a remote interface.
    fn add_interface(functions: HashMap<String, fn() -> ()>, name: String);
    /// Call a function of an interface.
    fn call(function: String, interface: String) -> Option<Any>;
    /// Removes an interface with the given name.
    fn remove_interface(name: String) -> bool;
}

/// Allows rendering of geometric shapes, text and sprites in the game world through the global object named `rendering`. Each render object is identified by an id that is universally unique for the lifetime of a whole game.
pub struct LuaRendering {
    /// This object's name.
    pub object_name: String,
}

pub enum LuaRenderingMethodsDrawAnimationOrientationTargetUnion {
    MapPosition(MapPosition),
    LuaEntity(LuaEntity),
}

pub enum LuaRenderingMethodsDrawAnimationTargetUnion {
    MapPosition(MapPosition),
    LuaEntity(LuaEntity),
}

pub enum LuaRenderingMethodsDrawArcTargetUnion {
    MapPosition(MapPosition),
    LuaEntity(LuaEntity),
}

pub enum LuaRenderingMethodsDrawCircleTargetUnion {
    MapPosition(MapPosition),
    LuaEntity(LuaEntity),
}

pub enum LuaRenderingMethodsDrawLightTargetUnion {
    MapPosition(MapPosition),
    LuaEntity(LuaEntity),
}

pub enum LuaRenderingMethodsDrawLineFromUnion {
    MapPosition(MapPosition),
    LuaEntity(LuaEntity),
}

pub enum LuaRenderingMethodsDrawLineToUnion {
    MapPosition(MapPosition),
    LuaEntity(LuaEntity),
}

pub enum LuaRenderingMethodsDrawPolygonOrientationTargetUnion {
    MapPosition(MapPosition),
    LuaEntity(LuaEntity),
}

pub enum LuaRenderingMethodsDrawPolygonTargetUnion {
    MapPosition(MapPosition),
    LuaEntity(LuaEntity),
}

pub enum LuaRenderingMethodsDrawRectangleLeftTopUnion {
    MapPosition(MapPosition),
    LuaEntity(LuaEntity),
}

pub enum LuaRenderingMethodsDrawRectangleRightBottomUnion {
    MapPosition(MapPosition),
    LuaEntity(LuaEntity),
}

pub enum LuaRenderingMethodsDrawSpriteOrientationTargetUnion {
    MapPosition(MapPosition),
    LuaEntity(LuaEntity),
}

pub enum LuaRenderingMethodsDrawSpriteTargetUnion {
    MapPosition(MapPosition),
    LuaEntity(LuaEntity),
}

pub enum LuaRenderingMethodsDrawTextTargetUnion {
    MapPosition(MapPosition),
    LuaEntity(LuaEntity),
}

pub enum LuaRenderingMethodsSetCornersLeftTopUnion {
    MapPosition(MapPosition),
    LuaEntity(LuaEntity),
}

pub enum LuaRenderingMethodsSetCornersRightBottomUnion {
    MapPosition(MapPosition),
    LuaEntity(LuaEntity),
}

pub enum LuaRenderingMethodsSetFromFromUnion {
    MapPosition(MapPosition),
    LuaEntity(LuaEntity),
}

pub enum LuaRenderingMethodsSetLeftTopLeftTopUnion {
    MapPosition(MapPosition),
    LuaEntity(LuaEntity),
}

pub enum LuaRenderingMethodsSetOrientationTargetOrientationTargetUnion {
    MapPosition(MapPosition),
    LuaEntity(LuaEntity),
}

pub enum LuaRenderingMethodsSetRightBottomRightBottomUnion {
    MapPosition(MapPosition),
    LuaEntity(LuaEntity),
}

pub enum LuaRenderingMethodsSetTargetTargetUnion {
    MapPosition(MapPosition),
    LuaEntity(LuaEntity),
}

pub enum LuaRenderingMethodsSetToToUnion {
    MapPosition(MapPosition),
    LuaEntity(LuaEntity),
}

/// Allows rendering of geometric shapes, text and sprites in the game world through the global object named `rendering`. Each render object is identified by an id that is universally unique for the lifetime of a whole game.
pub trait LuaRenderingMethods {
    /// Reorder this object so that it is drawn in front of the already existing objects.
    fn bring_to_front(id: u64);
    /// Destroys all render objects. Passing an empty string (`""`)
    fn clear(mod_name: String);
    /// Destroy the object with the given id.
    fn destroy(id: u64);
    /// Create an animation.
    fn draw_animation(animation: String, animation_offset: f64, animation_speed: f64, forces: Vec<ForceIdentification>, only_in_alt_mode: bool, orientation: RealOrientation, orientation_target: LuaRenderingMethodsDrawAnimationOrientationTargetUnion, orientation_target_offset: Vector, oriented_offset: Vector, players: Vec<PlayerIdentification>, render_layer: RenderLayer, surface: SurfaceIdentification, target: LuaRenderingMethodsDrawAnimationTargetUnion, target_offset: Vector, time_to_live: u32, tint: Color, use_target_orientation: bool, visible: bool, x_scale: f64, y_scale: f64) -> u64;
    /// Create an arc.
    fn draw_arc(angle: f32, color: Color, draw_on_ground: bool, forces: Vec<ForceIdentification>, max_radius: f64, min_radius: f64, only_in_alt_mode: bool, players: Vec<PlayerIdentification>, start_angle: f32, surface: SurfaceIdentification, target: LuaRenderingMethodsDrawArcTargetUnion, target_offset: Vector, time_to_live: u32, visible: bool) -> u64;
    /// Create a circle.
    fn draw_circle(color: Color, draw_on_ground: bool, filled: bool, forces: Vec<ForceIdentification>, only_in_alt_mode: bool, players: Vec<PlayerIdentification>, radius: f64, surface: SurfaceIdentification, target: LuaRenderingMethodsDrawCircleTargetUnion, target_offset: Vector, time_to_live: u32, visible: bool, width: f32) -> u64;
    /// Create a light.
    fn draw_light(color: Color, forces: Vec<ForceIdentification>, intensity: f32, minimum_darkness: f32, only_in_alt_mode: bool, orientation: RealOrientation, oriented: bool, players: Vec<PlayerIdentification>, scale: f32, sprite: SpritePath, surface: SurfaceIdentification, target: LuaRenderingMethodsDrawLightTargetUnion, target_offset: Vector, time_to_live: u32, visible: bool) -> u64;
    /// Create a line.
    fn draw_line(color: Color, dash_length: f64, draw_on_ground: bool, forces: Vec<ForceIdentification>, from: LuaRenderingMethodsDrawLineFromUnion, from_offset: Vector, gap_length: f64, only_in_alt_mode: bool, players: Vec<PlayerIdentification>, surface: SurfaceIdentification, time_to_live: u32, to: LuaRenderingMethodsDrawLineToUnion, to_offset: Vector, visible: bool, width: f32) -> u64;
    /// Create a triangle mesh defined by a triangle strip.
    fn draw_polygon(color: Color, draw_on_ground: bool, forces: Vec<ForceIdentification>, only_in_alt_mode: bool, orientation: RealOrientation, orientation_target: LuaRenderingMethodsDrawPolygonOrientationTargetUnion, orientation_target_offset: Vector, players: Vec<PlayerIdentification>, surface: SurfaceIdentification, target: LuaRenderingMethodsDrawPolygonTargetUnion, target_offset: Vector, time_to_live: u32, use_target_orientation: bool, vertices: Vec<ScriptRenderVertexTarget>, visible: bool) -> u64;
    /// Create a rectangle.
    fn draw_rectangle(color: Color, draw_on_ground: bool, filled: bool, forces: Vec<ForceIdentification>, left_top: LuaRenderingMethodsDrawRectangleLeftTopUnion, left_top_offset: Vector, only_in_alt_mode: bool, players: Vec<PlayerIdentification>, right_bottom: LuaRenderingMethodsDrawRectangleRightBottomUnion, right_bottom_offset: Vector, surface: SurfaceIdentification, time_to_live: u32, visible: bool, width: f32) -> u64;
    /// Create a sprite.
    fn draw_sprite(forces: Vec<ForceIdentification>, only_in_alt_mode: bool, orientation: RealOrientation, orientation_target: LuaRenderingMethodsDrawSpriteOrientationTargetUnion, orientation_target_offset: Vector, oriented_offset: Vector, players: Vec<PlayerIdentification>, render_layer: RenderLayer, sprite: SpritePath, surface: SurfaceIdentification, target: LuaRenderingMethodsDrawSpriteTargetUnion, target_offset: Vector, time_to_live: u32, tint: Color, use_target_orientation: bool, visible: bool, x_scale: f64, y_scale: f64) -> u64;
    /// Create a text.
    fn draw_text(alignment: String, color: Color, draw_on_ground: bool, font: String, forces: Vec<ForceIdentification>, only_in_alt_mode: bool, orientation: RealOrientation, players: Vec<PlayerIdentification>, scale: f64, scale_with_zoom: bool, surface: SurfaceIdentification, target: LuaRenderingMethodsDrawTextTargetUnion, target_offset: Vector, text: LocalisedString, time_to_live: u32, use_rich_text: bool, vertical_alignment: String, visible: bool) -> u64;
    /// Get the alignment of the text with this id.
    fn get_alignment(id: u64) -> Option<String>;
    /// Gets an array of all valid object ids.
    fn get_all_ids(mod_name: String) -> Vec<u64>;
    /// Get the angle of the arc with this id.
    fn get_angle(id: u64) -> Option<f32>;
    /// Get the animation prototype name of the animation with this id.
    fn get_animation(id: u64) -> Option<String>;
    /// Get the animation offset of the animation with this id.
    fn get_animation_offset(id: u64) -> Option<f64>;
    /// Get the animation speed of the animation with this id.
    fn get_animation_speed(id: u64) -> Option<f64>;
    /// Get the color or tint of the object with this id.
    fn get_color(id: u64) -> Option<Color>;
    /// Get the dash length of the line with this id.
    fn get_dash_length(id: u64) -> Option<f64>;
    /// Get whether this is being drawn on the ground, under most entities and sprites.
    fn get_draw_on_ground(id: u64) -> bool;
    /// Get if the circle or rectangle with this id is filled.
    fn get_filled(id: u64) -> Option<bool>;
    /// Get the font of the text with this id.
    fn get_font(id: u64) -> Option<String>;
    /// Get the forces that the object with this id is rendered to or `nil` if visible to all forces.
    fn get_forces(id: u64) -> Option<Vec<LuaForce>>;
    /// Get from where the line with this id is drawn.
    fn get_from(id: u64) -> Option<ScriptRenderTarget>;
    /// Get the length of the gaps in the line with this id.
    fn get_gap_length(id: u64) -> Option<f64>;
    /// Get the intensity of the light with this id.
    fn get_intensity(id: u64) -> Option<f32>;
    /// Get where top left corner of the rectangle with this id is drawn.
    fn get_left_top(id: u64) -> Option<ScriptRenderTarget>;
    /// Get the radius of the outer edge of the arc with this id.
    fn get_max_radius(id: u64) -> Option<f64>;
    /// Get the radius of the inner edge of the arc with this id.
    fn get_min_radius(id: u64) -> Option<f64>;
    /// Get the minimum darkness at which the light with this id is rendered.
    fn get_minimum_darkness(id: u64) -> Option<f32>;
    /// Get whether this is only rendered in alt-mode.
    fn get_only_in_alt_mode(id: u64) -> bool;
    /// Get the orientation of the object with this id.
    fn get_orientation(id: u64) -> Option<RealOrientation>;
    /// The object rotates so that it faces this target. Note that `orientation` is still applied to the object. Get the orientation_target of the object with this id.
    fn get_orientation_target(id: u64) -> Option<ScriptRenderTarget>;
    /// Get if the light with this id is rendered has the same orientation as the target entity. Note that `orientation` is still applied to the sprite.
    fn get_oriented(id: u64) -> Option<bool>;
    /// Offsets the center of the sprite or animation if `orientation_target` is given. This offset will rotate together with the sprite or animation. Get the oriented_offset of the sprite or animation with this id.
    fn get_oriented_offset(id: u64) -> Option<Vector>;
    /// Get the players that the object with this id is rendered to or `nil` if visible to all players.
    fn get_players(id: u64) -> Option<Vec<LuaPlayer>>;
    /// Get the radius of the circle with this id.
    fn get_radius(id: u64) -> Option<f64>;
    /// Get the render layer of the sprite or animation with this id.
    fn get_render_layer(id: u64) -> Option<RenderLayer>;
    /// Get where bottom right corner of the rectangle with this id is drawn.
    fn get_right_bottom(id: u64) -> Option<ScriptRenderTarget>;
    /// Get the scale of the text or light with this id.
    fn get_scale(id: u64) -> Option<f64>;
    /// Get if the text with this id scales with player zoom.
    fn get_scale_with_zoom(id: u64) -> Option<bool>;
    /// Get the sprite of the sprite or light with this id.
    fn get_sprite(id: u64) -> Option<SpritePath>;
    /// Get where the arc with this id starts.
    fn get_start_angle(id: u64) -> Option<f32>;
    /// The surface the object with this id is rendered on.
    fn get_surface(id: u64) -> LuaSurface;
    /// Get where the object with this id is drawn.
    fn get_target(id: u64) -> Option<ScriptRenderTarget>;
    /// Get the text that is displayed by the text with this id.
    fn get_text(id: u64) -> Option<LocalisedString>;
    /// Get the time to live of the object with this id. This will be 0 if the object does not expire.
    fn get_time_to_live(id: u64) -> u32;
    /// Get where the line with this id is drawn to.
    fn get_to(id: u64) -> Option<ScriptRenderTarget>;
    /// Gets the type of the given object. The types are "text", "line", "circle", "rectangle", "arc", "polygon", "sprite", "light" and "animation".
    fn get_type(id: u64) -> String;
    /// Get if the text with this id parses rich text tags.
    fn get_use_rich_text(id: u64) -> Option<bool>;
    /// Get whether this uses the target orientation.
    fn get_use_target_orientation(id: u64) -> bool;
    /// Get the vertical alignment of the text with this id.
    fn get_vertical_alignment(id: u64) -> Option<String>;
    /// Get the vertices of the polygon with this id.
    fn get_vertices(id: u64) -> Option<Vec<ScriptRenderTarget>>;
    /// Get whether this is rendered to anyone at all.
    fn get_visible(id: u64) -> bool;
    /// Get the width of the object with this id. Value is in pixels (32 per tile).
    fn get_width(id: u64) -> Option<f32>;
    /// Get the horizontal scale of the sprite or animation with this id.
    fn get_x_scale(id: u64) -> Option<f64>;
    /// Get the vertical scale of the sprite or animation with this id.
    fn get_y_scale(id: u64) -> Option<f64>;
    /// Does a font with this name exist?
    fn is_font_valid(font_name: String) -> bool;
    /// Does a valid object with this id exist?
    fn is_valid(id: u64) -> bool;
    /// Set the alignment of the text with this id. Does nothing if this object is not a text.
    fn set_alignment(alignment: String, id: u64);
    /// Set the angle of the arc with this id. Does nothing if this object is not a arc.
    fn set_angle(angle: f32, id: u64);
    /// Set the animation prototype name of the animation with this id. Does nothing if this object is not an animation.
    fn set_animation(animation: String, id: u64);
    /// Set the animation offset of the animation with this id. Does nothing if this object is not an animation.
    fn set_animation_offset(animation_offset: f64, id: u64);
    /// Set the animation speed of the animation with this id. Does nothing if this object is not an animation.
    fn set_animation_speed(animation_speed: f64, id: u64);
    /// Set the color or tint of the object with this id. Does nothing if this object does not support color.
    fn set_color(color: Color, id: u64);
    /// Set the corners of the rectangle with this id. Does nothing if this object is not a rectangle.
    fn set_corners(id: u64, left_top: LuaRenderingMethodsSetCornersLeftTopUnion, left_top_offset: Vector, right_bottom: LuaRenderingMethodsSetCornersRightBottomUnion, right_bottom_offset: Vector);
    /// Set the dash length of the line with this id. Does nothing if this object is not a line.
    fn set_dash_length(dash_length: f64, id: u64);
    /// Set the length of the dashes and the length of the gaps in the line with this id. Does nothing if this object is not a line.
    fn set_dashes(dash_length: f64, gap_length: f64, id: u64);
    /// Set whether this is being drawn on the ground, under most entities and sprites.
    fn set_draw_on_ground(draw_on_ground: bool, id: u64);
    /// Set if the circle or rectangle with this id is filled. Does nothing if this object is not a circle or rectangle.
    fn set_filled(filled: bool, id: u64);
    /// Set the font of the text with this id. Does nothing if this object is not a text.
    fn set_font(font: String, id: u64);
    /// Set the forces that the object with this id is rendered to.
    fn set_forces(forces: Vec<ForceIdentification>, id: u64);
    /// Set from where the line with this id is drawn. Does nothing if the object is not a line.
    fn set_from(from: LuaRenderingMethodsSetFromFromUnion, from_offset: Vector, id: u64);
    /// Set the length of the gaps in the line with this id. Does nothing if this object is not a line.
    fn set_gap_length(gap_length: f64, id: u64);
    /// Set the intensity of the light with this id. Does nothing if this object is not a light.
    fn set_intensity(id: u64, intensity: f32);
    /// Set where top left corner of the rectangle with this id is drawn. Does nothing if this object is not a rectangle.
    fn set_left_top(id: u64, left_top: LuaRenderingMethodsSetLeftTopLeftTopUnion, left_top_offset: Vector);
    /// Set the radius of the outer edge of the arc with this id. Does nothing if this object is not a arc.
    fn set_max_radius(id: u64, max_radius: f64);
    /// Set the radius of the inner edge of the arc with this id. Does nothing if this object is not a arc.
    fn set_min_radius(id: u64, min_radius: f64);
    /// Set the minimum darkness at which the light with this id is rendered. Does nothing if this object is not a light.
    fn set_minimum_darkness(id: u64, minimum_darkness: f32);
    /// Set whether this is only rendered in alt-mode.
    fn set_only_in_alt_mode(id: u64, only_in_alt_mode: bool);
    /// Set the orientation of the object with this id. Does nothing if this object is not a text, polygon, sprite, light or animation.
    fn set_orientation(id: u64, orientation: RealOrientation);
    /// The object rotates so that it faces this target. Note that `orientation` is still applied to the object. Set the orientation_target of the object with this id. Does nothing if this object is not a polygon, sprite, or animation. Set to `nil` if the object should not have an orientation_target.
    fn set_orientation_target(id: u64, orientation_target: LuaRenderingMethodsSetOrientationTargetOrientationTargetUnion, orientation_target_offset: Vector);
    /// Set if the light with this id is rendered has the same orientation as the target entity. Does nothing if this object is not a light. Note that `orientation` is still applied to the sprite.
    fn set_oriented(id: u64, oriented: bool);
    /// Offsets the center of the sprite or animation if `orientation_target` is given. This offset will rotate together with the sprite or animation. Set the oriented_offset of the sprite or animation with this id. Does nothing if this object is not a sprite or animation.
    fn set_oriented_offset(id: u64, oriented_offset: Vector);
    /// Set the players that the object with this id is rendered to.
    fn set_players(id: u64, players: Vec<PlayerIdentification>);
    /// Set the radius of the circle with this id. Does nothing if this object is not a circle.
    fn set_radius(id: u64, radius: f64);
    /// Set the render layer of the sprite or animation with this id. Does nothing if this object is not a sprite or animation.
    fn set_render_layer(id: u64, render_layer: RenderLayer);
    /// Set where top bottom right of the rectangle with this id is drawn. Does nothing if this object is not a rectangle.
    fn set_right_bottom(id: u64, right_bottom: LuaRenderingMethodsSetRightBottomRightBottomUnion, right_bottom_offset: Vector);
    /// Set the scale of the text or light with this id. Does nothing if this object is not a text or light.
    fn set_scale(id: u64, scale: f64);
    /// Set if the text with this id scales with player zoom, resulting in it always being the same size on screen, and the size compared to the game world changes. Does nothing if this object is not a text.
    fn set_scale_with_zoom(id: u64, scale_with_zoom: bool);
    /// Set the sprite of the sprite or light with this id. Does nothing if this object is not a sprite or light.
    fn set_sprite(id: u64, sprite: SpritePath);
    /// Set where the arc with this id starts. Does nothing if this object is not a arc.
    fn set_start_angle(id: u64, start_angle: f32);
    /// Set where the object with this id is drawn. Does nothing if this object does not support target.
    fn set_target(id: u64, target: LuaRenderingMethodsSetTargetTargetUnion, target_offset: Vector);
    /// Set the text that is displayed by the text with this id. Does nothing if this object is not a text.
    fn set_text(id: u64, text: LocalisedString);
    /// Set the time to live of the object with this id. Set to 0 if the object should not expire.
    fn set_time_to_live(id: u64, time_to_live: u32);
    /// Set where the line with this id is drawn to. Does nothing if this object is not a line.
    fn set_to(id: u64, to: LuaRenderingMethodsSetToToUnion, to_offset: Vector);
    /// Set if the text with this id parses rich text tags.
    fn set_use_rich_text(id: u64, use_rich_text: bool);
    /// Set whether this uses the target orientation.
    fn set_use_target_orientation(id: u64, use_target_orientation: bool);
    /// Set the vertical alignment of the text with this id. Does nothing if this object is not a text.
    fn set_vertical_alignment(alignment: String, id: u64);
    /// Set the vertices of the polygon with this id. Does nothing if this object is not a polygon.
    fn set_vertices(id: u64, vertices: Vec<ScriptRenderVertexTarget>);
    /// Set whether this is rendered to anyone at all.
    fn set_visible(id: u64, visible: bool);
    /// Set the width of the object with this id. Does nothing if this object does not support width. Value is in pixels (32 per tile).
    fn set_width(id: u64, width: f32);
    /// Set the horizontal scale of the sprite or animation with this id. Does nothing if this object is not a sprite or animation.
    fn set_x_scale(id: u64, x_scale: f64);
    /// Set the vertical scale of the sprite or animation with this id. Does nothing if this object is not a sprite or animation.
    fn set_y_scale(id: u64, y_scale: f64);
}

/// Prototype of a resource category.
pub struct LuaResourceCategoryPrototype {
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    /// Name of this prototype.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of a resource category.
pub trait LuaResourceCategoryPrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Control behavior for roboports.
pub struct LuaRoboportControlBehavior {
    pub lua_control_behavior: Box<LuaControlBehavior>,
    pub available_construction_output_signal: SignalID,
    pub available_logistic_output_signal: SignalID,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// `true` if the roboport should report the logistics network content to the circuit network.
    pub read_logistics: bool,
    /// `true` if the roboport should report the robot statistics to the circuit network.
    pub read_robot_stats: bool,
    pub total_construction_output_signal: SignalID,
    pub total_logistic_output_signal: SignalID,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Control behavior for roboports.
pub trait LuaRoboportControlBehaviorMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Object containing mod settings of three distinct types: `startup`, `global`, and `player`. An instance of LuaSettings is available through the global object named `settings`.
pub struct LuaSettings {
    /// The current global mod settings, indexed by prototype name.
    ///
    /// Even though this attribute is marked as read-only, individual settings can be changed by overwriting their [ModSetting](ModSetting) table. Mods can only change their own settings. Using the in-game console, all player settings can be changed.
    pub global: HashMap<String, ModSetting>,
    /// This object's name.
    pub object_name: String,
    /// The default player mod settings for this map, indexed by prototype name.
    ///
    /// Even though this attribute is marked as read-only, individual settings can be changed by overwriting their [ModSetting](ModSetting) table. Mods can only change their own settings. Using the in-game console, all player settings can be changed.
    pub player: HashMap<String, ModSetting>,
    /// The startup mod settings, indexed by prototype name.
    pub startup: HashMap<String, ModSetting>,
}

/// Object containing mod settings of three distinct types: `startup`, `global`, and `player`. An instance of LuaSettings is available through the global object named `settings`.
pub trait LuaSettingsMethods {
    /// Gets the current per-player settings for the given player, indexed by prototype name. Returns the same structure as [LuaPlayer::mod_settings](LuaPlayer::mod_settings). This table becomes invalid if its associated player does.
    ///
    /// Even though this attribute is marked as read-only, individual settings can be changed by overwriting their [ModSetting](ModSetting) table. Mods can only change their own settings. Using the in-game console, all player settings can be changed.
    fn get_player_settings(player: PlayerIdentification) -> HashMap<String, ModSetting>;
}

/// Prototype of a shortcut.
pub struct LuaShortcutPrototype {
    pub action: String,
    /// The control input that is associated with this shortcut, if any.
    pub associated_control_input: Option<String>,
    /// The item to create when this shortcut is used, if any.
    pub item_to_spawn: Option<LuaItemPrototype>,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    /// Name of this prototype.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    /// The technology that needs to be researched once (in any save) for this shortcut to be unlocked (in all saves).
    pub technology_to_unlock: Option<LuaTechnologyPrototype>,
    pub toggleable: bool,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of a shortcut.
pub trait LuaShortcutPrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Control behavior for storage tanks.
pub struct LuaStorageTankControlBehavior {
    pub lua_control_behavior: Box<LuaControlBehavior>,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Control behavior for storage tanks.
pub trait LuaStorageTankControlBehaviorMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

pub enum LuaStyleExtraMarginWhenActivatedUnion {
    Int(i32),
    Array(Vec<i32>),
}

pub enum LuaStyleExtraPaddingWhenActivatedUnion {
    Int(i32),
    Array(Vec<i32>),
}

pub enum LuaStyleMarginUnion {
    Int(i32),
    Array(Vec<i32>),
}

pub enum LuaStylePaddingUnion {
    Int(i32),
    Array(Vec<i32>),
}

pub enum LuaStyleSizeUnion {
    Int(i32),
    Array(Vec<i32>),
}

/// Style of a GUI element. All of the attributes listed here may be `nil` if not available for a particular GUI element.
pub struct LuaStyle {
    pub badge_font: String,
    pub badge_horizontal_spacing: i32,
    pub bar_width: u32,
    /// Space between the table cell contents bottom and border.
    pub bottom_cell_padding: i32,
    pub bottom_margin: i32,
    pub bottom_padding: i32,
    /// Space between the table cell contents and border. Sets top/right/bottom/left cell paddings to this value.
    pub cell_padding: i32,
    pub clicked_font_color: Color,
    pub clicked_vertical_offset: i32,
    pub color: Color,
    /// Array containing the alignment for every column of this table element. Even though this property is marked as read-only, the alignment can be changed by indexing the LuaCustomTable, like so:
    pub column_alignments: HashMap<u32, Alignment>,
    pub default_badge_font_color: Color,
    pub disabled_badge_font_color: Color,
    pub disabled_font_color: Color,
    pub extra_bottom_margin_when_activated: i32,
    pub extra_bottom_padding_when_activated: i32,
    pub extra_left_margin_when_activated: i32,
    pub extra_left_padding_when_activated: i32,
    /// Sets `extra_top/right/bottom/left_margin_when_activated` to this value. An array with two values sets top/bottom margin to the first value and left/right margin to the second value. An array with four values sets top, right, bottom, left margin respectively.
    pub extra_margin_when_activated: LuaStyleExtraMarginWhenActivatedUnion,
    /// Sets `extra_top/right/bottom/left_padding_when_activated` to this value. An array with two values sets top/bottom padding to the first value and left/right padding to the second value. An array with four values sets top, right, bottom, left padding respectively.
    pub extra_padding_when_activated: LuaStyleExtraPaddingWhenActivatedUnion,
    pub extra_right_margin_when_activated: i32,
    pub extra_right_padding_when_activated: i32,
    pub extra_top_margin_when_activated: i32,
    pub extra_top_padding_when_activated: i32,
    pub font: String,
    pub font_color: Color,
    /// Gui of the [LuaGuiElement](LuaGuiElement) of this style.
    pub gui: Box<LuaGui>,
    /// Sets both minimal and maximal height to the given value.
    pub height: i32,
    /// Horizontal align of the inner content of the widget, if any. Possible values are "left", "center" or "right".
    pub horizontal_align: Option<String>,
    /// Horizontal space between individual cells.
    pub horizontal_spacing: i32,
    /// Whether the GUI element can be squashed (by maximal width of some parent element) horizontally. `nil` if this element does not support squashing. This is mainly meant to be used for scroll-pane The default value is false.
    pub horizontally_squashable: Option<bool>,
    /// Whether the GUI element stretches its size horizontally to other elements. `nil` if this element does not support stretching.
    pub horizontally_stretchable: Option<bool>,
    pub hovered_font_color: Color,
    /// Space between the table cell contents left and border.
    pub left_cell_padding: i32,
    pub left_margin: i32,
    pub left_padding: i32,
    /// Sets top/right/bottom/left margins to this value. An array with two values sets top/bottom margin to the first value and left/right margin to the second value. An array with four values sets top, right, bottom, left margin respectively.
    pub margin: LuaStyleMarginUnion,
    /// Maximal height ensures, that the widget will never be bigger than than that size. It can't be stretched to be bigger.
    pub maximal_height: i32,
    /// Maximal width ensures, that the widget will never be bigger than than that size. It can't be stretched to be bigger.
    pub maximal_width: i32,
    /// Minimal height ensures, that the widget will never be smaller than than that size. It can't be squashed to be smaller.
    pub minimal_height: i32,
    /// Minimal width ensures, that the widget will never be smaller than than that size. It can't be squashed to be smaller.
    pub minimal_width: i32,
    /// Name of this style.
    pub name: String,
    /// Natural height specifies the height of the element tries to have, but it can still be squashed/stretched to have a smaller or bigger size.
    pub natural_height: i32,
    /// Natural width specifies the width of the element tries to have, but it can still be squashed/stretched to have a smaller or bigger size.
    pub natural_width: i32,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// Sets top/right/bottom/left paddings to this value. An array with two values sets top/bottom padding to the first value and left/right padding to the second value. An array with four values sets top, right, bottom, left padding respectively.
    pub padding: LuaStylePaddingUnion,
    pub pie_progress_color: Color,
    /// How this GUI element handles rich text.
    pub rich_text_setting: RichTextSetting,
    /// Space between the table cell contents right and border.
    pub right_cell_padding: i32,
    pub right_margin: i32,
    pub right_padding: i32,
    pub selected_badge_font_color: Color,
    pub selected_clicked_font_color: Color,
    pub selected_font_color: Color,
    pub selected_hovered_font_color: Color,
    pub single_line: bool,
    /// Sets both width and height to the given value. Also accepts an array with two values, setting width to the first and height to the second one.
    pub size: LuaStyleSizeUnion,
    pub stretch_image_to_widget_size: bool,
    pub strikethrough_color: Color,
    /// Space between the table cell contents top and border.
    pub top_cell_padding: i32,
    pub top_margin: i32,
    pub top_padding: i32,
    pub use_header_filler: bool,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
    /// Vertical align of the inner content of the widget, if any. Possible values are "top", "center" or "bottom".
    pub vertical_align: Option<String>,
    /// Vertical space between individual cells.
    pub vertical_spacing: i32,
    /// Whether the GUI element can be squashed (by maximal height of some parent element) vertically. `nil` if this element does not support squashing. This is mainly meant to be used for scroll-pane The default (parent) value for scroll pane is true, false otherwise.
    pub vertically_squashable: Option<bool>,
    /// Whether the GUI element stretches its size vertically to other elements. `nil` if this element does not support stretching.
    pub vertically_stretchable: Option<bool>,
    /// Sets both minimal and maximal width to the given value.
    pub width: i32,
}

/// Style of a GUI element. All of the attributes listed here may be `nil` if not available for a particular GUI element.
pub trait LuaStyleMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// A "domain" of the world. Surfaces can only be created and deleted through the API. Surfaces are uniquely identified by their name. Every game contains at least the surface "nauvis".
pub struct LuaSurface {
    /// When set to true, the sun will always shine.
    pub always_day: bool,
    /// Defines how surface daytime brightness influences each color channel of the current color lookup table (LUT).
    ///
    /// The LUT is multiplied by `((1 - weight) + brightness * weight)` and result is clamped to range [0, 1].
    ///
    /// Default is `{0, 0, 0}`, which means no influence.
    pub brightness_visual_weights: ColorModifier,
    /// Amount of darkness at the current time, as a number in range [0, 1].
    pub darkness: f32,
    /// The daytime when dawn starts.
    pub dawn: f64,
    /// Current time of day, as a number in range [0, 1).
    pub daytime: f64,
    /// The daytime when dusk starts.
    pub dusk: f64,
    /// The daytime when evening starts.
    pub evening: f64,
    /// True if daytime is currently frozen.
    pub freeze_daytime: bool,
    /// When set to true, new chunks will be generated with lab tiles, instead of using the surface's map generation settings.
    pub generate_with_lab_tiles: bool,
    /// This surface's index in [LuaGameScript::surfaces](LuaGameScript::surfaces) (unique ID). It is assigned when a surface is created, and remains so until it is [deleted](on_surface_deleted). Indexes of deleted surfaces can be reused.
    pub index: u32,
    /// The generation settings for this surface. These can be modified after surface generation, but note that this will not retroactively update the surface. To manually regenerate it, [LuaSurface::regenerate_entity](LuaSurface::regenerate_entity), [LuaSurface::regenerate_decorative](LuaSurface::regenerate_decorative), and [LuaSurface::delete_chunk](LuaSurface::delete_chunk) can be used.
    pub map_gen_settings: MapGenSettings,
    /// The minimal brightness during the night. Defaults to `0.15`. This has an effect on both rendering and game mechanics such as biter spawns and solar power.
    pub min_brightness: f64,
    /// The daytime when morning starts.
    pub morning: f64,
    /// The name of this surface. Names are unique among surfaces.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// Is peaceful mode enabled on this surface?
    pub peaceful_mode: bool,
    /// If clouds are shown on this surface.
    pub show_clouds: bool,
    /// The multiplier of solar power on this surface. Cannot be less than 0.
    pub solar_power_multiplier: f64,
    /// The number of ticks per day for this surface.
    pub ticks_per_day: u32,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
    /// Current wind direction.
    pub wind_orientation: RealOrientation,
    /// Change in wind orientation per tick.
    pub wind_orientation_change: f64,
    /// Current wind speed in tiles per tick.
    pub wind_speed: f64,
}

pub enum LuaSurfaceMethodsCloneAreaDestinationForceUnion {
    LuaForce(LuaForce),
    String(String),
}

pub enum LuaSurfaceMethodsCloneBrushDestinationForceUnion {
    LuaForce(LuaForce),
    String(String),
}

pub enum LuaSurfaceMethodsCountEntitiesFilteredCollisionMaskUnion {
    CollisionMaskLayer(CollisionMaskLayer),
    Array(Vec<CollisionMaskLayer>),
}

pub enum LuaSurfaceMethodsCountEntitiesFilteredDirectionUnion {
    DefinesDirection(Direction),
    Array(Vec<Direction>),
}

pub enum LuaSurfaceMethodsCountEntitiesFilteredForceUnion {
    ForceIdentification(ForceIdentification),
    Array(Vec<ForceIdentification>),
}

pub enum LuaSurfaceMethodsCountEntitiesFilteredGhostNameUnion {
    String(String),
    Array(Vec<String>),
}

pub enum LuaSurfaceMethodsCountEntitiesFilteredGhostTypeUnion {
    String(String),
    Array(Vec<String>),
}

pub enum LuaSurfaceMethodsCountEntitiesFilteredNameUnion {
    String(String),
    Array(Vec<String>),
}

pub enum LuaSurfaceMethodsCountEntitiesFilteredTypUnion {
    String(String),
    Array(Vec<String>),
}

pub enum LuaSurfaceMethodsCountTilesFilteredCollisionMaskUnion {
    CollisionMaskLayer(CollisionMaskLayer),
    Array(Vec<CollisionMaskLayer>),
}

pub enum LuaSurfaceMethodsCountTilesFilteredForceUnion {
    ForceIdentification(ForceIdentification),
    Array(Vec<ForceIdentification>),
}

pub enum LuaSurfaceMethodsCountTilesFilteredNameUnion {
    String(String),
    Array(Vec<String>),
}

pub enum LuaSurfaceMethodsCreateEntitySourceUnion {
    LuaEntity(LuaEntity),
    MapPosition(MapPosition),
}

pub enum LuaSurfaceMethodsCreateEntityTargetUnion {
    LuaEntity(LuaEntity),
    MapPosition(MapPosition),
}

pub enum LuaSurfaceMethodsDestroyDecorativesCollisionMaskUnion {
    CollisionMaskLayer(CollisionMaskLayer),
    Array(Vec<CollisionMaskLayer>),
}

pub enum LuaSurfaceMethodsDestroyDecorativesNameUnion {
    String(String),
    ArrayString(Vec<String>),
    LuaDecorativePrototype(LuaDecorativePrototype),
    ArrayLuaDecorativePrototype(Vec<LuaDecorativePrototype>),
}

pub enum LuaSurfaceMethodsFindDecorativesFilteredCollisionMaskUnion {
    CollisionMaskLayer(CollisionMaskLayer),
    Array(Vec<CollisionMaskLayer>),
}

pub enum LuaSurfaceMethodsFindDecorativesFilteredNameUnion {
    String(String),
    ArrayString(Vec<String>),
    LuaDecorativePrototype(LuaDecorativePrototype),
    ArrayLuaDecorativePrototype(Vec<LuaDecorativePrototype>),
}

pub enum LuaSurfaceMethodsFindEnemyUnitsForceUnion {
    LuaForce(LuaForce),
    String(String),
}

pub enum LuaSurfaceMethodsFindEntitiesFilteredCollisionMaskUnion {
    CollisionMaskLayer(CollisionMaskLayer),
    Array(Vec<CollisionMaskLayer>),
}

pub enum LuaSurfaceMethodsFindEntitiesFilteredDirectionUnion {
    DefinesDirection(Direction),
    Array(Vec<Direction>),
}

pub enum LuaSurfaceMethodsFindEntitiesFilteredForceUnion {
    ForceIdentification(ForceIdentification),
    Array(Vec<ForceIdentification>),
}

pub enum LuaSurfaceMethodsFindEntitiesFilteredGhostNameUnion {
    String(String),
    Array(Vec<String>),
}

pub enum LuaSurfaceMethodsFindEntitiesFilteredGhostTypeUnion {
    String(String),
    Array(Vec<String>),
}

pub enum LuaSurfaceMethodsFindEntitiesFilteredNameUnion {
    String(String),
    Array(Vec<String>),
}

pub enum LuaSurfaceMethodsFindEntitiesFilteredTypUnion {
    String(String),
    Array(Vec<String>),
}

pub enum LuaSurfaceMethodsFindTilesFilteredCollisionMaskUnion {
    CollisionMaskLayer(CollisionMaskLayer),
    Array(Vec<CollisionMaskLayer>),
}

pub enum LuaSurfaceMethodsFindTilesFilteredForceUnion {
    ForceIdentification(ForceIdentification),
    Array(Vec<ForceIdentification>),
}

pub enum LuaSurfaceMethodsFindTilesFilteredNameUnion {
    String(String),
    Array(Vec<String>),
}

pub enum LuaSurfaceMethodsFindUnitsForceUnion {
    LuaForce(LuaForce),
    String(String),
}

pub enum LuaSurfaceMethodsGetEntitiesWithForceForceUnion {
    LuaForce(LuaForce),
    String(String),
}

pub enum LuaSurfaceMethodsGetScriptAreaKeyUnion {
    String(String),
    Uint(u32),
}

pub enum LuaSurfaceMethodsGetScriptPositionKeyUnion {
    String(String),
    Uint(u32),
}

pub enum LuaSurfaceMethodsGetTrainStopsNameUnion {
    String(String),
    Array(Vec<String>),
}

pub enum LuaSurfaceMethodsRegenerateDecorativeDecorativesUnion {
    String(String),
    Array(Vec<String>),
}

pub enum LuaSurfaceMethodsRegenerateEntityEntitiesUnion {
    String(String),
    Array(Vec<String>),
}

pub enum LuaSurfaceMethodsRequestPathCollisionMaskUnion {
    CollisionMaskWithFlags(CollisionMaskWithFlags),
    Array(Vec<String>),
}

pub enum LuaSurfaceMethodsSetHiddenTileTileUnion {
    String(String),
    LuaTilePrototype(LuaTilePrototype),
}

pub enum LuaSurfaceMethodsSetTilesRemoveCollidingEntitiesUnion {
    Boolean(bool),
    String(String),
}

pub enum LuaSurfaceMethodsSpillItemStackForceUnion {
    LuaForce(LuaForce),
    String(String),
}

/// A "domain" of the world. Surfaces can only be created and deleted through the API. Surfaces are uniquely identified by their name. Every game contains at least the surface "nauvis".
pub trait LuaSurfaceMethods {
    /// Adds the given script area.
    fn add_script_area(area: ScriptArea) -> u32;
    /// Adds the given script position.
    fn add_script_position(area: ScriptPosition) -> u32;
    /// Sets the given area to the checkerboard lab tiles.
    fn build_checkerboard(area: BoundingBox);
    /// Send a group to build a new base.
    fn build_enemy_base(force: ForceIdentification, position: MapPosition, unit_count: u32);
    fn calculate_tile_properties(positions: Vec<MapPosition>, property_names: Vec<String>) -> HashMap<String, Vec<f64>>;
    /// If there exists an entity at the given location that can be fast-replaced with the given entity parameters.
    fn can_fast_replace(direction: Direction, force: ForceIdentification, name: String, position: MapPosition) -> bool;
    /// Check for collisions with terrain or other entities.
    fn can_place_entity(build_check_type: BuildCheckType, direction: Direction, force: ForceIdentification, forced: bool, inner_name: String, name: String, position: MapPosition) -> bool;
    /// Cancel a deconstruction order.
    fn cancel_deconstruct_area(area: BoundingBox, force: ForceIdentification, item: LuaItemStack, player: PlayerIdentification, skip_fog_of_war: bool);
    /// Cancel a upgrade order.
    fn cancel_upgrade_area(area: BoundingBox, force: ForceIdentification, item: LuaItemStack, player: PlayerIdentification, skip_fog_of_war: bool);
    /// Clears this surface deleting all entities and chunks on it.
    fn clear(ignore_characters: bool);
    /// Clears all pollution on this surface.
    fn clear_pollution();
    /// Clones the given area.
    fn clone_area(clear_destination_decoratives: bool, clear_destination_entities: bool, clone_decoratives: bool, clone_entities: bool, clone_tiles: bool, create_build_effect_smoke: bool, destination_area: BoundingBox, destination_force: LuaSurfaceMethodsCloneAreaDestinationForceUnion, destination_surface: SurfaceIdentification, expand_map: bool, source_area: BoundingBox);
    /// Clones the given area.
    fn clone_brush(clear_destination_decoratives: bool, clear_destination_entities: bool, clone_decoratives: bool, clone_entities: bool, clone_tiles: bool, create_build_effect_smoke: bool, destination_force: LuaSurfaceMethodsCloneBrushDestinationForceUnion, destination_offset: TilePosition, destination_surface: SurfaceIdentification, expand_map: bool, manual_collision_mode: bool, source_offset: TilePosition, source_positions: Vec<TilePosition>);
    /// Clones the given entities.
    fn clone_entities(create_build_effect_smoke: bool, destination_force: ForceIdentification, destination_offset: Vector, destination_surface: SurfaceIdentification, entities: Vec<LuaEntity>, snap_to_grid: bool);
    /// Count entities of given type or name in a given area. Works just like [LuaSurface::find_entities_filtered](LuaSurface::find_entities_filtered), except this only returns the count. As it doesn't construct all the wrapper objects, this is more efficient if one is only interested in the number of entities.
    ///
    /// If no `area` or `position` are given, the entire surface is searched. If `position` is given, this returns the entities colliding with that position (i.e the given position is within the entity's collision box). If `position` and `radius` are given, this returns entities in the radius of the position. If `area` is specified, this returns entities colliding with that area.
    fn count_entities_filtered(area: BoundingBox, collision_mask: LuaSurfaceMethodsCountEntitiesFilteredCollisionMaskUnion, direction: LuaSurfaceMethodsCountEntitiesFilteredDirectionUnion, force: LuaSurfaceMethodsCountEntitiesFilteredForceUnion, ghost_name: LuaSurfaceMethodsCountEntitiesFilteredGhostNameUnion, ghost_type: LuaSurfaceMethodsCountEntitiesFilteredGhostTypeUnion, invert: bool, is_military_target: bool, limit: u32, name: LuaSurfaceMethodsCountEntitiesFilteredNameUnion, position: MapPosition, radius: f64, to_be_deconstructed: bool, to_be_upgraded: bool, typ: LuaSurfaceMethodsCountEntitiesFilteredTypUnion) -> u32;
    /// Count tiles of a given name in a given area. Works just like [LuaSurface::find_tiles_filtered](LuaSurface::find_tiles_filtered), except this only returns the count. As it doesn't construct all the wrapper objects, this is more efficient if one is only interested in the number of tiles.
    ///
    /// If no `area` or `position` and `radius` is given, the entire surface is searched. If `position` and `radius` are given, only tiles within the radius of the position are included.
    fn count_tiles_filtered(area: BoundingBox, collision_mask: LuaSurfaceMethodsCountTilesFilteredCollisionMaskUnion, force: LuaSurfaceMethodsCountTilesFilteredForceUnion, has_hidden_tile: bool, has_tile_ghost: bool, invert: bool, limit: u32, name: LuaSurfaceMethodsCountTilesFilteredNameUnion, position: MapPosition, radius: f64, to_be_deconstructed: bool) -> u32;
    /// Adds the given decoratives to the surface.
    fn create_decoratives(check_collision: bool, decoratives: Vec<Decorative>);
    /// Create an entity on this surface.
    fn create_entity(character: LuaEntity, create_build_effect_smoke: bool, direction: Direction, fast_replace: bool, force: ForceIdentification, item: LuaItemStack, move_stuck_players: bool, name: String, player: PlayerIdentification, position: MapPosition, raise_built: bool, source: LuaSurfaceMethodsCreateEntitySourceUnion, spawn_decorations: bool, spill: bool, target: LuaSurfaceMethodsCreateEntityTargetUnion) -> Option<LuaEntity>;
    /// Creates a particle at the given location
    fn create_particle(frame_speed: f32, height: f32, movement: Vector, name: String, position: MapPosition, vertical_speed: f32);
    fn create_trivial_smoke(name: String, position: MapPosition);
    /// Create a new unit group at a given position.
    fn create_unit_group(force: ForceIdentification, position: MapPosition) -> LuaUnitGroup;
    /// Place a deconstruction request.
    fn deconstruct_area(area: BoundingBox, force: ForceIdentification, item: LuaItemStack, player: PlayerIdentification, skip_fog_of_war: bool);
    /// Whether the given decorative prototype collides at the given position and direction.
    fn decorative_prototype_collides(position: MapPosition, prototype: String) -> bool;
    fn delete_chunk(position: ChunkPosition);
    /// Removes all decoratives from the given area. If no area and no position are given, then the entire surface is searched.
    fn destroy_decoratives(area: BoundingBox, collision_mask: LuaSurfaceMethodsDestroyDecorativesCollisionMaskUnion, exclude_soft: bool, from_layer: String, invert: bool, limit: u32, name: LuaSurfaceMethodsDestroyDecorativesNameUnion, position: TilePosition, to_layer: String);
    /// Sets the given script area to the new values.
    fn edit_script_area(area: ScriptArea, id: u32);
    /// Sets the given script position to the new values.
    fn edit_script_position(area: ScriptPosition, id: u32);
    /// Whether the given entity prototype collides at the given position and direction.
    fn entity_prototype_collides(direction: Direction, position: MapPosition, prototype: EntityPrototypeIdentification, use_map_generation_bounding_box: bool) -> bool;
    /// Find decoratives of a given name in a given area.
    ///
    /// If no filters are given, returns all decoratives in the search area. If multiple filters are specified, returns only decoratives matching every given filter. If no area and no position are given, the entire surface is searched.
    fn find_decoratives_filtered(area: BoundingBox, collision_mask: LuaSurfaceMethodsFindDecorativesFilteredCollisionMaskUnion, exclude_soft: bool, from_layer: String, invert: bool, limit: u32, name: LuaSurfaceMethodsFindDecorativesFilteredNameUnion, position: TilePosition, to_layer: String) -> Vec<DecorativeResult>;
    /// Find enemy units (entities with type "unit") of a given force within an area.
    fn find_enemy_units(center: MapPosition, force: LuaSurfaceMethodsFindEnemyUnitsForceUnion, radius: f64) -> Vec<LuaEntity>;
    /// Find entities in a given area.
    ///
    /// If no area is given all entities on the surface are returned.
    fn find_entities(area: BoundingBox) -> Vec<LuaEntity>;
    /// Find all entities of the given type or name in the given area.
    ///
    /// If no filters (`name`, `type`, `force`, etc.) are given, this returns all entities in the search area. If multiple filters are specified, only entities matching all given filters are returned.
    ///
    /// - If no `area` or `position` are given, the entire surface is searched.
    /// - If `position` is given, this returns the entities colliding with that position (i.e the given position is within the entity's collision box).
    /// - If `position` and `radius` are given, this returns the entities within the radius of the position. Looks for the center of entities.
    /// - If `area` is specified, this returns the entities colliding with that area.
    fn find_entities_filtered(area: BoundingBox, collision_mask: LuaSurfaceMethodsFindEntitiesFilteredCollisionMaskUnion, direction: LuaSurfaceMethodsFindEntitiesFilteredDirectionUnion, force: LuaSurfaceMethodsFindEntitiesFilteredForceUnion, ghost_name: LuaSurfaceMethodsFindEntitiesFilteredGhostNameUnion, ghost_type: LuaSurfaceMethodsFindEntitiesFilteredGhostTypeUnion, has_item_inside: LuaItemPrototype, invert: bool, is_military_target: bool, limit: u32, name: LuaSurfaceMethodsFindEntitiesFilteredNameUnion, position: MapPosition, radius: f64, to_be_deconstructed: bool, to_be_upgraded: bool, typ: LuaSurfaceMethodsFindEntitiesFilteredTypUnion) -> Vec<LuaEntity>;
    /// Find a specific entity at a specific position.
    fn find_entity(entity: String, position: MapPosition) -> Option<LuaEntity>;
    /// Find the logistic network that covers a given position.
    fn find_logistic_network_by_position(force: ForceIdentification, position: MapPosition) -> Option<LuaLogisticNetwork>;
    /// Finds all of the logistics networks whose construction area intersects with the given position.
    fn find_logistic_networks_by_construction_area(force: ForceIdentification, position: MapPosition) -> Vec<LuaLogisticNetwork>;
    /// Find the enemy military target ([military entity](https://wiki.factorio.com/Military_units_and_structures)) closest to the given position.
    fn find_nearest_enemy(force: ForceIdentification, max_distance: f64, position: MapPosition) -> Option<LuaEntity>;
    /// Find the enemy entity-with-owner closest to the given position.
    fn find_nearest_enemy_entity_with_owner(force: ForceIdentification, max_distance: f64, position: MapPosition) -> LuaEntity;
    /// Find a non-colliding position within a given radius.
    fn find_non_colliding_position(center: MapPosition, force_to_tile_center: bool, name: String, precision: f64, radius: f64) -> Option<MapPosition>;
    /// Find a non-colliding position within a given rectangle.
    fn find_non_colliding_position_in_box(force_to_tile_center: bool, name: String, precision: f64, search_space: BoundingBox) -> Option<MapPosition>;
    /// Find all tiles of the given name in the given area.
    ///
    /// If no filters are given, this returns all tiles in the search area.
    ///
    /// If no `area` or `position` and `radius` is given, the entire surface is searched. If `position` and `radius` are given, only tiles within the radius of the position are included.
    fn find_tiles_filtered(area: BoundingBox, collision_mask: LuaSurfaceMethodsFindTilesFilteredCollisionMaskUnion, force: LuaSurfaceMethodsFindTilesFilteredForceUnion, has_hidden_tile: bool, has_tile_ghost: bool, invert: bool, limit: u32, name: LuaSurfaceMethodsFindTilesFilteredNameUnion, position: MapPosition, radius: f64, to_be_deconstructed: bool) -> Vec<LuaTile>;
    /// Find units (entities with type "unit") of a given force and force condition within a given area.
    fn find_units(area: BoundingBox, condition: ForceCondition, force: LuaSurfaceMethodsFindUnitsForceUnion) -> Vec<LuaEntity>;
    /// Blocks and generates all chunks that have been requested using all available threads.
    fn force_generate_chunk_requests();
    /// Get an iterator going over every chunk on this surface.
    fn get_chunks() -> LuaChunkIterator;
    /// Gets the closest entity in the list to this position.
    fn get_closest(entities: Vec<LuaEntity>, position: MapPosition) -> Option<LuaEntity>;
    /// Gets all tiles of the given types that are connected horizontally or vertically to the given tile position including the given tile position.
    fn get_connected_tiles(position: TilePosition, tiles: Vec<String>) -> Vec<TilePosition>;
    /// Returns all the military targets (entities with force) on this chunk for the given force.
    fn get_entities_with_force(force: LuaSurfaceMethodsGetEntitiesWithForceForceUnion, position: ChunkPosition) -> Vec<LuaEntity>;
    /// The hidden tile name.
    fn get_hidden_tile(position: TilePosition) -> Option<String>;
    /// Gets the map exchange string for the current map generation settings of this surface.
    fn get_map_exchange_string() -> String;
    /// Get the pollution for a given position.
    fn get_pollution(position: MapPosition) -> f64;
    /// Gets a random generated chunk position or 0,0 if no chunks have been generated on this surface.
    fn get_random_chunk() -> ChunkPosition;
    /// Gets the resource amount of all resources on this surface
    fn get_resource_counts() -> HashMap<String, u32>;
    /// Gets the first script area by name or id.
    fn get_script_area(key: LuaSurfaceMethodsGetScriptAreaKeyUnion) -> Option<ScriptArea>;
    /// Gets the script areas that match the given name or if no name is given all areas are returned.
    fn get_script_areas(name: String) -> Vec<ScriptArea>;
    /// Gets the first script position by name or id.
    fn get_script_position(key: LuaSurfaceMethodsGetScriptPositionKeyUnion) -> Option<ScriptPosition>;
    /// Gets the script positions that match the given name or if no name is given all positions are returned.
    fn get_script_positions(name: String) -> Vec<ScriptPosition>;
    /// Gets the starting area radius of this surface.
    fn get_starting_area_radius() -> f64;
    /// Get the tile at a given position. An alternative call signature for this method is passing it a single [TilePosition](TilePosition).
    fn get_tile(x: i32, y: i32) -> LuaTile;
    /// Gets the total amount of pollution on the surface by iterating over all of the chunks containing pollution.
    fn get_total_pollution() -> f64;
    /// Gets train stops matching the given filters.
    fn get_train_stops(force: ForceIdentification, name: LuaSurfaceMethodsGetTrainStopsNameUnion) -> Vec<LuaEntity>;
    fn get_trains(force: ForceIdentification) -> Vec<LuaTrain>;
    /// All methods and properties that this object supports.
    fn help() -> String;
    /// Is a given chunk generated?
    fn is_chunk_generated(position: ChunkPosition) -> bool;
    /// Play a sound for every player on this surface.
    fn play_sound(override_sound_type: SoundType, path: SoundPath, position: MapPosition, volume_modifier: f64);
    /// Spawn pollution at the given position.
    fn pollute(amount: f64, source: MapPosition);
    /// Print text to the chat console of all players on this surface.
    fn print(color: Color, message: LocalisedString);
    /// Regenerate autoplacement of some decoratives on this surface. This can be used to autoplace newly-added decoratives.
    fn regenerate_decorative(chunks: Vec<ChunkPosition>, decoratives: LuaSurfaceMethodsRegenerateDecorativeDecorativesUnion);
    /// Regenerate autoplacement of some entities on this surface. This can be used to autoplace newly-added entities.
    fn regenerate_entity(chunks: Vec<ChunkPosition>, entities: LuaSurfaceMethodsRegenerateEntityEntitiesUnion);
    /// Removes the given script area.
    fn remove_script_area(id: u32) -> bool;
    /// Removes the given script position.
    fn remove_script_position(id: u32) -> bool;
    /// Generates a path with the specified constraints (as an array of [PathfinderWaypoints](PathfinderWaypoint)) using the unit pathfinding algorithm. This path can be used to emulate pathing behavior by script for non-unit entities, such as vehicles. If you want to command actual units (such as biters or spitters) to move, use [LuaEntity::set_command](LuaEntity::set_command) instead.
    ///
    /// The resulting path is ultimately returned asynchronously via [on_script_path_request_finished](on_script_path_request_finished).
    fn request_path(bounding_box: BoundingBox, can_open_gates: bool, collision_mask: LuaSurfaceMethodsRequestPathCollisionMaskUnion, entity_to_ignore: LuaEntity, force: ForceIdentification, goal: MapPosition, path_resolution_modifier: i32, pathfind_flags: PathfinderFlags, radius: f64, start: MapPosition) -> u32;
    /// Request that the game's map generator generate chunks at the given position for the given radius on this surface.
    fn request_to_generate_chunks(position: MapPosition, radius: u32);
    /// Set generated status of a chunk. Useful when copying chunks.
    fn set_chunk_generated_status(position: ChunkPosition, status: ChunkGeneratedStatus);
    /// Set the hidden tile for the specified position. While during normal gameplay only [non-mineable](LuaTilePrototype::mineable_properties) tiles can become hidden, this method allows any kind of tile to be set as the hidden one.
    fn set_hidden_tile(position: TilePosition, tile: LuaSurfaceMethodsSetHiddenTileTileUnion);
    /// Give a command to multiple units. This will automatically select suitable units for the task.
    fn set_multi_command(command: Command, force: ForceIdentification, unit_count: u32, unit_search_distance: u32) -> u32;
    /// Set tiles at specified locations. Can automatically correct the edges around modified tiles.
    ///
    /// Placing a [mineable](LuaTilePrototype::mineable_properties) tile on top of a non-mineable one will turn the latter into the [LuaTile::hidden_tile](LuaTile::hidden_tile) for that tile. Placing a mineable tile on a mineable one or a non-mineable tile on a non-mineable one will not modify the hidden tile. This restriction can however be circumvented by using [LuaSurface::set_hidden_tile](LuaSurface::set_hidden_tile).
    fn set_tiles(correct_tiles: bool, raise_event: bool, remove_colliding_decoratives: bool, remove_colliding_entities: LuaSurfaceMethodsSetTilesRemoveCollidingEntitiesUnion, tiles: Vec<Tile>);
    /// Spill items on the ground centered at a given location.
    fn spill_item_stack(allow_belts: bool, enable_looted: bool, force: LuaSurfaceMethodsSpillItemStackForceUnion, items: ItemStackIdentification, position: MapPosition) -> Vec<LuaEntity>;
    /// Place an upgrade request.
    fn upgrade_area(area: BoundingBox, force: ForceIdentification, item: LuaItemStack, player: PlayerIdentification, skip_fog_of_war: bool);
}

/// One research item.
pub struct LuaTechnology {
    /// Effects applied when this technology is researched.
    pub effects: Vec<TechnologyModifier>,
    /// Can this technology be researched?
    pub enabled: bool,
    /// The force this technology belongs to.
    pub force: Box<LuaForce>,
    /// The current level of this technology. For level-based technology writing to this is the same as researching the technology to the previous level. Writing the level will set [LuaTechnology::enabled](LuaTechnology::enabled) to `true`.
    pub level: u32,
    pub localised_description: LocalisedString,
    /// Localised name of this technology.
    pub localised_name: LocalisedString,
    /// Name of this technology.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    /// Prerequisites of this technology. The result maps technology name to the [LuaTechnology](LuaTechnology) object.
    pub prerequisites: HashMap<String, LuaTechnology>,
    /// The prototype of this technology.
    pub prototype: LuaTechnologyPrototype,
    /// The number of research units required for this technology.
    pub research_unit_count: u32,
    /// The count formula, if this research has any. See the [wiki](https://wiki.factorio.com/Prototype/Technology#Technology_data) for details.
    pub research_unit_count_formula: Option<String>,
    /// Amount of energy required to finish a unit of research.
    pub research_unit_energy: f64,
    /// The types of ingredients that labs will require to research this technology.
    pub research_unit_ingredients: Vec<Ingredient>,
    /// Has this technology been researched? Switching from `false` to `true` will trigger the technology advancement perks; switching from `true` to `false` will reverse them.
    pub researched: bool,
    /// Is this an upgrade-type research?
    pub upgrade: bool,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
    /// If this technology will be visible in the research GUI even though it is disabled.
    pub visible_when_disabled: bool,
}

/// One research item.
pub trait LuaTechnologyMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
    /// Reload this technology from its prototype.
    fn reload();
}

/// A Technology prototype.
pub struct LuaTechnologyPrototype {
    /// Effects applied when this technology is researched.
    pub effects: Vec<TechnologyModifier>,
    /// If this technology prototype is enabled by default (enabled at the beginning of a game).
    pub enabled: bool,
    /// If this technology prototype is hidden.
    pub hidden: bool,
    /// If this technology ignores the technology cost multiplier setting.
    pub ignore_tech_cost_multiplier: bool,
    /// The level of this research.
    pub level: u32,
    pub localised_description: LocalisedString,
    /// Localised name of this technology.
    pub localised_name: LocalisedString,
    /// The max level of this research.
    pub max_level: u32,
    /// Name of this technology.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    /// Prerequisites of this technology. The result maps technology name to the [LuaTechnologyPrototype](LuaTechnologyPrototype) object.
    pub prerequisites: HashMap<String, LuaTechnologyPrototype>,
    /// The number of research units required for this technology.
    pub research_unit_count: u32,
    /// The count formula, if this research has any. See the [wiki](https://wiki.factorio.com/Prototype/Technology#Technology_data) for details.
    pub research_unit_count_formula: Option<String>,
    /// Amount of energy required to finish a unit of research.
    pub research_unit_energy: f64,
    /// The types of ingredients that labs will require to research this technology.
    pub research_unit_ingredients: Vec<Ingredient>,
    /// If the is technology prototype is an upgrade to some other technology.
    pub upgrade: bool,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
    /// If this technology will be visible in the research GUI even though it is disabled.
    pub visible_when_disabled: bool,
}

/// A Technology prototype.
pub trait LuaTechnologyPrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// A single "square" on the map.
pub struct LuaTile {
    /// The name of the [LuaTilePrototype](LuaTilePrototype) hidden under this tile, if any. During normal gameplay, only [non-mineable](LuaTilePrototype::mineable_properties) tiles can become hidden. This can however be circumvented with [LuaSurface::set_hidden_tile](LuaSurface::set_hidden_tile).
    pub hidden_tile: Option<String>,
    /// Prototype name of this tile. E.g. `"sand-3"` or `"grass-2"`.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The position this tile references.
    pub position: TilePosition,
    pub prototype: LuaTilePrototype,
    /// The surface this tile is on.
    pub surface: LuaSurface,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// A single "square" on the map.
pub trait LuaTileMethods {
    /// Cancels deconstruction if it is scheduled, does nothing otherwise.
    fn cancel_deconstruction(force: ForceIdentification, player: PlayerIdentification);
    /// What type of things can collide with this tile?
    fn collides_with(layer: CollisionMaskLayer) -> bool;
    /// Gets all tile ghosts on this tile.
    fn get_tile_ghosts(force: ForceIdentification) -> Vec<LuaTile>;
    /// Does this tile have any tile ghosts on it.
    fn has_tile_ghost(force: ForceIdentification) -> bool;
    /// All methods and properties that this object supports.
    fn help() -> String;
    /// Orders deconstruction of this tile by the given force.
    fn order_deconstruction(force: ForceIdentification, player: PlayerIdentification) -> Option<LuaEntity>;
    /// Is this tile marked for deconstruction?
    fn to_be_deconstructed(force: ForceIdentification) -> bool;
}

pub struct LuaTilePrototypeMineableProperties {
    /// Is this tile mineable at all?
    pub minable: bool,
    /// Prototype name of the particle produced when mining this tile. Will only be present if this tile produces any particle during mining.
    pub mining_particle: Option<String>,
    /// Energy required to mine a tile.
    pub mining_time: f64,
    /// Products obtained by mining this tile.
    pub products: Vec<Product>,
}

/// Prototype of a tile.
pub struct LuaTilePrototype {
    pub allowed_neighbors: HashMap<String, LuaTilePrototype>,
    pub automatic_neighbors: bool,
    /// Autoplace specification for this prototype, if any.
    pub autoplace_specification: Option<AutoplaceSpecification>,
    /// False if this tile is not allowed in blueprints regardless of the ability to build it.
    pub can_be_part_of_blueprint: bool,
    /// True if building this tile should check for colliding entities above and prevent building if such are found. Also during mining tiles above this tile checks for entities colliding with this tile and prevents mining if such are found.
    pub check_collision_with_entities: bool,
    /// The collision mask this tile uses
    pub collision_mask: CollisionMask,
    pub collision_mask_with_flags: CollisionMaskWithFlags,
    /// The probability that decorative entities will be removed from on top of this tile when this tile is generated.
    pub decorative_removal_probability: f32,
    /// Amount of pollution emissions per second this tile will absorb.
    pub emissions_per_second: f64,
    /// Items that when placed will produce this tile, if any. Construction bots will choose the first item in the list to build this tile.
    pub items_to_place_this: Option<Vec<ItemStackDefinition>>,
    pub layer: u32,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub map_color: Color,
    pub mineable_properties: LuaTilePrototypeMineableProperties,
    /// Name of this prototype.
    pub name: String,
    /// If this tile needs correction logic applied when it's generated in the world..
    pub needs_correction: bool,
    /// The next direction of this tile, if any. Used when a tile has multiple directions (such as hazard concrete)
    pub next_direction: Option<Box<LuaTilePrototype>>,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
    pub vehicle_friction_modifier: f32,
    pub walking_speed_modifier: f32,
}

/// Prototype of a tile.
pub trait LuaTilePrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// A train. Trains are a sequence of connected rolling stocks -- locomotives and wagons.
pub struct LuaTrain {
    /// The rail at the back end of the train, if any.
    pub back_rail: Option<LuaEntity>,
    /// The back stock of this train, if any. The back of the train is at the opposite end of the [front](LuaTrain::front_stock).
    pub back_stock: Option<LuaEntity>,
    /// The cargo carriages the train contains.
    pub cargo_wagons: Vec<LuaEntity>,
    /// The rolling stocks this train is composed of, with the numbering starting at the [front](LuaTrain::front_stock) of the train.
    pub carriages: Vec<LuaEntity>,
    /// The fluid carriages the train contains.
    pub fluid_wagons: Vec<LuaEntity>,
    /// The rail at the front end of the train, if any.
    pub front_rail: Option<LuaEntity>,
    /// The front stock of this train, if any. The front of the train is in the direction that a majority of locomotives are pointing in. If it's a tie, the North and West directions take precedence.
    pub front_stock: Option<LuaEntity>,
    /// If this train has a path.
    pub has_path: bool,
    /// The unique train ID.
    pub id: u32,
    /// The total number of kills by this train.
    pub kill_count: u32,
    /// The players killed by this train.
    ///
    /// The keys are the player indices, the values are how often this train killed that player.
    pub killed_players: HashMap<u32, u32>,
    /// Arrays of locomotives. The result is two arrays, indexed by `"front_movers"` and `"back_movers"` containing the locomotives. E.g. `{front_movers={loco1, loco2}, back_movers={loco3}}`.
    pub locomotives: HashMap<String, Vec<LuaEntity>>,
    /// When `true`, the train is explicitly controlled by the player or script. When `false`, the train moves autonomously according to its schedule.
    pub manual_mode: bool,
    /// Current max speed when moving backwards, depends on locomotive prototype and fuel.
    pub max_backward_speed: f64,
    /// Current max speed when moving forward, depends on locomotive prototype and fuel.
    pub max_forward_speed: f64,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The player passengers on the train
    pub passengers: Vec<LuaPlayer>,
    /// The path this train is using, if any.
    pub path: Option<LuaRailPath>,
    /// The destination rail this train is currently pathing to, if any.
    pub path_end_rail: Option<LuaEntity>,
    /// The destination train stop this train is currently pathing to, if any.
    pub path_end_stop: Option<LuaEntity>,
    pub rail_direction_from_back_rail: RailDirection,
    pub rail_direction_from_front_rail: RailDirection,
    /// The riding state of this train.
    pub riding_state: RidingState,
    /// This train's current schedule, if any. Set to `nil` to clear.
    pub schedule: Option<TrainSchedule>,
    /// The signal this train is arriving or waiting at, if any.
    pub signal: Option<LuaEntity>,
    /// Current speed.
    pub speed: f64,
    /// This train's current state.
    pub state: TrainState,
    /// The train stop this train is stopped at, if any.
    pub station: Option<LuaEntity>,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
    /// The weight of this train.
    pub weight: f64,
}

/// A train. Trains are a sequence of connected rolling stocks -- locomotives and wagons.
pub trait LuaTrainMethods {
    /// Clears all fluids in this train.
    fn clear_fluids_inside();
    /// Clear all items in this train.
    fn clear_items_inside();
    /// Get a mapping of the train's inventory.
    fn get_contents() -> HashMap<String, u32>;
    /// Gets a mapping of the train's fluid inventory.
    fn get_fluid_contents() -> HashMap<String, f64>;
    /// Get the amount of a particular fluid stored in the train.
    fn get_fluid_count(fluid: String) -> f64;
    /// Get the amount of a particular item stored in the train.
    fn get_item_count(item: String) -> u32;
    /// Gets all rails under the train.
    fn get_rails() -> Vec<LuaEntity>;
    /// Go to the station specified by the index in the train's schedule.
    fn go_to_station(index: u32);
    /// All methods and properties that this object supports.
    fn help() -> String;
    /// Insert a stack into the train.
    fn insert(stack: ItemStackIdentification);
    /// Inserts the given fluid into the first available location in this train.
    fn insert_fluid(fluid: Fluid) -> f64;
    /// Checks if the path is invalid and tries to re-path if it isn't.
    fn recalculate_path(force: bool) -> bool;
    /// Remove some fluid from the train.
    fn remove_fluid(fluid: Fluid) -> f64;
    /// Remove some items from the train.
    fn remove_item(stack: ItemStackIdentification) -> u32;
}

/// Control behavior for train stops.
pub struct LuaTrainStopControlBehavior {
    pub lua_generic_on_off_control_behavior: Box<LuaGenericOnOffControlBehavior>,
    /// `true` if the train stop is enabled/disabled through the circuit network.
    pub enable_disable: bool,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// `true` if the train stop should send the train contents to the circuit network.
    pub read_from_train: bool,
    /// `true` if the train stop should send the stopped train id to the circuit network.
    pub read_stopped_train: bool,
    /// `true` if the train stop should send amount of incoming trains to the circuit network.
    pub read_trains_count: bool,
    /// `true` if the train stop should send the circuit network contents to the train to use.
    pub send_to_train: bool,
    /// `true` if the trains_limit_signal is used to set a limit of trains incoming for train stop.
    pub set_trains_limit: bool,
    /// The signal that will be sent when using the send-train-id option.
    pub stopped_train_signal: SignalID,
    /// The signal that will be sent when using the read-trains-count option.
    pub trains_count_signal: SignalID,
    /// The signal to be used by set-trains-limit to limit amount of incoming trains
    pub trains_limit_signal: SignalID,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Control behavior for train stops.
pub trait LuaTrainStopControlBehaviorMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Control behavior for transport belts.
pub struct LuaTransportBeltControlBehavior {
    pub lua_generic_on_off_control_behavior: Box<LuaGenericOnOffControlBehavior>,
    /// If the belt will be enabled/disabled based off the circuit network.
    pub enable_disable: bool,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// If the belt will read the contents and send them to the circuit network.
    pub read_contents: bool,
    /// The read mode for the belt.
    pub read_contents_mode: ControlBehaviorTransportBeltContentReadMode,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Control behavior for transport belts.
pub trait LuaTransportBeltControlBehaviorMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// One line on a transport belt.
pub struct LuaTransportLine {
    /// The transport lines that this transport line is fed by or an empty table if none.
    pub input_lines: Vec<LuaTransportLine>,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The transport lines that this transport line outputs items to or an empty table if none.
    pub output_lines: Vec<LuaTransportLine>,
    /// The entity this transport line belongs to.
    pub owner: LuaEntity,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// One line on a transport belt.
pub trait LuaTransportLineMethods {
    /// Can an item be inserted at a given position?
    fn can_insert_at(position: f32) -> bool;
    /// Can an item be inserted at the back of this line?
    fn can_insert_at_back() -> bool;
    /// Remove all items from this transport line.
    fn clear();
    /// Get counts of all items on this line, similar to how [LuaInventory::get_contents](LuaInventory::get_contents) does.
    fn get_contents() -> HashMap<String, u32>;
    /// Count some or all items on this line, similar to how [LuaInventory::get_item_count](LuaInventory::get_item_count) does.
    fn get_item_count(item: String) -> u32;
    /// All methods and properties that this object supports.
    fn help() -> String;
    /// Insert items at a given position.
    fn insert_at(items: ItemStackIdentification, position: f32) -> bool;
    /// Insert items at the back of this line.
    fn insert_at_back(items: ItemStackIdentification) -> bool;
    /// Returns whether the associated internal transport line of this line is the same as the others associated internal transport line.
    fn line_equals(other: LuaTransportLine) -> bool;
    /// Remove some items from this line.
    fn remove_item(items: ItemStackIdentification) -> u32;
}

/// Prototype of a trivial smoke.
pub struct LuaTrivialSmokePrototype {
    pub affected_by_wind: bool,
    pub color: Color,
    pub cyclic: bool,
    pub duration: u32,
    pub end_scale: f64,
    pub fade_away_duration: u32,
    pub fade_in_duration: u32,
    pub glow_animation: bool,
    pub glow_fade_away_duration: u32,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub movement_slow_down_factor: f64,
    /// Name of this prototype.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    pub render_layer: RenderLayer,
    pub show_when_smoke_off: bool,
    pub spread_duration: u32,
    pub start_scale: f64,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of a trivial smoke.
pub trait LuaTrivialSmokePrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// A collection of units moving and attacking together. The engine creates autonomous unit groups to attack polluted areas. The script can create and control such groups as well. Groups can accept commands in the same manner as regular units.
pub struct LuaUnitGroup {
    /// The command given to this group, if any.
    pub command: Option<Box<Command>>,
    /// The distraction command given to this group, if any.
    pub distraction_command: Option<Box<Command>>,
    /// The force of this unit group.
    pub force: LuaForce,
    /// The group number for this unit group.
    pub group_number: u32,
    /// Whether this unit group is controlled by a script or by the game engine. This can be changed using [LuaUnitGroup::set_autonomous](LuaUnitGroup::set_autonomous).
    pub is_script_driven: bool,
    /// Members of this group.
    pub members: Vec<LuaEntity>,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// Group position. This can have different meanings depending on the group state. When the group is gathering, the position is the place of gathering. When the group is moving, the position is the expected position of its members along the path. When the group is attacking, it is the average position of its members.
    pub position: MapPosition,
    /// Whether this group is gathering, moving or attacking.
    pub state: GroupState,
    /// The surface of this unit group.
    pub surface: LuaSurface,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// A collection of units moving and attacking together. The engine creates autonomous unit groups to attack polluted areas. The script can create and control such groups as well. Groups can accept commands in the same manner as regular units.
pub trait LuaUnitGroupMethods {
    /// Make a unit a member of this group. Has the same effect as giving a `group_command` with this group to the unit.
    fn add_member(unit: LuaEntity);
    /// Dissolve this group. Its members won't be destroyed, they will be merely unlinked from this group.
    fn destroy();
    /// All methods and properties that this object supports.
    fn help() -> String;
    /// Make this group autonomous. Autonomous groups will automatically attack polluted areas. Autonomous groups aren't considered to be [script-driven](LuaUnitGroup::is_script_driven).
    fn set_autonomous();
    /// Give this group a command.
    fn set_command(command: Command);
    /// Give this group a distraction command.
    fn set_distraction_command(command: Command);
    /// Make the group start moving even if some of its members haven't yet arrived.
    fn start_moving();
}

/// Prototype of a virtual signal.
pub struct LuaVirtualSignalPrototype {
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    /// Name of this prototype.
    pub name: String,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    /// The string used to alphabetically sort these prototypes. It is a simple string that has no additional semantic meaning.
    pub order: String,
    /// Whether this is a special signal. The `everything`, `anything`, `each`, and `unknown` signals are considered special.
    pub special: bool,
    pub subgroup: LuaGroup,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of a virtual signal.
pub trait LuaVirtualSignalPrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Prototype of a void energy source.
pub struct LuaVoidEnergySourcePrototype {
    /// The emissions of this energy source in `pollution/Joule`. Multiplying it by energy consumption in `Watt` gives `pollution/second`.
    pub emissions: f64,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    pub render_no_network_icon: bool,
    pub render_no_power_icon: bool,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Prototype of a void energy source.
pub trait LuaVoidEnergySourcePrototypeMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}

/// Control behavior for walls.
pub struct LuaWallControlBehavior {
    pub lua_control_behavior: Box<LuaControlBehavior>,
    /// The circuit condition.
    pub circuit_condition: CircuitConditionDefinition,
    /// The class name of this object. Available even when `valid` is false. For LuaStruct objects it may also be suffixed with a dotted path to a member of the struct.
    pub object_name: String,
    pub open_gate: bool,
    pub output_signal: SignalID,
    pub read_sensor: bool,
    /// Is this object valid? This Lua object holds a reference to an object within the game engine. It is possible that the game-engine object is removed whilst a mod still holds the corresponding Lua object. If that happens, the object becomes invalid, i.e. this attribute will be `false`. Mods are advised to check for object validity if any change to the game state might have occurred between the creation of the Lua object and its access.
    pub valid: bool,
}

/// Control behavior for walls.
pub trait LuaWallControlBehaviorMethods {
    /// All methods and properties that this object supports.
    fn help() -> String;
}
