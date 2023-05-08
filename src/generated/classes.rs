use std::collections::HashMap;
use std::collections::HashSet;

use super::concepts::*;
use super::defines::*;

/// Collection of settings for overriding default ai behavior.
pub struct LuaAISettings {
    pub allow_destroy_when_commands_fail: bool,
    pub allow_try_return_to_spawner: bool,
    pub do_separation: bool,
    pub object_name: String,
    pub path_resolution_modifier: i8,
    pub valid: bool,
}

/// Control behavior for accumulators.
pub struct LuaAccumulatorControlBehavior {
    pub lua_control_behavior: Box<LuaControlBehavior>,
    pub object_name: String,
    pub output_signal: SignalID,
    pub valid: bool,
}

/// Prototype of a achievement.
pub struct LuaAchievementPrototype {
    pub allowed_without_fight: bool,
    pub hidden: bool,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub valid: bool,
}

/// Prototype of a ammo category.
pub struct LuaAmmoCategoryPrototype {
    pub bonus_gui_order: String,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub valid: bool,
}

/// Control behavior for arithmetic combinators.
pub struct LuaArithmeticCombinatorControlBehavior {
    pub lua_combinator_control_behavior: Box<LuaCombinatorControlBehavior>,
    pub object_name: String,
    pub parameters: ArithmeticCombinatorParameters,
    pub valid: bool,
}

/// Prototype of an autoplace control.
pub struct LuaAutoplaceControlPrototype {
    pub can_be_disabled: bool,
    pub category: String,
    pub control_order: String,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub richness: bool,
    pub valid: bool,
}

pub struct LuaBootstrapLevel {
    pub campaign_name: Option<String>,
    pub is_simulation: Option<bool>,
    pub is_tutorial: Option<bool>,
    pub level_name: String,
    pub mod_name: Option<String>,
}

/// Entry point for registering event handlers. It is accessible through the global object named `script`.
pub struct LuaBootstrap {
    pub active_mods: HashMap<String, String>,
    pub level: LuaBootstrapLevel,
    pub mod_name: String,
    pub object_name: String,
}

pub enum LuaBurnerOwnerUnion {
    LuaEntity(LuaEntity),
    LuaEquipment(LuaEquipment),
}

/// A reference to the burner energy source owned by a specific [LuaEntity](LuaEntity) or [LuaEquipment](LuaEquipment).
pub struct LuaBurner {
    pub burnt_result_inventory: LuaInventory,
    pub currently_burning: Option<LuaItemPrototype>,
    pub fuel_categories: HashMap<String, bool>,
    pub heat: f64,
    pub heat_capacity: f64,
    pub inventory: LuaInventory,
    pub object_name: String,
    pub owner: LuaBurnerOwnerUnion,
    pub remaining_burning_fuel: f64,
    pub valid: bool,
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
    pub emissions: f64,
    pub fuel_categories: HashMap<String, bool>,
    pub fuel_inventory_size: u32,
    pub light_flicker: Option<LuaBurnerPrototypeLightFlicker>,
    pub object_name: String,
    pub render_no_network_icon: bool,
    pub render_no_power_icon: bool,
    pub smoke: Option<Vec<SmokeSource>>,
    pub valid: bool,
}

/// A chunk iterator can be used for iterating chunks coordinates of a surface.
/// 
/// The returned type is a [ChunkPositionAndArea](ChunkPositionAndArea) containing the chunk coordinates and its area.
pub struct LuaChunkIterator {
    pub object_name: String,
    pub valid: bool,
}

/// A circuit network associated with a given entity, connector, and wire type.
pub struct LuaCircuitNetwork {
    pub circuit_connector_id: CircuitConnectorId,
    pub connected_circuit_count: u32,
    pub entity: LuaEntity,
    pub network_id: u32,
    pub object_name: String,
    pub signals: Option<Vec<Signal>>,
    pub valid: bool,
    pub wire_type: WireType,
}

pub struct LuaCombinatorControlBehavior {
    pub lua_control_behavior: Box<LuaControlBehavior>,
    pub signals_last_tick: Vec<Signal>,
}

/// Allows for the registration of custom console commands through the global object named `commands`. Similarly to [event subscriptions](LuaBootstrap::on_event), these don't persist through a save-and-load cycle.
pub struct LuaCommandProcessor {
    pub commands: HashMap<String, LocalisedString>,
    pub game_commands: HashMap<String, LocalisedString>,
    pub object_name: String,
}

/// Control behavior for constant combinators.
pub struct LuaConstantCombinatorControlBehavior {
    pub lua_control_behavior: Box<LuaControlBehavior>,
    pub enabled: bool,
    pub object_name: String,
    pub parameters: Option<Vec<ConstantCombinatorParameters>>,
    pub signals_count: u32,
    pub valid: bool,
}

/// Control behavior for container entities.
pub struct LuaContainerControlBehavior {
    pub lua_control_behavior: Box<LuaControlBehavior>,
    pub object_name: String,
    pub valid: bool,
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
    pub mining: bool,
    pub position: Option<MapPosition>,
}

pub struct LuaControlRepairState {
    pub position: MapPosition,
    pub repairing: bool,
}

pub struct LuaControlShootingState {
    pub position: MapPosition,
    pub state: Shooting,
}

pub struct LuaControlWalkingState {
    pub direction: Direction,
    pub walking: bool,
}

/// This is an abstract base class containing the common functionality between [LuaPlayer](LuaPlayer) and entities (see [LuaEntity](LuaEntity)). When accessing player-related functions through a [LuaEntity](LuaEntity), it must refer to a character entity.
pub struct LuaControl {
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
    pub character_mining_progress: f64,
    pub character_mining_speed_modifier: f64,
    pub character_personal_logistic_requests_enabled: bool,
    pub character_reach_distance_bonus: u32,
    pub character_resource_reach_distance_bonus: u32,
    pub character_running_speed: f64,
    pub character_running_speed_modifier: f64,
    pub character_trash_slot_count_bonus: u32,
    pub cheat_mode: bool,
    pub crafting_queue: Vec<CraftingQueueItem>,
    pub crafting_queue_progress: f64,
    pub crafting_queue_size: u32,
    pub cursor_ghost: Option<ItemPrototypeIdentification>,
    pub cursor_stack: Option<LuaItemStack>,
    pub driving: bool,
    pub drop_item_distance: u32,
    pub following_robots: Vec<LuaEntity>,
    pub force: ForceIdentification,
    pub force_index: u32,
    pub in_combat: bool,
    pub item_pickup_distance: f64,
    pub loot_pickup_distance: f64,
    pub mining_state: LuaControlMiningState,
    pub opened: Option<LuaControlOpenedUnion>,
    pub opened_gui_type: Option<GuiType>,
    pub picking_state: bool,
    pub position: MapPosition,
    pub reach_distance: u32,
    pub repair_state: LuaControlRepairState,
    pub resource_reach_distance: f64,
    pub riding_state: RidingState,
    pub selected: Option<LuaEntity>,
    pub shooting_state: LuaControlShootingState,
    pub surface: LuaSurface,
    pub surface_index: u32,
    pub vehicle: Option<LuaEntity>,
    pub vehicle_logistic_requests_enabled: bool,
    pub walking_state: LuaControlWalkingState,
}

/// The control behavior for an entity. Inserters have logistic network and circuit network behavior logic, lamps have circuit logic and so on. This is an abstract base class that concrete control behaviors inherit.
pub struct LuaControlBehavior {
    pub entity: LuaEntity,
    pub typ: ControlBehaviorType,
}

/// A custom tag that shows on the map view.
pub struct LuaCustomChartTag {
    pub force: LuaForce,
    pub icon: SignalID,
    pub last_user: Option<LuaPlayer>,
    pub object_name: String,
    pub position: MapPosition,
    pub surface: LuaSurface,
    pub tag_number: u32,
    pub text: String,
    pub valid: bool,
}

/// Prototype of a custom input.
pub struct LuaCustomInputPrototype {
    pub action: String,
    pub alternative_key_sequence: Option<String>,
    pub consuming: String,
    pub enabled: bool,
    pub enabled_while_in_cutscene: bool,
    pub enabled_while_spectating: bool,
    pub include_selected_prototype: bool,
    pub item_to_spawn: Option<LuaItemPrototype>,
    pub key_sequence: String,
    pub linked_game_control: Option<String>,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub valid: bool,
}

/// Lazily evaluated table. For performance reasons, we sometimes return a custom table-like type instead of a native Lua table. This custom type lazily constructs the necessary Lua wrappers of the corresponding C++ objects, therefore preventing their unnecessary construction in some cases.
/// 
/// There are some notable consequences to the usage of a custom table type rather than the native Lua table type: Iterating a custom table is only possible using the `pairs` Lua function; `ipairs` won't work. Another key difference is that custom tables cannot be serialised into a game save file -- if saving the game would require serialisation of a custom table, an error will be displayed and the game will not be saved.
pub struct LuaCustomTable {
    pub object_name: String,
    pub valid: bool,
}

/// Prototype of a damage.
pub struct LuaDamagePrototype {
    pub hidden: bool,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub valid: bool,
}

/// Control behavior for decider combinators.
pub struct LuaDeciderCombinatorControlBehavior {
    pub lua_combinator_control_behavior: Box<LuaCombinatorControlBehavior>,
    pub object_name: String,
    pub parameters: DeciderCombinatorParameters,
    pub valid: bool,
}

/// Prototype of an optimized decorative.
pub struct LuaDecorativePrototype {
    pub autoplace_specification: Option<AutoplaceSpecification>,
    pub collision_box: BoundingBox,
    pub collision_mask: CollisionMask,
    pub collision_mask_with_flags: CollisionMaskWithFlags,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub valid: bool,
}

/// Prototype of an electric energy source.
pub struct LuaElectricEnergySourcePrototype {
    pub buffer_capacity: f64,
    pub drain: f64,
    pub emissions: f64,
    pub input_flow_limit: f64,
    pub object_name: String,
    pub output_flow_limit: f64,
    pub render_no_network_icon: bool,
    pub render_no_power_icon: bool,
    pub usage_priority: String,
    pub valid: bool,
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
    pub green: Vec<LuaEntity>,
    pub red: Vec<LuaEntity>,
}

/// The primary interface for interacting with entities through the Lua API. Entities are everything that exists on the map except for tiles (see [LuaTile](LuaTile)).
/// 
/// Most functions on LuaEntity also work when the entity is contained in a ghost.
pub struct LuaEntity {
    pub lua_control: Box<LuaControl>,
    pub active: bool,
    pub ai_settings: LuaAISettings,
    pub alert_parameters: ProgrammableSpeakerAlertParameters,
    pub allow_dispatching_robots: bool,
    pub amount: u32,
    pub armed: bool,
    pub associated_player: Option<LuaEntityAssociatedPlayerUnion>,
    pub auto_launch: bool,
    pub autopilot_destination: Option<MapPosition>,
    pub autopilot_destinations: Vec<MapPosition>,
    pub backer_name: Option<String>,
    pub beacons_count: Option<u32>,
    pub belt_neighbours: HashMap<String, Vec<LuaEntity>>,
    pub belt_to_ground_type: String,
    pub bonus_mining_progress: Option<f64>,
    pub bonus_progress: f64,
    pub bounding_box: BoundingBox,
    pub burner: Option<Box<LuaBurner>>,
    pub chain_signal_state: ChainSignalState,
    pub character_corpse_death_cause: LocalisedString,
    pub character_corpse_player_index: u32,
    pub character_corpse_tick_of_death: u32,
    pub circuit_connected_entities: Option<LuaEntityCircuitConnectedEntities>,
    pub circuit_connection_definitions: Option<Vec<CircuitConnectionDefinition>>,
    pub cliff_orientation: CliffOrientation,
    pub color: Option<Color>,
    pub combat_robot_owner: Option<Box<LuaEntity>>,
    pub command: Option<Box<Command>>,
    pub connected_rail: Option<Box<LuaEntity>>,
    pub connected_rail_direction: RailDirection,
    pub consumption_bonus: f64,
    pub consumption_modifier: f32,
    pub corpse_expires: bool,
    pub corpse_immune_to_entity_placement: bool,
    pub crafting_progress: f32,
    pub crafting_speed: f64,
    pub damage_dealt: f64,
    pub destructible: bool,
    pub direction: Direction,
    pub distraction_command: Option<Box<Command>>,
    pub driver_is_gunner: Option<bool>,
    pub drop_position: MapPosition,
    pub drop_target: Option<Box<LuaEntity>>,
    pub effective_speed: Option<f32>,
    pub effectivity_modifier: f32,
    pub effects: Option<ModuleEffects>,
    pub electric_buffer_size: Option<f64>,
    pub electric_drain: Option<f64>,
    pub electric_emissions: Option<f64>,
    pub electric_input_flow_limit: Option<f64>,
    pub electric_network_id: Option<u32>,
    pub electric_network_statistics: LuaFlowStatistics,
    pub electric_output_flow_limit: Option<f64>,
    pub enable_logistics_while_moving: bool,
    pub energy: f64,
    pub energy_generated_last_tick: f64,
    pub entity_label: Option<String>,
    pub filter_slot_count: u32,
    pub fluidbox: LuaFluidBox,
    pub follow_offset: Option<Vector>,
    pub follow_target: Option<Box<LuaEntity>>,
    pub friction_modifier: f32,
    pub ghost_localised_description: LocalisedString,
    pub ghost_localised_name: LocalisedString,
    pub ghost_name: String,
    pub ghost_prototype: LuaEntityGhostPrototypeUnion,
    pub ghost_type: String,
    pub ghost_unit_number: Option<u32>,
    pub graphics_variation: Option<u8>,
    pub grid: Option<LuaEquipmentGrid>,
    pub health: Option<f32>,
    pub held_stack: LuaItemStack,
    pub held_stack_position: MapPosition,
    pub highlight_box_blink_interval: u32,
    pub highlight_box_type: String,
    pub infinity_container_filters: Vec<InfinityInventoryFilter>,
    pub initial_amount: Option<u32>,
    pub inserter_filter_mode: Option<String>,
    pub inserter_stack_size_override: u32,
    pub inserter_target_pickup_count: u32,
    pub is_entity_with_force: bool,
    pub is_entity_with_health: bool,
    pub is_entity_with_owner: bool,
    pub is_military_target: bool,
    pub item_requests: HashMap<String, u32>,
    pub kills: u32,
    pub last_user: Option<LuaEntityLastUserUnion>,
    pub link_id: u32,
    pub linked_belt_neighbour: Option<Box<LuaEntity>>,
    pub linked_belt_type: String,
    pub loader_container: Option<Box<LuaEntity>>,
    pub loader_type: String,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub logistic_cell: LuaLogisticCell,
    pub logistic_network: LuaLogisticNetwork,
    pub minable: bool,
    pub mining_progress: Option<f64>,
    pub mining_target: Option<Box<LuaEntity>>,
    pub moving: bool,
    pub name: String,
    pub neighbour_bonus: f64,
    pub neighbours: Box<LuaEntityNeighboursUnion>,
    pub object_name: String,
    pub operable: bool,
    pub orientation: RealOrientation,
    pub parameters: ProgrammableSpeakerParameters,
    pub pickup_position: MapPosition,
    pub pickup_target: Option<Box<LuaEntity>>,
    pub player: Option<LuaPlayer>,
    pub pollution_bonus: f64,
    pub power_production: f64,
    pub power_switch_state: bool,
    pub power_usage: f64,
    pub previous_recipe: Option<LuaRecipe>,
    pub productivity_bonus: f64,
    pub products_finished: u32,
    pub prototype: LuaEntityPrototype,
    pub proxy_target: Option<Box<LuaEntity>>,
    pub pump_rail_target: Option<Box<LuaEntity>>,
    pub radar_scan_progress: f32,
    pub recipe_locked: bool,
    pub relative_turret_orientation: Option<RealOrientation>,
    pub remove_unfiltered_items: bool,
    pub render_player: Option<LuaEntityRenderPlayerUnion>,
    pub render_to_forces: Option<Vec<ForceIdentification>>,
    pub request_from_buffers: bool,
    pub request_slot_count: u32,
    pub rocket_parts: u32,
    pub rocket_silo_status: RocketSiloStatus,
    pub rotatable: bool,
    pub secondary_bounding_box: Option<BoundingBox>,
    pub secondary_selection_box: Option<BoundingBox>,
    pub selected_gun_index: Option<u32>,
    pub selection_box: BoundingBox,
    pub shooting_target: Option<Box<LuaEntity>>,
    pub signal_state: SignalState,
    pub spawner: Option<Box<LuaEntity>>,
    pub speed: Option<f32>,
    pub speed_bonus: f64,
    pub splitter_filter: Option<LuaItemPrototype>,
    pub splitter_input_priority: String,
    pub splitter_output_priority: String,
    pub stack: LuaItemStack,
    pub status: Option<EntityStatus>,
    pub sticked_to: Box<LuaEntity>,
    pub stickers: Option<Vec<LuaEntity>>,
    pub storage_filter: LuaItemPrototype,
    pub supports_direction: bool,
    pub tags: Option<Tags>,
    pub temperature: Option<f64>,
    pub text: LocalisedString,
    pub tick_of_last_attack: u32,
    pub tick_of_last_damage: u32,
    pub tile_height: u32,
    pub tile_width: u32,
    pub time_to_live: u32,
    pub time_to_next_effect: u32,
    pub timeout: u32,
    pub to_be_looted: bool,
    pub torso_orientation: RealOrientation,
    pub train: Option<Box<LuaTrain>>,
    pub trains_count: u32,
    pub trains_in_block: u32,
    pub trains_limit: u32,
    pub tree_color_index: u8,
    pub tree_color_index_max: u8,
    pub tree_gray_stage_index: u8,
    pub tree_gray_stage_index_max: u8,
    pub tree_stage_index: u8,
    pub tree_stage_index_max: u8,
    pub typ: String,
    pub unit_group: Option<Box<LuaUnitGroup>>,
    pub unit_number: Option<u32>,
    pub units: Vec<LuaEntity>,
    pub valid: bool,
    pub vehicle_automatic_targeting_parameters: VehicleAutomaticTargetingParameters,
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
    pub fluid_amount: Option<f64>,
    pub minable: bool,
    pub mining_particle: Option<String>,
    pub mining_time: f64,
    pub mining_trigger: Option<Vec<TriggerItem>>,
    pub products: Option<Vec<Product>>,
    pub required_fluid: Option<String>,
}

pub struct LuaEntityPrototypeSpawnCooldown {
    pub max: f64,
    pub min: f64,
}

/// Prototype of an entity.
pub struct LuaEntityPrototype {
    pub active_energy_usage: Option<f64>,
    pub additional_pastable_entities: Vec<LuaEntityPrototype>,
    pub adjacent_tile_collision_box: Option<BoundingBox>,
    pub adjacent_tile_collision_mask: Option<CollisionMask>,
    pub adjacent_tile_collision_test: Option<CollisionMask>,
    pub affected_by_tiles: Option<bool>,
    pub air_resistance: Option<f64>,
    pub alert_icon_scale: f32,
    pub alert_icon_shift: Vector,
    pub alert_when_attacking: Option<bool>,
    pub alert_when_damaged: Option<bool>,
    pub allow_access_to_all_forces: Option<bool>,
    pub allow_burner_leech: Option<bool>,
    pub allow_copy_paste: bool,
    pub allow_custom_vectors: Option<bool>,
    pub allow_passengers: Option<bool>,
    pub allow_run_time_change_of_is_military_target: Option<bool>,
    pub allowed_effects: Option<HashMap<String, bool>>,
    pub always_on: Option<bool>,
    pub ammo_category: Option<String>,
    pub animation_speed_coefficient: Option<f64>,
    pub attack_parameters: Option<AttackParameters>,
    pub attack_result: Option<Vec<TriggerItem>>,
    pub automated_ammo_count: Option<u32>,
    pub automatic_weapon_cycling: Option<bool>,
    pub autoplace_specification: Option<AutoplaceSpecification>,
    pub base_productivity: Option<f64>,
    pub belt_distance: Option<f64>,
    pub belt_length: Option<f64>,
    pub belt_speed: Option<f64>,
    pub braking_force: Option<f64>,
    pub build_base_evolution_requirement: f64,
    pub build_distance: Option<u32>,
    pub building_grid_bit_shift: u32,
    pub burner_prototype: Option<LuaBurnerPrototype>,
    pub burns_fluid: Option<bool>,
    pub call_for_help_radius: Option<f64>,
    pub can_open_gates: Option<bool>,
    pub center_collision_mask: Option<CollisionMask>,
    pub chain_shooting_cooldown_modifier: Option<f64>,
    pub character_corpse: Option<Box<LuaEntityPrototype>>,
    pub chunk_exploration_radius: Option<f64>,
    pub cliff_explosive_prototype: Option<String>,
    pub collision_box: BoundingBox,
    pub collision_mask: CollisionMask,
    pub collision_mask_collides_with_self: bool,
    pub collision_mask_collides_with_tiles_only: bool,
    pub collision_mask_considers_tile_transitions: bool,
    pub collision_mask_with_flags: CollisionMaskWithFlags,
    pub color: Option<Color>,
    pub construction_radius: Option<f64>,
    pub consumption: Option<f64>,
    pub container_distance: Option<f64>,
    pub corpses: Option<HashMap<String, LuaEntityPrototype>>,
    pub count_as_rock_for_filtered_deconstruction: Option<bool>,
    pub crafting_categories: Option<HashMap<String, bool>>,
    pub crafting_speed: Option<f64>,
    pub create_ghost_on_death: bool,
    pub created_effect: Option<Vec<TriggerItem>>,
    pub created_smoke: Option<LuaEntityPrototypeCreatedSmoke>,
    pub damage_hit_tint: Option<Color>,
    pub darkness_for_all_lamps_off: Option<f32>,
    pub darkness_for_all_lamps_on: Option<f32>,
    pub default_collision_mask_with_flags: CollisionMaskWithFlags,
    pub destroy_non_fuel_fluid: Option<bool>,
    pub distraction_cooldown: Option<u32>,
    pub distribution_effectivity: Option<f64>,
    pub door_opening_speed: Option<f64>,
    pub draw_cargo: Option<bool>,
    pub drawing_box: BoundingBox,
    pub drop_item_distance: Option<u32>,
    pub dying_speed: Option<f32>,
    pub effectivity: Option<f64>,
    pub electric_energy_source_prototype: Option<LuaElectricEnergySourcePrototype>,
    pub emissions_per_second: f64,
    pub enemy_map_color: Color,
    pub energy_per_hit_point: Option<f64>,
    pub energy_per_move: Option<f64>,
    pub energy_per_tick: Option<f64>,
    pub energy_usage: Option<f64>,
    pub engine_starting_speed: Option<f64>,
    pub enter_vehicle_distance: Option<f64>,
    pub explosion_beam: Option<f64>,
    pub explosion_rotate: Option<f64>,
    pub fast_replaceable_group: Option<String>,
    pub filter_count: Option<u32>,
    pub final_attack_result: Option<Vec<TriggerItem>>,
    pub fixed_recipe: Option<String>,
    pub flags: EntityPrototypeFlags,
    pub fluid: Option<LuaFluidPrototype>,
    pub fluid_capacity: f64,
    pub fluid_energy_source_prototype: Option<LuaFluidEnergySourcePrototype>,
    pub fluid_usage_per_tick: Option<f64>,
    pub fluidbox_prototypes: Vec<LuaFluidBoxPrototype>,
    pub flying_acceleration: Option<f64>,
    pub flying_speed: Option<f64>,
    pub friction_force: Option<f64>,
    pub friendly_map_color: Color,
    pub grid_prototype: Option<LuaEquipmentGridPrototype>,
    pub group: LuaGroup,
    pub guns: Option<HashMap<String, LuaItemPrototype>>,
    pub has_belt_immunity: Option<bool>,
    pub healing_per_tick: Option<f32>,
    pub heat_buffer_prototype: Option<LuaHeatBufferPrototype>,
    pub heat_energy_source_prototype: Option<LuaHeatEnergySourcePrototype>,
    pub height: Option<f64>,
    pub idle_energy_usage: Option<f64>,
    pub indexed_guns: Option<Vec<LuaItemPrototype>>,
    pub infinite_depletion_resource_amount: Option<u32>,
    pub infinite_resource: Option<bool>,
    pub ingredient_count: Option<u32>,
    pub inserter_chases_belt_items: Option<bool>,
    pub inserter_drop_position: Option<Vector>,
    pub inserter_extension_speed: Option<f64>,
    pub inserter_pickup_position: Option<Vector>,
    pub inserter_rotation_speed: Option<f64>,
    pub inserter_stack_size_bonus: Option<f64>,
    pub instruments: Option<Vec<ProgrammableSpeakerInstrument>>,
    pub is_building: bool,
    pub is_entity_with_owner: bool,
    pub is_military_target: Option<bool>,
    pub item_pickup_distance: Option<f64>,
    pub item_slot_count: Option<u32>,
    pub items_to_place_this: Option<Vec<ItemStackDefinition>>,
    pub lab_inputs: Option<Vec<String>>,
    pub lamp_energy_usage: Option<f64>,
    pub launch_wait_time: Option<u8>,
    pub light_blinking_speed: Option<f64>,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub logistic_mode: Option<String>,
    pub logistic_parameters: Option<LuaEntityPrototypeLogisticParameters>,
    pub logistic_radius: Option<f64>,
    pub loot: Option<Vec<Loot>>,
    pub loot_pickup_distance: Option<f64>,
    pub manual_range_modifier: Option<f64>,
    pub map_color: Option<Color>,
    pub map_generator_bounding_box: BoundingBox,
    pub max_circuit_wire_distance: f64,
    pub max_count_of_owned_units: Option<f64>,
    pub max_darkness_to_spawn: Option<f32>,
    pub max_distance_of_nearby_sector_revealed: Option<u32>,
    pub max_distance_of_sector_revealed: Option<u32>,
    pub max_energy: Option<f64>,
    pub max_energy_production: f64,
    pub max_energy_usage: f64,
    pub max_friends_around_to_spawn: Option<f64>,
    pub max_health: f32,
    pub max_payload_size: Option<u32>,
    pub max_polyphony: Option<u32>,
    pub max_power_output: Option<f64>,
    pub max_pursue_distance: Option<f64>,
    pub max_speed: Option<f64>,
    pub max_to_charge: Option<f32>,
    pub max_underground_distance: Option<u8>,
    pub max_wire_distance: f64,
    pub maximum_corner_sliding_distance: Option<f64>,
    pub maximum_temperature: Option<f64>,
    pub min_darkness_to_spawn: Option<f32>,
    pub min_pursue_time: Option<u32>,
    pub min_to_charge: Option<f32>,
    pub mineable_properties: LuaEntityPrototypeMineableProperties,
    pub minimum_resource_amount: Option<u32>,
    pub mining_drill_radius: Option<f64>,
    pub mining_speed: Option<f64>,
    pub module_inventory_size: Option<u32>,
    pub move_while_shooting: Option<bool>,
    pub name: String,
    pub neighbour_bonus: Option<f64>,
    pub next_upgrade: Option<Box<LuaEntityPrototype>>,
    pub normal_resource_amount: Option<u32>,
    pub object_name: String,
    pub order: String,
    pub pollution_to_join_attack: Option<f32>,
    pub protected_from_tile_building: bool,
    pub pumping_speed: Option<f64>,
    pub radar_range: Option<u32>,
    pub radius: f64,
    pub reach_distance: Option<u32>,
    pub reach_resource_distance: Option<f64>,
    pub related_underground_belt: Option<Box<LuaEntityPrototype>>,
    pub remains_when_mined: Vec<LuaEntityPrototype>,
    pub remove_decoratives: String,
    pub repair_speed_modifier: Option<u32>,
    pub researching_speed: Option<f64>,
    pub resistances: Option<HashMap<String, Resistance>>,
    pub resource_categories: Option<HashMap<String, bool>>,
    pub resource_category: Option<String>,
    pub respawn_time: Option<u32>,
    pub result_units: Option<Vec<UnitSpawnDefinition>>,
    pub rising_speed: Option<f64>,
    pub rocket_entity_prototype: Option<Box<LuaEntityPrototype>>,
    pub rocket_parts_required: Option<u32>,
    pub rocket_rising_delay: Option<u8>,
    pub rotation_speed: Option<f64>,
    pub running_speed: Option<f64>,
    pub scale_fluid_usage: Option<bool>,
    pub secondary_collision_box: Option<BoundingBox>,
    pub selectable_in_game: bool,
    pub selection_box: BoundingBox,
    pub selection_priority: u32,
    pub shooting_cursor_size: f64,
    pub spawn_cooldown: Option<LuaEntityPrototypeSpawnCooldown>,
    pub spawning_radius: Option<f64>,
    pub spawning_spacing: Option<f64>,
    pub spawning_time_modifier: Option<f64>,
    pub speed: Option<f64>,
    pub speed_multiplier_when_out_of_energy: Option<f32>,
    pub stack: Option<bool>,
    pub sticker_box: BoundingBox,
    pub subgroup: LuaGroup,
    pub supply_area_distance: Option<f64>,
    pub supports_direction: bool,
    pub tank_driving: Option<bool>,
    pub target_temperature: Option<f64>,
    pub terrain_friction_modifier: Option<f32>,
    pub ticks_to_keep_aiming_direction: Option<u32>,
    pub ticks_to_keep_gun: Option<u32>,
    pub ticks_to_stay_in_combat: Option<u32>,
    pub tile_height: u32,
    pub tile_width: u32,
    pub time_to_live: u32,
    pub timeout: Option<u32>,
    pub torso_bob_speed: Option<f64>,
    pub torso_rotation_speed: Option<f64>,
    pub tree_color_count: Option<u8>,
    pub trigger_collision_mask: Option<CollisionMaskWithFlags>,
    pub turret_range: Option<u32>,
    pub turret_rotation_speed: Option<f64>,
    pub typ: String,
    pub use_exact_mode: Option<bool>,
    pub valid: bool,
    pub vision_distance: Option<f64>,
    pub void_energy_source_prototype: Option<LuaVoidEnergySourcePrototype>,
    pub weight: Option<f64>,
}

pub struct LuaEquipmentShape {
    pub height: u32,
    pub width: u32,
}

/// An item in a [LuaEquipmentGrid](LuaEquipmentGrid), for example a fusion reactor placed in one's power armor.
/// 
/// An equipment reference becomes invalid once the equipment is removed or the equipment grid it resides in is destroyed.
pub struct LuaEquipment {
    pub burner: Option<Box<LuaBurner>>,
    pub energy: f64,
    pub generator_power: f64,
    pub max_energy: f64,
    pub max_shield: f64,
    pub max_solar_power: f64,
    pub movement_bonus: f64,
    pub name: String,
    pub object_name: String,
    pub position: EquipmentPosition,
    pub prototype: LuaEquipmentPrototype,
    pub shape: LuaEquipmentShape,
    pub shield: f64,
    pub typ: String,
    pub valid: bool,
}

/// Prototype of an equipment category.
pub struct LuaEquipmentCategoryPrototype {
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub valid: bool,
}

/// An equipment grid is for example the inside of a power armor.
pub struct LuaEquipmentGrid {
    pub available_in_batteries: f64,
    pub battery_capacity: f64,
    pub equipment: Vec<LuaEquipment>,
    pub generator_energy: f64,
    pub height: u32,
    pub inhibit_movement_bonus: bool,
    pub max_shield: f32,
    pub max_solar_energy: f64,
    pub object_name: String,
    pub prototype: LuaEquipmentGridPrototype,
    pub shield: f32,
    pub unique_id: u32,
    pub valid: bool,
    pub width: u32,
}

/// Prototype of an equipment grid.
pub struct LuaEquipmentGridPrototype {
    pub equipment_categories: Vec<String>,
    pub height: u32,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub locked: bool,
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub valid: bool,
    pub width: u32,
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
    pub points: Option<Vec<EquipmentPoint>>,
    pub width: u32,
}

/// Prototype of a modular equipment.
pub struct LuaEquipmentPrototype {
    pub attack_parameters: Option<AttackParameters>,
    pub automatic: bool,
    pub background_color: Color,
    pub burner_prototype: Option<LuaBurnerPrototype>,
    pub electric_energy_source_prototype: Option<LuaElectricEnergySourcePrototype>,
    pub energy_consumption: f64,
    pub energy_per_shield: f64,
    pub energy_production: f64,
    pub energy_source: LuaElectricEnergySourcePrototype,
    pub equipment_categories: Vec<String>,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub logistic_parameters: Option<LuaEquipmentPrototypeLogisticParameters>,
    pub movement_bonus: Option<f32>,
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub shape: LuaEquipmentPrototypeShape,
    pub shield: f32,
    pub take_result: Option<Box<LuaItemPrototype>>,
    pub typ: String,
    pub valid: bool,
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
    pub force: Option<Box<LuaForce>>,
    pub input_counts: HashMap<String, LuaFlowStatisticsInputCountsUnion>,
    pub object_name: String,
    pub output_counts: HashMap<String, LuaFlowStatisticsOutputCountsUnion>,
    pub valid: bool,
}

/// An array of fluid boxes of an entity. Entities may contain more than one fluid box, and some can change the number of fluid boxes -- for instance, an assembling machine will change its number of fluid boxes depending on its active recipe. See [Fluid](Fluid).
/// 
/// Do note that reading from a [LuaFluidBox](LuaFluidBox) creates a new table and writing will copy the given fields from the table into the engine's own fluid box structure. Therefore, the correct way to update a fluidbox of an entity is to read it first, modify the table, then write the modified table back. Directly accessing the returned table's attributes won't have the desired effect.
pub struct LuaFluidBox {
    pub object_name: String,
    pub owner: Box<LuaEntity>,
    pub valid: bool,
}

/// A prototype of a fluidbox owned by some [LuaEntityPrototype](LuaEntityPrototype).
pub struct LuaFluidBoxPrototype {
    pub base_area: f64,
    pub base_level: f64,
    pub entity: Box<LuaEntityPrototype>,
    pub filter: Option<LuaFluidPrototype>,
    pub height: f64,
    pub index: u32,
    pub maximum_temperature: Option<f64>,
    pub minimum_temperature: Option<f64>,
    pub object_name: String,
    pub pipe_connections: Vec<FluidBoxConnection>,
    pub production_type: String,
    pub render_layer: String,
    pub secondary_draw_orders: Vec<i32>,
    pub valid: bool,
    pub volume: f64,
}

/// Prototype of a fluid energy source.
pub struct LuaFluidEnergySourcePrototype {
    pub burns_fluid: bool,
    pub destroy_non_fuel_fluid: bool,
    pub effectivity: f64,
    pub emissions: f64,
    pub fluid_box: LuaFluidBoxPrototype,
    pub fluid_usage_per_tick: f64,
    pub maximum_temperature: f64,
    pub object_name: String,
    pub render_no_network_icon: bool,
    pub render_no_power_icon: bool,
    pub scale_fluid_usage: bool,
    pub smoke: Vec<SmokeSource>,
    pub valid: bool,
}

/// Prototype of a fluid.
pub struct LuaFluidPrototype {
    pub base_color: Color,
    pub default_temperature: f64,
    pub emissions_multiplier: f64,
    pub flow_color: Color,
    pub fuel_value: f64,
    pub gas_temperature: f64,
    pub group: LuaGroup,
    pub heat_capacity: f64,
    pub hidden: bool,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub max_temperature: f64,
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub subgroup: LuaGroup,
    pub valid: bool,
}

/// Prototype of a font.
pub struct LuaFontPrototype {
    pub border: bool,
    pub border_color: Option<Color>,
    pub filtered: bool,
    pub from: String,
    pub name: String,
    pub object_name: String,
    pub size: i32,
    pub spacing: f32,
    pub valid: bool,
}

/// `LuaForce` encapsulates data local to each "force" or "faction" of the game. Default forces are player, enemy and neutral. Players and mods can create additional forces (up to 64 total).
pub struct LuaForce {
    pub ai_controllable: bool,
    pub artillery_range_modifier: f64,
    pub character_build_distance_bonus: u32,
    pub character_health_bonus: f64,
    pub character_inventory_slots_bonus: u32,
    pub character_item_drop_distance_bonus: u32,
    pub character_item_pickup_distance_bonus: f64,
    pub character_logistic_requests: bool,
    pub character_loot_pickup_distance_bonus: f64,
    pub character_reach_distance_bonus: u32,
    pub character_resource_reach_distance_bonus: f64,
    pub character_running_speed_modifier: f64,
    pub character_trash_slot_count: f64,
    pub color: Color,
    pub connected_players: Vec<LuaPlayer>,
    pub current_research: Option<LuaTechnology>,
    pub custom_color: Option<Color>,
    pub deconstruction_time_to_live: u32,
    pub entity_build_count_statistics: LuaFlowStatistics,
    pub evolution_factor: f64,
    pub evolution_factor_by_killing_spawners: f64,
    pub evolution_factor_by_pollution: f64,
    pub evolution_factor_by_time: f64,
    pub fluid_production_statistics: LuaFlowStatistics,
    pub following_robots_lifetime_modifier: f64,
    pub friendly_fire: bool,
    pub ghost_time_to_live: u32,
    pub index: u32,
    pub inserter_stack_size_bonus: f64,
    pub item_production_statistics: LuaFlowStatistics,
    pub items_launched: HashMap<String, u32>,
    pub kill_count_statistics: LuaFlowStatistics,
    pub laboratory_productivity_bonus: f64,
    pub laboratory_speed_modifier: f64,
    pub logistic_networks: HashMap<String, Vec<LuaLogisticNetwork>>,
    pub manual_crafting_speed_modifier: f64,
    pub manual_mining_speed_modifier: f64,
    pub max_failed_attempts_per_tick_per_construction_queue: u32,
    pub max_successful_attempts_per_tick_per_construction_queue: u32,
    pub maximum_following_robot_count: u32,
    pub mining_drill_productivity_bonus: f64,
    pub name: String,
    pub object_name: String,
    pub players: Vec<LuaPlayer>,
    pub previous_research: Option<LuaTechnology>,
    pub recipes: HashMap<String, LuaRecipe>,
    pub research_enabled: bool,
    pub research_progress: f64,
    pub research_queue: Vec<TechnologyIdentification>,
    pub research_queue_enabled: bool,
    pub rockets_launched: u32,
    pub share_chart: bool,
    pub stack_inserter_capacity_bonus: u32,
    pub technologies: HashMap<String, LuaTechnology>,
    pub train_braking_force_bonus: f64,
    pub valid: bool,
    pub worker_robots_battery_modifier: f64,
    pub worker_robots_speed_modifier: f64,
    pub worker_robots_storage_bonus: f64,
    pub zoom_to_world_blueprint_enabled: bool,
    pub zoom_to_world_deconstruction_planner_enabled: bool,
    pub zoom_to_world_enabled: bool,
    pub zoom_to_world_ghost_building_enabled: bool,
    pub zoom_to_world_selection_tool_enabled: bool,
}

/// Prototype of a fuel category.
pub struct LuaFuelCategoryPrototype {
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub valid: bool,
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
    pub achievement_prototypes: HashMap<String, LuaAchievementPrototype>,
    pub active_mods: HashMap<String, String>,
    pub ammo_category_prototypes: HashMap<String, LuaAmmoCategoryPrototype>,
    pub autoplace_control_prototypes: HashMap<String, LuaAutoplaceControlPrototype>,
    pub autosave_enabled: bool,
    pub backer_names: HashMap<u32, String>,
    pub connected_players: Vec<LuaPlayer>,
    pub console_command_used: bool,
    pub custom_input_prototypes: HashMap<String, LuaCustomInputPrototype>,
    pub damage_prototypes: HashMap<String, LuaDamagePrototype>,
    pub decorative_prototypes: HashMap<String, LuaDecorativePrototype>,
    pub default_map_gen_settings: MapGenSettings,
    pub difficulty: Difficulty,
    pub difficulty_settings: DifficultySettings,
    pub draw_resource_selection: bool,
    pub enemy_has_vision_on_land_mines: bool,
    pub entity_prototypes: HashMap<String, LuaEntityPrototype>,
    pub equipment_category_prototypes: HashMap<String, LuaEquipmentCategoryPrototype>,
    pub equipment_grid_prototypes: HashMap<String, LuaEquipmentGridPrototype>,
    pub equipment_prototypes: HashMap<String, LuaEquipmentPrototype>,
    pub finished: bool,
    pub finished_but_continuing: bool,
    pub fluid_prototypes: HashMap<String, LuaFluidPrototype>,
    pub font_prototypes: HashMap<String, LuaFontPrototype>,
    pub forces: HashMap<LuaGameScriptForcesUnion, LuaForce>,
    pub fuel_category_prototypes: HashMap<String, LuaFuelCategoryPrototype>,
    pub item_group_prototypes: HashMap<String, LuaGroup>,
    pub item_prototypes: HashMap<String, LuaItemPrototype>,
    pub item_subgroup_prototypes: HashMap<String, LuaGroup>,
    pub map_gen_presets: HashMap<String, MapGenPreset>,
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
    pub mod_setting_prototypes: HashMap<String, LuaModSettingPrototype>,
    pub module_category_prototypes: HashMap<String, LuaModuleCategoryPrototype>,
    pub named_noise_expressions: HashMap<String, LuaNamedNoiseExpression>,
    pub noise_layer_prototypes: HashMap<String, LuaNoiseLayerPrototype>,
    pub object_name: String,
    pub particle_prototypes: HashMap<String, LuaParticlePrototype>,
    pub permissions: LuaPermissionGroups,
    pub player: Option<LuaPlayer>,
    pub players: HashMap<LuaGameScriptPlayersUnion, LuaPlayer>,
    pub pollution_statistics: LuaFlowStatistics,
    pub recipe_category_prototypes: HashMap<String, LuaRecipeCategoryPrototype>,
    pub recipe_prototypes: HashMap<String, LuaRecipePrototype>,
    pub resource_category_prototypes: HashMap<String, LuaResourceCategoryPrototype>,
    pub shortcut_prototypes: HashMap<String, LuaShortcutPrototype>,
    pub speed: f32,
    pub styles: HashMap<String, String>,
    pub surfaces: HashMap<LuaGameScriptSurfacesUnion, LuaSurface>,
    pub technology_prototypes: HashMap<String, LuaTechnologyPrototype>,
    pub tick: u32,
    pub tick_paused: bool,
    pub ticks_played: u32,
    pub ticks_to_run: u32,
    pub tile_prototypes: HashMap<String, LuaTilePrototype>,
    pub trivial_smoke_prototypes: HashMap<String, LuaTrivialSmokePrototype>,
    pub virtual_signal_prototypes: HashMap<String, LuaVirtualSignalPrototype>,
}

/// An abstract base class for behaviors that support switching the entity on or off based on some condition.
pub struct LuaGenericOnOffControlBehavior {
    pub lua_control_behavior: Box<LuaControlBehavior>,
    pub circuit_condition: CircuitConditionDefinition,
    pub connect_to_logistic_network: bool,
    pub disabled: bool,
    pub logistic_condition: CircuitConditionDefinition,
    pub object_name: String,
    pub valid: bool,
}

/// Item group or subgroup.
pub struct LuaGroup {
    pub group: Option<Box<LuaGroup>>,
    pub localised_name: Option<LocalisedString>,
    pub name: Option<String>,
    pub object_name: String,
    pub order: String,
    pub order_in_recipe: Option<String>,
    pub subgroups: Option<Vec<LuaGroup>>,
    pub typ: Option<String>,
    pub valid: bool,
}

/// The root of the GUI. This type houses the root elements, `top`, `left`, `center`, `goal`, and `screen`, to which other elements can be added to be displayed on screen.
pub struct LuaGui {
    pub center: LuaGuiElement,
    pub children: HashMap<String, LuaGuiElement>,
    pub goal: LuaGuiElement,
    pub left: LuaGuiElement,
    pub object_name: String,
    pub player: LuaPlayer,
    pub relative: LuaGuiElement,
    pub screen: LuaGuiElement,
    pub top: LuaGuiElement,
    pub valid: bool,
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
    pub allow_decimal: bool,
    pub allow_negative: bool,
    pub allow_none_state: bool,
    pub anchor: Option<GuiAnchor>,
    pub auto_center: bool,
    pub badge_text: LocalisedString,
    pub caption: LocalisedString,
    pub children: Vec<LuaGuiElement>,
    pub children_names: Vec<String>,
    pub clear_and_focus_on_right_click: bool,
    pub clicked_sprite: SpritePath,
    pub column_count: u32,
    pub direction: String,
    pub drag_target: Option<Box<LuaGuiElement>>,
    pub draw_horizontal_line_after_headers: bool,
    pub draw_horizontal_lines: bool,
    pub draw_vertical_lines: bool,
    pub elem_filters: Option<PrototypeFilter>,
    pub elem_type: String,
    pub elem_value: Option<LuaGuiElementElemValueUnion>,
    pub enabled: bool,
    pub entity: Option<LuaEntity>,
    pub force: Option<String>,
    pub gui: Box<LuaGui>,
    pub horizontal_scroll_policy: String,
    pub hovered_sprite: SpritePath,
    pub ignored_by_interaction: bool,
    pub index: u32,
    pub is_password: bool,
    pub items: Vec<LocalisedString>,
    pub left_label_caption: LocalisedString,
    pub left_label_tooltip: LocalisedString,
    pub location: Option<GuiLocation>,
    pub locked: bool,
    pub lose_focus_on_confirm: bool,
    pub minimap_player_index: u32,
    pub mouse_button_filter: MouseButtonFlags,
    pub name: String,
    pub number: f64,
    pub numeric: bool,
    pub object_name: String,
    pub parent: Option<Box<LuaGuiElement>>,
    pub player_index: u32,
    pub position: MapPosition,
    pub raise_hover_events: bool,
    pub read_only: bool,
    pub resize_to_sprite: bool,
    pub right_label_caption: LocalisedString,
    pub right_label_tooltip: LocalisedString,
    pub selectable: bool,
    pub selected_index: u32,
    pub selected_tab_index: Option<u32>,
    pub show_percent_for_small_numbers: bool,
    pub slider_value: f64,
    pub sprite: SpritePath,
    pub state: bool,
    pub style: LuaGuiElementStyleUnion,
    pub surface_index: u32,
    pub switch_state: String,
    pub tabs: Vec<TabAndContent>,
    pub tags: Tags,
    pub text: String,
    pub tooltip: LocalisedString,
    pub typ: String,
    pub valid: bool,
    pub value: f64,
    pub vertical_centering: bool,
    pub vertical_scroll_policy: String,
    pub visible: bool,
    pub word_wrap: bool,
    pub zoom: f64,
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
    pub object_name: String,
    pub specific_heat: f64,
    pub valid: bool,
}

/// Prototype of a heat energy source.
pub struct LuaHeatEnergySourcePrototype {
    pub connections: Vec<HeatConnection>,
    pub default_temperature: f64,
    pub emissions: f64,
    pub heat_buffer_prototype: LuaHeatBufferPrototype,
    pub max_temperature: f64,
    pub max_transfer: f64,
    pub min_temperature_gradient: f64,
    pub min_working_temperature: f64,
    pub minimum_glow_temperature: f64,
    pub object_name: String,
    pub render_no_network_icon: bool,
    pub render_no_power_icon: bool,
    pub specific_heat: f64,
    pub valid: bool,
}

/// Control behavior for inserters.
pub struct LuaInserterControlBehavior {
    pub lua_generic_on_off_control_behavior: Box<LuaGenericOnOffControlBehavior>,
    pub circuit_hand_read_mode: ControlBehaviorInserterHandReadMode,
    pub circuit_mode_of_operation: ControlBehaviorInserterCircuitModeOfOperation,
    pub circuit_read_hand_contents: bool,
    pub circuit_set_stack_size: bool,
    pub circuit_stack_control_signal: SignalID,
    pub object_name: String,
    pub valid: bool,
}

/// A storage of item stacks.
pub struct LuaInventory {
    pub entity_owner: Option<Box<LuaEntity>>,
    pub equipment_owner: Option<Box<LuaEquipment>>,
    pub index: Option<Inventory>,
    pub mod_owner: Option<String>,
    pub object_name: String,
    pub player_owner: Option<LuaPlayer>,
    pub valid: bool,
}

/// Prototype of an item.
pub struct LuaItemPrototype {
    pub alt_entity_filter_mode: Option<String>,
    pub alt_entity_filters: Option<HashMap<String, LuaEntityPrototype>>,
    pub alt_entity_type_filters: Option<HashMap<String, bool>>,
    pub alt_reverse_alt_entity_filter_mode: Option<String>,
    pub alt_reverse_entity_filters: Option<HashMap<String, LuaEntityPrototype>>,
    pub alt_reverse_entity_type_filters: Option<HashMap<String, bool>>,
    pub alt_reverse_selection_border_color: Option<Color>,
    pub alt_reverse_selection_cursor_box_type: Option<String>,
    pub alt_reverse_selection_mode_flags: Option<SelectionModeFlags>,
    pub alt_reverse_tile_filter_mode: Option<String>,
    pub alt_reverse_tile_filters: Option<HashMap<String, LuaTilePrototype>>,
    pub alt_selection_border_color: Option<Color>,
    pub alt_selection_cursor_box_type: Option<String>,
    pub alt_selection_mode_flags: Option<SelectionModeFlags>,
    pub alt_tile_filter_mode: Option<String>,
    pub alt_tile_filters: Option<HashMap<String, LuaTilePrototype>>,
    pub always_include_tiles: Option<bool>,
    pub attack_parameters: Option<AttackParameters>,
    pub burnt_result: Option<Box<LuaItemPrototype>>,
    pub can_be_mod_opened: bool,
    pub capsule_action: Option<CapsuleAction>,
    pub category: Option<String>,
    pub curved_rail: Option<LuaEntityPrototype>,
    pub default_label_color: Option<Color>,
    pub default_request_amount: u32,
    pub draw_label_for_cursor_render: Option<bool>,
    pub durability: Option<f64>,
    pub durability_description_key: Option<String>,
    pub entity_filter_mode: Option<String>,
    pub entity_filter_slots: Option<u32>,
    pub entity_filters: Option<HashMap<String, LuaEntityPrototype>>,
    pub entity_type_filters: Option<HashMap<String, bool>>,
    pub equipment_grid: Option<LuaEquipmentGridPrototype>,
    pub extend_inventory_by_default: Option<bool>,
    pub filter_mode: Option<String>,
    pub flags: ItemPrototypeFlags,
    pub fuel_acceleration_multiplier: f64,
    pub fuel_category: Option<String>,
    pub fuel_emissions_multiplier: f64,
    pub fuel_top_speed_multiplier: f64,
    pub fuel_value: f32,
    pub group: LuaGroup,
    pub infinite: Option<bool>,
    pub insertion_priority_mode: Option<String>,
    pub inventory_size: Option<u32>,
    pub inventory_size_bonus: Option<u32>,
    pub item_filters: Option<HashMap<String, LuaItemPrototype>>,
    pub item_group_filters: Option<HashMap<String, LuaGroup>>,
    pub item_subgroup_filters: Option<HashMap<String, LuaGroup>>,
    pub limitation_message_key: Option<String>,
    pub limitations: Option<Vec<String>>,
    pub localised_description: LocalisedString,
    pub localised_filter_message: Option<LocalisedString>,
    pub localised_name: LocalisedString,
    pub magazine_size: Option<f32>,
    pub mapper_count: Option<u32>,
    pub module_effects: Option<ModuleEffects>,
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub place_as_equipment_result: Option<LuaEquipmentPrototype>,
    pub place_as_tile_result: Option<PlaceAsTileResult>,
    pub place_result: Option<LuaEntityPrototype>,
    pub reload_time: Option<f32>,
    pub repair_result: Option<Vec<TriggerItem>>,
    pub resistances: Option<HashMap<String, Resistance>>,
    pub reverse_alt_entity_filter_mode: Option<String>,
    pub reverse_entity_filters: Option<HashMap<String, LuaEntityPrototype>>,
    pub reverse_entity_type_filters: Option<HashMap<String, bool>>,
    pub reverse_selection_border_color: Option<Color>,
    pub reverse_selection_cursor_box_type: Option<String>,
    pub reverse_selection_mode_flags: Option<SelectionModeFlags>,
    pub reverse_tile_filter_mode: Option<String>,
    pub reverse_tile_filters: Option<HashMap<String, LuaTilePrototype>>,
    pub rocket_launch_products: Vec<Product>,
    pub selection_border_color: Option<Color>,
    pub selection_cursor_box_type: Option<String>,
    pub selection_mode_flags: Option<SelectionModeFlags>,
    pub speed: Option<f32>,
    pub stack_size: u32,
    pub stackable: bool,
    pub straight_rail: Option<LuaEntityPrototype>,
    pub subgroup: LuaGroup,
    pub tier: Option<u32>,
    pub tile_filter_mode: Option<String>,
    pub tile_filter_slots: Option<u32>,
    pub tile_filters: Option<HashMap<String, LuaTilePrototype>>,
    pub typ: String,
    pub valid: bool,
    pub wire_count: u32,
}

/// A reference to an item and count owned by some external entity.
pub struct LuaItemStack {
    pub active_index: Option<u32>,
    pub allow_manual_label_change: bool,
    pub ammo: u32,
    pub blueprint_absolute_snapping: bool,
    pub blueprint_icons: Option<Vec<BlueprintSignalIcon>>,
    pub blueprint_position_relative_to_grid: Option<TilePosition>,
    pub blueprint_snap_to_grid: Option<TilePosition>,
    pub connected_entity: Option<Box<LuaEntity>>,
    pub cost_to_build: HashMap<String, u32>,
    pub count: u32,
    pub custom_description: LocalisedString,
    pub default_icons: Vec<BlueprintSignalIcon>,
    pub durability: Option<f64>,
    pub entity_color: Option<Color>,
    pub entity_filter_count: u32,
    pub entity_filter_mode: DeconstructionItemEntityFilterMode,
    pub entity_filters: Vec<String>,
    pub entity_label: Option<String>,
    pub extends_inventory: bool,
    pub grid: Option<LuaEquipmentGrid>,
    pub health: f32,
    pub is_armor: bool,
    pub is_blueprint: bool,
    pub is_blueprint_book: bool,
    pub is_deconstruction_item: bool,
    pub is_item_with_entity_data: bool,
    pub is_item_with_inventory: bool,
    pub is_item_with_label: bool,
    pub is_item_with_tags: bool,
    pub is_mining_tool: bool,
    pub is_module: bool,
    pub is_repair_tool: bool,
    pub is_selection_tool: bool,
    pub is_tool: bool,
    pub is_upgrade_item: bool,
    pub item_number: Option<u32>,
    pub label: Option<String>,
    pub label_color: Option<Color>,
    pub name: String,
    pub object_name: String,
    pub prioritize_insertion_mode: String,
    pub prototype: LuaItemPrototype,
    pub tags: Tags,
    pub tile_filter_count: u32,
    pub tile_filter_mode: DeconstructionItemTileFilterMode,
    pub tile_filters: Vec<String>,
    pub tile_selection_mode: DeconstructionItemTileSelectionMode,
    pub trees_and_rocks_only: bool,
    pub typ: String,
    pub valid: bool,
    pub valid_for_read: bool,
}

/// Control behavior for lamps.
pub struct LuaLampControlBehavior {
    pub lua_generic_on_off_control_behavior: Box<LuaGenericOnOffControlBehavior>,
    pub color: Option<Color>,
    pub object_name: String,
    pub use_colors: bool,
    pub valid: bool,
}

/// A lazily loaded value. For performance reasons, we sometimes return a custom lazily-loaded value type instead of the native Lua value. This custom type lazily constructs the necessary value when [LuaLazyLoadedValue::get](LuaLazyLoadedValue::get) is called, therefore preventing its unnecessary construction in some cases.
/// 
/// An instance of LuaLazyLoadedValue is only valid during the event it was created from and cannot be saved.
pub struct LuaLazyLoadedValue {
    pub object_name: String,
    pub valid: bool,
}

/// Logistic cell of a particular [LuaEntity](LuaEntity). A "Logistic Cell" is the given name for settings and properties used by what would normally be seen as a "Roboport". A logistic cell however doesn't have to be attached to the roboport entity (the character has one for the personal roboport).
pub struct LuaLogisticCell {
    pub charge_approach_distance: f32,
    pub charging_robot_count: u32,
    pub charging_robots: Vec<LuaEntity>,
    pub construction_radius: f32,
    pub logistic_network: Option<LuaLogisticNetwork>,
    pub logistic_radius: f32,
    pub logistics_connection_distance: f32,
    pub mobile: bool,
    pub neighbours: Vec<LuaLogisticCell>,
    pub object_name: String,
    pub owner: Box<LuaEntity>,
    pub stationed_construction_robot_count: u32,
    pub stationed_logistic_robot_count: u32,
    pub to_charge_robot_count: u32,
    pub to_charge_robots: Vec<LuaEntity>,
    pub transmitting: bool,
    pub valid: bool,
}

/// Control behavior for logistic chests.
pub struct LuaLogisticContainerControlBehavior {
    pub lua_control_behavior: Box<LuaControlBehavior>,
    pub circuit_mode_of_operation: ControlBehaviorLogisticContainerCircuitModeOfOperation,
    pub object_name: String,
    pub valid: bool,
}

/// A single logistic network of a given force on a given surface.
pub struct LuaLogisticNetwork {
    pub active_provider_points: Vec<LuaLogisticPoint>,
    pub all_construction_robots: u32,
    pub all_logistic_robots: u32,
    pub available_construction_robots: u32,
    pub available_logistic_robots: u32,
    pub cells: Vec<LuaLogisticCell>,
    pub construction_robots: Vec<LuaEntity>,
    pub empty_provider_points: Vec<LuaLogisticPoint>,
    pub empty_providers: Vec<LuaEntity>,
    pub force: LuaForce,
    pub logistic_members: Vec<LuaEntity>,
    pub logistic_robots: Vec<LuaEntity>,
    pub object_name: String,
    pub passive_provider_points: Vec<LuaLogisticPoint>,
    pub provider_points: Vec<LuaLogisticPoint>,
    pub providers: Vec<LuaEntity>,
    pub requester_points: Vec<LuaLogisticPoint>,
    pub requesters: Vec<LuaEntity>,
    pub robot_limit: u32,
    pub robots: Vec<LuaEntity>,
    pub storage_points: Vec<LuaLogisticPoint>,
    pub storages: Vec<LuaEntity>,
    pub valid: bool,
}

/// Logistic point of a particular [LuaEntity](LuaEntity). A "Logistic point" is the name given for settings and properties used by requester, provider, and storage points in a given logistic network. These "points" don't have to be a logistic container but often are. One other entity that can own several points is the "character" character type entity.
pub struct LuaLogisticPoint {
    pub exact: bool,
    pub filters: Option<Vec<LogisticFilter>>,
    pub force: LuaForce,
    pub logistic_member_index: u32,
    pub logistic_network: LuaLogisticNetwork,
    pub mode: LogisticMode,
    pub object_name: String,
    pub owner: LuaEntity,
    pub targeted_items_deliver: HashMap<String, u32>,
    pub targeted_items_pickup: HashMap<String, u32>,
    pub valid: bool,
}

/// Control behavior for mining drills.
pub struct LuaMiningDrillControlBehavior {
    pub lua_generic_on_off_control_behavior: Box<LuaGenericOnOffControlBehavior>,
    pub circuit_enable_disable: bool,
    pub circuit_read_resources: bool,
    pub object_name: String,
    pub resource_read_mode: ControlBehaviorMiningDrillResourceReadMode,
    pub resource_read_targets: Vec<LuaEntity>,
    pub valid: bool,
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
    pub allow_blank: Option<bool>,
    pub allowed_values: Option<LuaModSettingPrototypeAllowedValuesUnion>,
    pub auto_trim: Option<bool>,
    pub default_value: LuaModSettingPrototypeDefaultValueUnion,
    pub hidden: bool,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub maximum_value: Option<LuaModSettingPrototypeMaximumValueUnion>,
    pub minimum_value: Option<LuaModSettingPrototypeMinimumValueUnion>,
    pub mod_name: String,
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub setting_type: String,
    pub valid: bool,
}

/// Prototype of a module category.
pub struct LuaModuleCategoryPrototype {
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub valid: bool,
}

/// Prototype of a named noise expression.
pub struct LuaNamedNoiseExpression {
    pub expression: NoiseExpression,
    pub intended_property: String,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub valid: bool,
}

/// Prototype of a noise layer.
pub struct LuaNoiseLayerPrototype {
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub valid: bool,
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
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub regular_trigger_effect: TriggerEffectItem,
    pub regular_trigger_effect_frequency: u32,
    pub render_layer: RenderLayer,
    pub render_layer_when_on_ground: RenderLayer,
    pub valid: bool,
}

/// A permission group that defines what players in this group are allowed to do.
pub struct LuaPermissionGroup {
    pub group_id: u32,
    pub name: String,
    pub object_name: String,
    pub players: Vec<LuaPlayer>,
    pub valid: bool,
}

/// All permission groups.
pub struct LuaPermissionGroups {
    pub groups: Vec<LuaPermissionGroup>,
    pub object_name: String,
    pub valid: bool,
}

/// A player in the game. Pay attention that a player may or may not have a character, which is the [LuaEntity](LuaEntity) of the little guy running around the world doing things.
pub struct LuaPlayer {
    pub lua_control: Box<LuaControl>,
    pub admin: bool,
    pub afk_time: u32,
    pub auto_sort_main_inventory: bool,
    pub blueprint_to_setup: LuaItemStack,
    pub character: Option<Box<LuaEntity>>,
    pub chat_color: Color,
    pub color: Color,
    pub connected: bool,
    pub controller_type: Controllers,
    pub cursor_stack_temporary: bool,
    pub cutscene_character: Option<Box<LuaEntity>>,
    pub display_resolution: DisplayResolution,
    pub display_scale: f64,
    pub drag_target: Option<Box<DragTarget>>,
    pub entity_copy_source: Option<Box<LuaEntity>>,
    pub game_view_settings: GameViewSettings,
    pub gui: Box<LuaGui>,
    pub hand_location: Option<ItemStackLocation>,
    pub index: u32,
    pub infinity_inventory_filters: Vec<InfinityInventoryFilter>,
    pub last_online: u32,
    pub map_view_settings: MapViewSettings,
    pub minimap_enabled: bool,
    pub mod_settings: HashMap<String, ModSetting>,
    pub name: String,
    pub object_name: String,
    pub online_time: u32,
    pub opened_self: bool,
    pub permission_group: Option<LuaPermissionGroup>,
    pub remove_unfiltered_items: bool,
    pub render_mode: RenderMode,
    pub show_on_map: bool,
    pub spectator: bool,
    pub stashed_controller_type: Option<Controllers>,
    pub tag: String,
    pub ticks_to_respawn: Option<u32>,
    pub valid: bool,
    pub zoom: f64,
}

/// An object used to measure script performance.
pub struct LuaProfiler {
    pub object_name: String,
    pub valid: bool,
}

/// Control behavior for programmable speakers.
pub struct LuaProgrammableSpeakerControlBehavior {
    pub lua_control_behavior: Box<LuaControlBehavior>,
    pub circuit_condition: CircuitConditionDefinition,
    pub circuit_parameters: ProgrammableSpeakerCircuitParameters,
    pub object_name: String,
    pub valid: bool,
}

/// An interface to send messages to the calling RCON interface through the global object named `rcon`.
pub struct LuaRCON {
    pub object_name: String,
}

/// Control behavior for rail chain signals.
pub struct LuaRailChainSignalControlBehavior {
    pub lua_control_behavior: Box<LuaControlBehavior>,
    pub blue_signal: SignalID,
    pub green_signal: SignalID,
    pub object_name: String,
    pub orange_signal: SignalID,
    pub red_signal: SignalID,
    pub valid: bool,
}

/// A rail path.
pub struct LuaRailPath {
    pub current: u32,
    pub is_front: bool,
    pub object_name: String,
    pub rails: HashMap<u32, LuaEntity>,
    pub size: u32,
    pub total_distance: f64,
    pub travelled_distance: f64,
    pub valid: bool,
}

/// Control behavior for rail signals.
pub struct LuaRailSignalControlBehavior {
    pub lua_control_behavior: Box<LuaControlBehavior>,
    pub circuit_condition: CircuitConditionDefinition,
    pub close_signal: bool,
    pub green_signal: SignalID,
    pub object_name: String,
    pub orange_signal: SignalID,
    pub read_signal: bool,
    pub red_signal: SignalID,
    pub valid: bool,
}

/// A deterministic random generator independent from the core games random generator that can be seeded and re-seeded at will. This random generator can be saved and loaded and will maintain its state. Note this is entirely different from calling [math.random](Libraries.html#math.random)() and you should be sure you actually want to use this over calling `math.random()`. If you aren't sure if you need to use this over calling `math.random()` then you probably don't need to use this.
pub struct LuaRandomGenerator {
    pub object_name: String,
    pub valid: bool,
}

/// A crafting recipe. Recipes belong to forces (see [LuaForce](LuaForce)) because some recipes are unlocked by research, and researches are per-force.
pub struct LuaRecipe {
    pub category: String,
    pub enabled: bool,
    pub energy: f64,
    pub force: LuaForce,
    pub group: LuaGroup,
    pub hidden: bool,
    pub hidden_from_flow_stats: bool,
    pub ingredients: Vec<Ingredient>,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub products: Vec<Product>,
    pub prototype: LuaRecipePrototype,
    pub subgroup: LuaGroup,
    pub valid: bool,
}

/// Prototype of a recipe category.
pub struct LuaRecipeCategoryPrototype {
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub valid: bool,
}

/// A crafting recipe prototype.
pub struct LuaRecipePrototype {
    pub allow_as_intermediate: bool,
    pub allow_decomposition: bool,
    pub allow_inserter_overload: bool,
    pub allow_intermediates: bool,
    pub always_show_made_in: bool,
    pub always_show_products: bool,
    pub category: String,
    pub emissions_multiplier: f64,
    pub enabled: bool,
    pub energy: f64,
    pub group: LuaGroup,
    pub hidden: bool,
    pub hidden_from_flow_stats: bool,
    pub hidden_from_player_crafting: bool,
    pub ingredients: Vec<Ingredient>,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub main_product: Option<Product>,
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub overload_multiplier: u32,
    pub products: Vec<Product>,
    pub request_paste_multiplier: u32,
    pub show_amount_in_title: bool,
    pub subgroup: LuaGroup,
    pub unlock_results: bool,
    pub valid: bool,
}

/// Registry of interfaces between scripts. An interface is simply a dictionary mapping names to functions. A script or mod can then register an interface with [LuaRemote](LuaRemote), after that any script can call the registered functions, provided it knows the interface name and the desired function name. An instance of LuaRemote is available through the global object named `remote`.
pub struct LuaRemote {
    pub interfaces: HashMap<String, HashSet<String>>,
    pub object_name: String,
}

/// Allows rendering of geometric shapes, text and sprites in the game world through the global object named `rendering`. Each render object is identified by an id that is universally unique for the lifetime of a whole game.
pub struct LuaRendering {
    pub object_name: String,
}

/// Prototype of a resource category.
pub struct LuaResourceCategoryPrototype {
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub valid: bool,
}

/// Control behavior for roboports.
pub struct LuaRoboportControlBehavior {
    pub lua_control_behavior: Box<LuaControlBehavior>,
    pub available_construction_output_signal: SignalID,
    pub available_logistic_output_signal: SignalID,
    pub object_name: String,
    pub read_logistics: bool,
    pub read_robot_stats: bool,
    pub total_construction_output_signal: SignalID,
    pub total_logistic_output_signal: SignalID,
    pub valid: bool,
}

/// Object containing mod settings of three distinct types: `startup`, `global`, and `player`. An instance of LuaSettings is available through the global object named `settings`.
pub struct LuaSettings {
    pub global: HashMap<String, ModSetting>,
    pub object_name: String,
    pub player: HashMap<String, ModSetting>,
    pub startup: HashMap<String, ModSetting>,
}

/// Prototype of a shortcut.
pub struct LuaShortcutPrototype {
    pub action: String,
    pub associated_control_input: Option<String>,
    pub item_to_spawn: Option<LuaItemPrototype>,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub technology_to_unlock: Option<LuaTechnologyPrototype>,
    pub toggleable: bool,
    pub valid: bool,
}

/// Control behavior for storage tanks.
pub struct LuaStorageTankControlBehavior {
    pub lua_control_behavior: Box<LuaControlBehavior>,
    pub object_name: String,
    pub valid: bool,
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
    pub bottom_cell_padding: i32,
    pub bottom_margin: i32,
    pub bottom_padding: i32,
    pub cell_padding: i32,
    pub clicked_font_color: Color,
    pub clicked_vertical_offset: i32,
    pub color: Color,
    pub column_alignments: HashMap<u32, Alignment>,
    pub default_badge_font_color: Color,
    pub disabled_badge_font_color: Color,
    pub disabled_font_color: Color,
    pub extra_bottom_margin_when_activated: i32,
    pub extra_bottom_padding_when_activated: i32,
    pub extra_left_margin_when_activated: i32,
    pub extra_left_padding_when_activated: i32,
    pub extra_margin_when_activated: LuaStyleExtraMarginWhenActivatedUnion,
    pub extra_padding_when_activated: LuaStyleExtraPaddingWhenActivatedUnion,
    pub extra_right_margin_when_activated: i32,
    pub extra_right_padding_when_activated: i32,
    pub extra_top_margin_when_activated: i32,
    pub extra_top_padding_when_activated: i32,
    pub font: String,
    pub font_color: Color,
    pub gui: Box<LuaGui>,
    pub height: i32,
    pub horizontal_align: Option<String>,
    pub horizontal_spacing: i32,
    pub horizontally_squashable: Option<bool>,
    pub horizontally_stretchable: Option<bool>,
    pub hovered_font_color: Color,
    pub left_cell_padding: i32,
    pub left_margin: i32,
    pub left_padding: i32,
    pub margin: LuaStyleMarginUnion,
    pub maximal_height: i32,
    pub maximal_width: i32,
    pub minimal_height: i32,
    pub minimal_width: i32,
    pub name: String,
    pub natural_height: i32,
    pub natural_width: i32,
    pub object_name: String,
    pub padding: LuaStylePaddingUnion,
    pub pie_progress_color: Color,
    pub rich_text_setting: RichTextSetting,
    pub right_cell_padding: i32,
    pub right_margin: i32,
    pub right_padding: i32,
    pub selected_badge_font_color: Color,
    pub selected_clicked_font_color: Color,
    pub selected_font_color: Color,
    pub selected_hovered_font_color: Color,
    pub single_line: bool,
    pub size: LuaStyleSizeUnion,
    pub stretch_image_to_widget_size: bool,
    pub strikethrough_color: Color,
    pub top_cell_padding: i32,
    pub top_margin: i32,
    pub top_padding: i32,
    pub use_header_filler: bool,
    pub valid: bool,
    pub vertical_align: Option<String>,
    pub vertical_spacing: i32,
    pub vertically_squashable: Option<bool>,
    pub vertically_stretchable: Option<bool>,
    pub width: i32,
}

/// A "domain" of the world. Surfaces can only be created and deleted through the API. Surfaces are uniquely identified by their name. Every game contains at least the surface "nauvis".
pub struct LuaSurface {
    pub always_day: bool,
    pub brightness_visual_weights: ColorModifier,
    pub darkness: f32,
    pub dawn: f64,
    pub daytime: f64,
    pub dusk: f64,
    pub evening: f64,
    pub freeze_daytime: bool,
    pub generate_with_lab_tiles: bool,
    pub index: u32,
    pub map_gen_settings: MapGenSettings,
    pub min_brightness: f64,
    pub morning: f64,
    pub name: String,
    pub object_name: String,
    pub peaceful_mode: bool,
    pub show_clouds: bool,
    pub solar_power_multiplier: f64,
    pub ticks_per_day: u32,
    pub valid: bool,
    pub wind_orientation: RealOrientation,
    pub wind_orientation_change: f64,
    pub wind_speed: f64,
}

/// One research item.
pub struct LuaTechnology {
    pub effects: Vec<TechnologyModifier>,
    pub enabled: bool,
    pub force: Box<LuaForce>,
    pub level: u32,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub prerequisites: HashMap<String, LuaTechnology>,
    pub prototype: LuaTechnologyPrototype,
    pub research_unit_count: u32,
    pub research_unit_count_formula: Option<String>,
    pub research_unit_energy: f64,
    pub research_unit_ingredients: Vec<Ingredient>,
    pub researched: bool,
    pub upgrade: bool,
    pub valid: bool,
    pub visible_when_disabled: bool,
}

/// A Technology prototype.
pub struct LuaTechnologyPrototype {
    pub effects: Vec<TechnologyModifier>,
    pub enabled: bool,
    pub hidden: bool,
    pub ignore_tech_cost_multiplier: bool,
    pub level: u32,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub max_level: u32,
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub prerequisites: HashMap<String, LuaTechnologyPrototype>,
    pub research_unit_count: u32,
    pub research_unit_count_formula: Option<String>,
    pub research_unit_energy: f64,
    pub research_unit_ingredients: Vec<Ingredient>,
    pub upgrade: bool,
    pub valid: bool,
    pub visible_when_disabled: bool,
}

/// A single "square" on the map.
pub struct LuaTile {
    pub hidden_tile: Option<String>,
    pub name: String,
    pub object_name: String,
    pub position: TilePosition,
    pub prototype: LuaTilePrototype,
    pub surface: LuaSurface,
    pub valid: bool,
}

pub struct LuaTilePrototypeMineableProperties {
    pub minable: bool,
    pub mining_particle: Option<String>,
    pub mining_time: f64,
    pub products: Vec<Product>,
}

/// Prototype of a tile.
pub struct LuaTilePrototype {
    pub allowed_neighbors: HashMap<String, LuaTilePrototype>,
    pub automatic_neighbors: bool,
    pub autoplace_specification: Option<AutoplaceSpecification>,
    pub can_be_part_of_blueprint: bool,
    pub check_collision_with_entities: bool,
    pub collision_mask: CollisionMask,
    pub collision_mask_with_flags: CollisionMaskWithFlags,
    pub decorative_removal_probability: f32,
    pub emissions_per_second: f64,
    pub items_to_place_this: Option<Vec<ItemStackDefinition>>,
    pub layer: u32,
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub map_color: Color,
    pub mineable_properties: LuaTilePrototypeMineableProperties,
    pub name: String,
    pub needs_correction: bool,
    pub next_direction: Option<Box<LuaTilePrototype>>,
    pub object_name: String,
    pub order: String,
    pub valid: bool,
    pub vehicle_friction_modifier: f32,
    pub walking_speed_modifier: f32,
}

/// A train. Trains are a sequence of connected rolling stocks -- locomotives and wagons.
pub struct LuaTrain {
    pub back_rail: Option<LuaEntity>,
    pub back_stock: Option<LuaEntity>,
    pub cargo_wagons: Vec<LuaEntity>,
    pub carriages: Vec<LuaEntity>,
    pub fluid_wagons: Vec<LuaEntity>,
    pub front_rail: Option<LuaEntity>,
    pub front_stock: Option<LuaEntity>,
    pub has_path: bool,
    pub id: u32,
    pub kill_count: u32,
    pub killed_players: HashMap<u32, u32>,
    pub locomotives: HashMap<String, Vec<LuaEntity>>,
    pub manual_mode: bool,
    pub max_backward_speed: f64,
    pub max_forward_speed: f64,
    pub object_name: String,
    pub passengers: Vec<LuaPlayer>,
    pub path: Option<LuaRailPath>,
    pub path_end_rail: Option<LuaEntity>,
    pub path_end_stop: Option<LuaEntity>,
    pub rail_direction_from_back_rail: RailDirection,
    pub rail_direction_from_front_rail: RailDirection,
    pub riding_state: RidingState,
    pub schedule: Option<TrainSchedule>,
    pub signal: Option<LuaEntity>,
    pub speed: f64,
    pub state: TrainState,
    pub station: Option<LuaEntity>,
    pub valid: bool,
    pub weight: f64,
}

/// Control behavior for train stops.
pub struct LuaTrainStopControlBehavior {
    pub lua_generic_on_off_control_behavior: Box<LuaGenericOnOffControlBehavior>,
    pub enable_disable: bool,
    pub object_name: String,
    pub read_from_train: bool,
    pub read_stopped_train: bool,
    pub read_trains_count: bool,
    pub send_to_train: bool,
    pub set_trains_limit: bool,
    pub stopped_train_signal: SignalID,
    pub trains_count_signal: SignalID,
    pub trains_limit_signal: SignalID,
    pub valid: bool,
}

/// Control behavior for transport belts.
pub struct LuaTransportBeltControlBehavior {
    pub lua_generic_on_off_control_behavior: Box<LuaGenericOnOffControlBehavior>,
    pub enable_disable: bool,
    pub object_name: String,
    pub read_contents: bool,
    pub read_contents_mode: ControlBehaviorTransportBeltContentReadMode,
    pub valid: bool,
}

/// One line on a transport belt.
pub struct LuaTransportLine {
    pub input_lines: Vec<LuaTransportLine>,
    pub object_name: String,
    pub output_lines: Vec<LuaTransportLine>,
    pub owner: LuaEntity,
    pub valid: bool,
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
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub render_layer: RenderLayer,
    pub show_when_smoke_off: bool,
    pub spread_duration: u32,
    pub start_scale: f64,
    pub valid: bool,
}

/// A collection of units moving and attacking together. The engine creates autonomous unit groups to attack polluted areas. The script can create and control such groups as well. Groups can accept commands in the same manner as regular units.
pub struct LuaUnitGroup {
    pub command: Option<Box<Command>>,
    pub distraction_command: Option<Box<Command>>,
    pub force: LuaForce,
    pub group_number: u32,
    pub is_script_driven: bool,
    pub members: Vec<LuaEntity>,
    pub object_name: String,
    pub position: MapPosition,
    pub state: GroupState,
    pub surface: LuaSurface,
    pub valid: bool,
}

/// Prototype of a virtual signal.
pub struct LuaVirtualSignalPrototype {
    pub localised_description: LocalisedString,
    pub localised_name: LocalisedString,
    pub name: String,
    pub object_name: String,
    pub order: String,
    pub special: bool,
    pub subgroup: LuaGroup,
    pub valid: bool,
}

/// Prototype of a void energy source.
pub struct LuaVoidEnergySourcePrototype {
    pub emissions: f64,
    pub object_name: String,
    pub render_no_network_icon: bool,
    pub render_no_power_icon: bool,
    pub valid: bool,
}

/// Control behavior for walls.
pub struct LuaWallControlBehavior {
    pub lua_control_behavior: Box<LuaControlBehavior>,
    pub circuit_condition: CircuitConditionDefinition,
    pub object_name: String,
    pub open_gate: bool,
    pub output_signal: SignalID,
    pub read_sensor: bool,
    pub valid: bool,
}
