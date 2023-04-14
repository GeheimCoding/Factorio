pub struct AchievementPrototypeFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct AdvancedMapGenSettings {
    pub difficulty_settings: DifficultySettings,
    pub enemy_evolution: EnemyEvolutionMapSettings,
    pub enemy_expansion: EnemyExpansionMapSettings,
    pub pollution: PollutionMapSettings,
}

pub struct Alert {
    pub icon: Option<SignalID>,
    pub message: Option<LocalisedString>,
    pub position: Option<MapPosition>,
    pub prototype: Option<LuaEntityPrototype>,
    pub target: Option<LuaEntity>,
    pub tick: u32,
}

pub enum Alignment {
    TopLeft,
    MiddleLeft,
    Left,
    BottomLeft,
    TopCenter,
    MiddleCenter,
    Center,
    BottomCenter,
    TopRight,
    Right,
    BottomRight,
}

pub struct AmmoType {
    pub action: Option<Vec<TriggerItem>>,
    pub category: String,
    pub clamp_position: Option<bool>,
    pub consumption_modifier: Option<f64>,
    pub cooldown_modifier: Option<f64>,
    pub energy_consumption: Option<f64>,
    pub range_modifier: Option<f64>,
    pub target_type: String,
}

pub enum Any {
    String(String),
    Boolean(bool),
    Number(f64),
    Table,
    LuaObject,
}

pub enum AnyBasic {
    String(String),
    Boolean(bool),
    Number(f64),
    Table,
}

pub struct ArithmeticCombinatorParameters {
    pub first_constant: Option<i32>,
    pub first_signal: Option<SignalID>,
    pub operation: Option<String>,
    pub output_signal: Option<SignalID>,
    pub second_constant: Option<i32>,
    pub second_signal: Option<SignalID>,
}

pub struct AttackParameterFluid {
    pub damage_modifier: f64,
    pub typ: String,
}

pub struct AttackParameters {
    pub ammo_categories: Option<Vec<String>>,
    pub ammo_consumption_modifier: f32,
    pub ammo_type: Option<AmmoType>,
    pub cooldown: f32,
    pub damage_modifier: f32,
    pub fire_penalty: f32,
    pub health_penalty: f32,
    pub min_attack_distance: f32,
    pub min_range: f32,
    pub movement_slow_down_cooldown: f32,
    pub movement_slow_down_factor: f64,
    pub range: f32,
    pub range_mode: String,
    pub rotate_penalty: f32,
    pub turn_range: f32,
    pub typ: String,
    pub warmup: u32,
}

pub struct AutoplaceControl {
    pub frequency: MapGenSize,
    pub richness: MapGenSize,
    pub size: MapGenSize,
}

pub struct AutoplaceSettings {
    pub settings: HashMap<String, AutoplaceControl>,
    pub treat_missing_as_default: bool,
}

pub struct AutoplaceSpecification {
    pub control: Option<String>,
    pub coverage: f64,
    pub default_enabled: bool,
    pub force: String,
    pub max_probability: f64,
    pub order: String,
    pub peaks: Option<Vec<AutoplaceSpecificationPeak>>,
    pub placement_density: u32,
    pub probability_expression: NoiseExpression,
    pub random_probability_penalty: f64,
    pub richness_base: f64,
    pub richness_expression: NoiseExpression,
    pub richness_multiplier: f64,
    pub richness_multiplier_distance_bonus: f64,
    pub sharpness: f64,
    pub starting_area_size: u32,
    pub tile_restriction: Option<Vec<AutoplaceSpecificationRestriction>>,
}

pub struct AutoplaceSpecificationPeak {
    pub aux_max_range: f64,
    pub aux_optimal: f64,
    pub aux_range: f64,
    pub aux_top_property_limit: f64,
    pub distance_max_range: f64,
    pub distance_optimal: f64,
    pub distance_range: f64,
    pub distance_top_property_limit: f64,
    pub elevation_max_range: f64,
    pub elevation_optimal: f64,
    pub elevation_range: f64,
    pub elevation_top_property_limit: f64,
    pub influence: f64,
    pub max_influence: f64,
    pub min_influence: f64,
    pub noise_persistence: f64,
    pub noise_layer: Option<String>,
    pub noise_octaves_difference: f64,
    pub richness_influence: f64,
    pub starting_area_weight_max_range: f64,
    pub starting_area_weight_optimal: f64,
    pub starting_area_weight_range: f64,
    pub starting_area_weight_top_property_limit: f64,
    pub temperature_max_range: f64,
    pub temperature_optimal: f64,
    pub temperature_range: f64,
    pub temperature_top_property_limit: f64,
    pub tier_from_start_max_range: f64,
    pub tier_from_start_optimal: f64,
    pub tier_from_start_range: f64,
    pub tier_from_start_top_property_limit: f64,
    pub water_max_range: f64,
    pub water_optimal: f64,
    pub water_range: f64,
    pub water_top_property_limit: f64,
}

pub struct AutoplaceSpecificationRestriction {
    pub first: Option<String>,
    pub second: Option<String>,
}

pub struct BeamTarget {
    pub entity: Option<LuaEntity>,
    pub position: Option<MapPosition>,
}

pub struct BlueprintEntity {
    pub connections: Option<BlueprintCircuitConnection>,
    pub control_behavior: Option<BlueprintControlBehavior>,
    pub direction: Option<Direction>,
    pub entity_number: u32,
    pub items: Option<HashMap<String, u32>>,
    pub name: String,
    pub position: MapPosition,
    pub schedule: Option<Vec<TrainScheduleRecord>>,
    pub tags: Option<Tags>,
}

pub struct BlueprintSignalIcon {
    pub index: u32,
    pub signal: SignalID,
}

pub struct BoundingBox {
    pub left_top: MapPosition,
    pub orientation: Option<RealOrientation>,
    pub right_bottom: MapPosition,
}

pub struct CapsuleAction {
    pub typ: String,
}

pub struct ChartTagSpec {
    pub icon: Option<SignalID>,
    pub last_user: Option<PlayerIdentification>,
    pub position: MapPosition,
    pub text: Option<String>,
}

pub struct ChunkPosition {
    pub x: i32,
    pub y: i32,
}

pub struct ChunkPositionAndArea {
    pub area: BoundingBox,
    pub x: i32,
    pub y: i32,
}

pub struct CircuitCondition {
    pub comparator: Option<ComparatorString>,
    pub constant: Option<i32>,
    pub first_signal: Option<SignalID>,
    pub second_signal: Option<SignalID>,
}

pub struct CircuitConditionDefinition {
    pub condition: CircuitCondition,
    pub fulfilled: Option<bool>,
}

pub struct CircuitConnectionDefinition {
    pub source_circuit_id: CircuitConnectorId,
    pub target_circuit_id: CircuitConnectorId,
    pub target_entity: LuaEntity,
    pub wire: WireType,
}

pub struct CircularParticleCreationSpecification {
    pub center: Vector,
    pub creation_distance: f64,
    pub creation_distance_orientation: f64,
    pub direction: f32,
    pub direction_deviation: f32,
    pub height: f32,
    pub height_deviation: f32,
    pub name: String,
    pub speed: f32,
    pub speed_deviation: f32,
    pub starting_frame_speed: f32,
    pub starting_frame_speed_deviation: f32,
    pub use_source_position: bool,
    pub vertical_speed: f32,
    pub vertical_speed_deviation: f32,
}

pub struct CircularProjectileCreationSpecification {
    pub field_0: RealOrientation,
    pub field_1: Vector,
}

pub enum CliffOrientation {
    WestToEast,
    NorthToSouth,
    EastToWest,
    SouthToNorth,
    WestToNorth,
    NorthToEast,
    EastToSouth,
    SouthToWest,
    WestToSouth,
    NorthToWest,
    EastToNorth,
    SouthToEast,
    WestToNone,
    NoneToEast,
    EastToNone,
    NoneToWest,
    NorthToNone,
    NoneToSouth,
    SouthToNone,
    NoneToNorth,
}

pub struct CliffPlacementSettings {
    pub cliff_elevation_0: f32,
    pub cliff_elevation_interval: f32,
    pub name: String,
    pub richness: MapGenSize,
}

type CollisionMask = HashSet<CollisionMaskLayer>;

pub enum CollisionMaskLayer {
    GroundTile,
    WaterTile,
    ResourceLayer,
    DoodadLayer,
    FloorLayer,
    ItemLayer,
    GhostLayer,
    ObjectLayer,
    PlayerLayer,
    TrainLayer,
    RailLayer,
    TransportBeltLayer,
    NotSetup,
}

pub enum CollisionMaskWithFlagsUnion {
    CollisionMaskLayer(CollisionMaskLayer),
    NotCollidingWithItself,
    ConsiderTileTransitions,
    CollidingWithTilesOnly,
}

type CollisionMaskWithFlags = HashSet<CollisionMaskWithFlagsUnion>;

pub struct Color {
    pub a: Option<f32>,
    pub b: Option<f32>,
    pub g: Option<f32>,
    pub r: Option<f32>,
}

pub struct ColorModifier {
    pub a: Option<f32>,
    pub b: Option<f32>,
    pub g: Option<f32>,
    pub r: Option<f32>,
}

pub struct Command {
    pub typ: Command,
}

pub enum ComparatorString {
    EqualTo,
    GreaterThan,
    LesserThan,
    GreaterThanOrEqualTo,
    LesserThanOrEqualTo,
    NotEqualTo,
}

pub struct ConfigurationChangedData {
    pub migration_applied: bool,
    pub mod_changes: HashMap<String, ModChangeData>,
    pub mod_startup_settings_changed: bool,
    pub new_version: Option<String>,
    pub old_version: Option<String>,
}

pub struct ConstantCombinatorParameters {
    pub count: i32,
    pub index: u32,
    pub signal: SignalID,
}

pub struct CraftingQueueItem {
    pub count: u32,
    pub index: u32,
    pub prerequisite: bool,
    pub recipe: String,
}

pub enum CursorBoxRenderType {
    Entity,
    NotAllowed,
    Electricity,
    Pair,
    Copy,
    TrainVisualization,
    Logistics,
    BlueprintSnapRectangle,
}

pub struct CustomCommandData {
    pub name: String,
    pub parameter: Option<String>,
    pub player_index: Option<u32>,
    pub tick: u32,
}

pub enum CutsceneWaypointTargetUnion {
    LuaEntity(LuaEntity),
    LuaUnitGroup(LuaUnitGroup),
}

pub struct CutsceneWaypoint {
    pub position: Option<MapPosition>,
    pub target: Option<CutsceneWaypointTargetUnion>,
    pub time_to_wait: u32,
    pub transition_time: u32,
    pub zoom: Option<f64>,
}

pub struct DeciderCombinatorParameters {
    pub comparator: Option<ComparatorString>,
    pub constant: Option<u32>,
    pub copy_count_from_input: Option<bool>,
    pub first_signal: Option<SignalID>,
    pub output_signal: Option<SignalID>,
    pub second_signal: Option<SignalID>,
}

pub struct Decorative {
    pub amount: u8,
    pub name: String,
    pub position: TilePosition,
}

pub struct DecorativePrototypeFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct DecorativeResult {
    pub amount: u32,
    pub decorative: LuaDecorativePrototype,
    pub position: TilePosition,
}

pub struct DifficultySettings {
    pub recipe_difficulty: DifficultySettingsRecipeDifficulty,
    pub research_queue_setting: String,
    pub technology_difficulty: DifficultySettingsTechnologyDifficulty,
    pub technology_price_multiplier: f64,
}

pub struct DisplayResolution {
    pub height: u32,
    pub width: u32,
}

pub struct DragTarget {
    pub target_circuit_id: Option<CircuitConnectorId>,
    pub target_entity: LuaEntity,
    pub target_wire_id: Option<WireConnectionId>,
}

pub struct EnemyEvolutionMapSettings {
    pub destroy_factor: f64,
    pub enabled: bool,
    pub pollution_factor: f64,
    pub time_factor: f64,
}

pub struct EnemyExpansionMapSettings {
    pub building_coefficient: f64,
    pub enabled: bool,
    pub enemy_building_influence_radius: u32,
    pub friendly_base_influence_radius: u32,
    pub max_colliding_tiles_coefficient: f64,
    pub max_expansion_cooldown: u32,
    pub max_expansion_distance: u32,
    pub min_expansion_cooldown: u32,
    pub neighbouring_base_chunk_coefficient: f64,
    pub neighbouring_chunk_coefficient: f64,
    pub other_base_coefficient: f64,
    pub settler_group_max_size: u32,
    pub settler_group_min_size: u32,
}

pub struct EntityPrototypeFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub enum EntityPrototypeFlagsUnion {
    NotRotatable,
    PlaceableNeutral,
    PlaceablePlayer,
    PlaceableEnemy,
    PlaceableOffGrid,
    PlayerCreation,
    BuildingDirection8Way,
    FilterDirections,
    FastReplaceableNoBuildWhileMoving,
    BreathsAir,
    NotRepairable,
    NotOnMap,
    NotDeconstructable,
    NotBlueprintable,
    Hidden,
    HideAltInfo,
    FastReplaceableNoCrossTypeWhileMoving,
    NoGapFillWhileBuilding,
    NotFlammable,
    NoAutomatedItemRemoval,
    NoAutomatedItemInsertion,
    NoCopyPaste,
    NotSelectableInGame,
    NotUpgradable,
    NotInKillStatistics,
    NotInMadeIn,
}

type EntityPrototypeFlags = HashSet<EntityPrototypeFlagsUnion>;

pub enum EntityPrototypeIdentification {
    LuaEntity(LuaEntity),
    LuaEntityPrototype(LuaEntityPrototype),
    String(String),
}

pub struct EquipmentPoint {
    pub x: u32,
    pub y: u32,
}

pub struct EquipmentPosition {
    pub x: i32,
    pub y: i32,
}

pub struct EquipmentPrototypeFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct EventData {
    pub mod_name: Option<String>,
    pub name: Events,
    pub tick: u32,
}

pub enum EventFilterUnion {
    LuaEntityClonedEventFilter(LuaEntityClonedEventFilter),
    LuaEntityDamagedEventFilter(LuaEntityDamagedEventFilter),
    LuaPlayerMinedEntityEventFilter(LuaPlayerMinedEntityEventFilter),
    LuaPreRobotMinedEntityEventFilter(LuaPreRobotMinedEntityEventFilter),
    LuaRobotBuiltEntityEventFilter(LuaRobotBuiltEntityEventFilter),
    LuaPostEntityDiedEventFilter(LuaPostEntityDiedEventFilter),
    LuaEntityDiedEventFilter(LuaEntityDiedEventFilter),
    LuaScriptRaisedReviveEventFilter(LuaScriptRaisedReviveEventFilter),
    LuaPrePlayerMinedEntityEventFilter(LuaPrePlayerMinedEntityEventFilter),
    LuaEntityMarkedForDeconstructionEventFilter(LuaEntityMarkedForDeconstructionEventFilter),
    LuaPreGhostDeconstructedEventFilter(LuaPreGhostDeconstructedEventFilter),
    LuaPreGhostUpgradedEventFilter(LuaPreGhostUpgradedEventFilter),
    LuaEntityDeconstructionCancelledEventFilter(LuaEntityDeconstructionCancelledEventFilter),
    LuaEntityMarkedForUpgradeEventFilter(LuaEntityMarkedForUpgradeEventFilter),
    LuaSectorScannedEventFilter(LuaSectorScannedEventFilter),
    LuaRobotMinedEntityEventFilter(LuaRobotMinedEntityEventFilter),
    LuaScriptRaisedDestroyEventFilter(LuaScriptRaisedDestroyEventFilter),
    LuaUpgradeCancelledEventFilter(LuaUpgradeCancelledEventFilter),
    LuaScriptRaisedBuiltEventFilter(LuaScriptRaisedBuiltEventFilter),
    LuaPlayerBuiltEntityEventFilter(LuaPlayerBuiltEntityEventFilter),
    LuaPlayerRepairedEntityEventFilter(LuaPlayerRepairedEntityEventFilter),
}

type EventFilter = Vec<EventFilterUnion>;

pub struct Fluid {
    pub amount: f64,
    pub name: String,
    pub temperature: Option<f64>,
}

pub struct FluidBoxConnection {
    pub max_underground_distance: Option<u32>,
    pub positions: Vec<Vector>,
    pub typ: String,
}

pub struct FluidBoxFilter {
    pub maximum_temperature: f64,
    pub minimum_temperature: f64,
    pub name: String,
}

pub struct FluidBoxFilterSpec {
    pub force: Option<bool>,
    pub maximum_temperature: Option<f64>,
    pub minimum_temperature: Option<f64>,
    pub name: String,
}

pub enum FluidIdentification {
    String(String),
    LuaFluidPrototype(LuaFluidPrototype),
    Fluid(Fluid),
}

pub struct FluidPrototypeFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub enum ForceCondition {
    All,
    Enemy,
    Ally,
    Friend,
    NotFriend,
    Same,
    NotSame,
}

pub enum ForceIdentification {
    Uint8(u8),
    String(String),
    LuaForce(LuaForce),
}

pub struct GameViewSettings {
    pub show_alert_gui: bool,
    pub show_controller_gui: bool,
    pub show_entity_info: bool,
    pub show_map_view_options: bool,
    pub show_minimap: bool,
    pub show_quickbar: bool,
    pub show_rail_block_visualisation: bool,
    pub show_research_info: bool,
    pub show_shortcut_bar: bool,
    pub show_side_menu: bool,
    pub update_entity_selection: bool,
}

pub struct GuiAnchor {
    pub gui: RelativeGuiType,
    pub name: Option<String>,
    pub names: Option<Vec<String>>,
    pub position: RelativeGuiPosition,
    pub typ: Option<String>,
}

pub struct GuiArrowSpecification {
    pub typ: String,
}

pub struct GuiLocation {
    pub x: i32,
    pub y: i32,
}

pub struct HeatConnection {
    pub direction: Direction,
    pub position: Vector,
}

pub struct HeatSetting {
    pub mode: Option<String>,
    pub temperature: Option<f64>,
}

pub struct InfinityInventoryFilter {
    pub count: Option<u32>,
    pub index: u32,
    pub mode: Option<String>,
    pub name: String,
}

pub struct InfinityPipeFilter {
    pub mode: Option<String>,
    pub name: String,
    pub percentage: Option<f64>,
    pub temperature: Option<f64>,
}

pub enum IngredientCatalystAmountUnion {
    Uint(u32),
    Double(f64),
}

pub struct Ingredient {
    pub amount: f64,
    pub catalyst_amount: Option<IngredientCatalystAmountUnion>,
    pub name: String,
    pub typ: String,
}

pub struct InserterCircuitConditions {
    pub circuit: Option<CircuitCondition>,
    pub logistics: Option<CircuitCondition>,
}

pub struct InventoryFilter {
    pub index: u32,
    pub name: String,
}

pub struct ItemPrototypeFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub enum ItemPrototypeFlagsUnion {
    DrawLogisticOverlay,
    Hidden,
    AlwaysShow,
    HideFromBonusGui,
    HideFromFuelTooltip,
    NotStackable,
    CanExtendInventory,
    PrimaryPlaceResult,
    ModOpenable,
    OnlyInCursor,
    Spawnable,
}

type ItemPrototypeFlags = HashSet<ItemPrototypeFlagsUnion>;

pub enum ItemPrototypeIdentification {
    LuaItemStack(LuaItemStack),
    LuaItemPrototype(LuaItemPrototype),
    String(String),
}

pub struct ItemStackDefinition {
    pub ammo: Option<f64>,
    pub count: Option<u32>,
    pub durability: Option<f64>,
    pub health: Option<f32>,
    pub name: String,
    pub tags: Option<Vec<String>>,
}

pub enum ItemStackIdentification {
    SimpleItemStack(SimpleItemStack),
    LuaItemStack(LuaItemStack),
}

pub struct ItemStackLocation {
    pub inventory: Inventory,
    pub slot: u32,
}

pub enum LocalisedString {
    String(String),
    Number(f64),
    Boolean(bool),
    LuaObject,
    Nil,
    Array,
}

pub struct LogisticFilter {
    pub count: u32,
    pub index: u32,
    pub name: String,
}

pub struct LogisticParameters {
    pub max: Option<u32>,
    pub min: Option<u32>,
    pub name: Option<String>,
}

pub struct Loot {
    pub count_max: f64,
    pub count_min: f64,
    pub item: String,
    pub probability: f64,
}

pub struct LuaEntityClonedEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct LuaEntityDamagedEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct LuaEntityDeconstructionCancelledEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct LuaEntityDiedEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct LuaEntityMarkedForDeconstructionEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct LuaEntityMarkedForUpgradeEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct LuaPlayerBuiltEntityEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct LuaPlayerMinedEntityEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct LuaPlayerRepairedEntityEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct LuaPostEntityDiedEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct LuaPreGhostDeconstructedEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct LuaPreGhostUpgradedEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct LuaPrePlayerMinedEntityEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct LuaPreRobotMinedEntityEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct LuaRobotBuiltEntityEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct LuaRobotMinedEntityEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct LuaScriptRaisedBuiltEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct LuaScriptRaisedDestroyEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct LuaScriptRaisedReviveEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct LuaScriptRaisedTeleportedEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct LuaSectorScannedEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct LuaUpgradeCancelledEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct MapAndDifficultySettings {
    pub difficulty_settings: DifficultySettings,
    pub enemy_evolution: EnemyEvolutionMapSettings,
    pub enemy_expansion: EnemyExpansionMapSettings,
    pub max_failed_behavior_count: u32,
    pub path_finder: PathFinderMapSettings,
    pub pollution: PollutionMapSettings,
    pub steering: SteeringMapSettings,
    pub unit_group: UnitGroupMapSettings,
}

pub struct MapExchangeStringData {
    pub map_gen_settings: MapGenSettings,
    pub map_settings: MapAndDifficultySettings,
}

pub struct MapGenPreset {
    pub advanced_settings: Option<AdvancedMapGenSettings>,
    pub basic_settings: Option<MapGenSettings>,
    pub default: Option<bool>,
    pub order: String,
}

pub struct MapGenSettings {
    pub autoplace_controls: HashMap<String, AutoplaceControl>,
    pub autoplace_settings: HashMap<String, AutoplaceSettings>,
    pub cliff_settings: CliffPlacementSettings,
    pub default_enable_all_autoplace_controls: bool,
    pub height: u32,
    pub peaceful_mode: bool,
    pub property_expression_names: HashMap<String, String>,
    pub seed: u32,
    pub starting_area: MapGenSize,
    pub starting_points: Vec<MapPosition>,
    pub terrain_segmentation: MapGenSize,
    pub water: MapGenSize,
    pub width: u32,
}

pub enum MapGenSize {
    Float(f32),
    None,
    VeryLow,
    VerySmall,
    VeryPoor,
    Low,
    Small,
    Poor,
    Normal,
    Medium,
    Regular,
    High,
    Big,
    Good,
    VeryHigh,
    VeryBig,
    VeryGood,
}

pub struct MapPosition {
    pub x: f64,
    pub y: f64,
}

pub struct MapSettings {
    pub enemy_evolution: EnemyEvolutionMapSettings,
    pub enemy_expansion: EnemyExpansionMapSettings,
    pub max_failed_behavior_count: u32,
    pub path_finder: PathFinderMapSettings,
    pub pollution: PollutionMapSettings,
    pub steering: SteeringMapSettings,
    pub unit_group: UnitGroupMapSettings,
}

pub struct MapViewSettings {
    pub show_electric_network: Option<bool>,
    pub show_logistic_network: Option<bool>,
    pub show_networkless_logistic_members: Option<bool>,
    pub show_non_standard_map_info: Option<bool>,
    pub show_player_names: Option<bool>,
    pub show_pollution: Option<bool>,
    pub show_train_station_names: Option<bool>,
    pub show_turret_range: Option<bool>,
}

pub struct ModChangeData {
    pub new_version: String,
    pub old_version: String,
}

pub enum ModSettingValueUnion {
    Int(i32),
    Double(f64),
    Boolean(bool),
    String(String),
    Color(Color),
}

pub struct ModSetting {
    pub value: ModSettingValueUnion,
}

pub struct ModSettingPrototypeFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct ModuleEffectValue {
    pub bonus: f32,
}

pub struct ModuleEffects {
    pub consumption: Option<ModuleEffectValue>,
    pub pollution: Option<ModuleEffectValue>,
    pub productivity: Option<ModuleEffectValue>,
    pub speed: Option<ModuleEffectValue>,
}

pub enum MouseButtonFlagsUnion {
    Left,
    Right,
    Middle,
    Button4,
    Button5,
    Button6,
    Button7,
    Button8,
    Button9,
}

type MouseButtonFlags = HashSet<MouseButtonFlagsUnion>;

pub struct NoiseExpression {
    pub typ: String,
}

pub struct NthTickEventData {
    pub nth_tick: u32,
    pub tick: u32,
}

pub struct Offer {
    pub offer: TechnologyModifier,
    pub price: Vec<Ingredient>,
}

pub struct OldTileAndPosition {
    pub old_tile: LuaTilePrototype,
    pub position: TilePosition,
}

pub struct PathFinderMapSettings {
    pub cache_accept_path_end_distance_ratio: f64,
    pub cache_accept_path_start_distance_ratio: f64,
    pub cache_max_connect_to_cache_steps_multiplier: u32,
    pub cache_path_end_distance_rating_multiplier: f64,
    pub cache_path_start_distance_rating_multiplier: f64,
    pub direct_distance_to_consider_short_request: u32,
    pub enemy_with_different_destination_collision_penalty: f64,
    pub extended_collision_penalty: f64,
    pub fwd2bwd_ratio: u32,
    pub general_entity_collision_penalty: f64,
    pub general_entity_subsequent_collision_penalty: f64,
    pub goal_pressure_ratio: f64,
    pub ignore_moving_enemy_collision_distance: f64,
    pub long_cache_min_cacheable_distance: f64,
    pub long_cache_size: u32,
    pub max_clients_to_accept_any_new_request: u32,
    pub max_clients_to_accept_short_new_request: u32,
    pub max_steps_worked_per_tick: f64,
    pub max_work_done_per_tick: u32,
    pub min_steps_to_check_path_find_termination: u32,
    pub negative_cache_accept_path_end_distance_ratio: f64,
    pub negative_cache_accept_path_start_distance_ratio: f64,
    pub negative_path_cache_delay_interval: u32,
    pub overload_levels: Vec<u32>,
    pub overload_multipliers: Vec<f64>,
    pub short_cache_min_algo_steps_to_cache: u32,
    pub short_cache_min_cacheable_distance: f64,
    pub short_cache_size: u32,
    pub short_request_max_steps: u32,
    pub short_request_ratio: f64,
    pub stale_enemy_with_same_destination_collision_penalty: f64,
    pub start_to_goal_cost_multiplier_to_terminate_path_find: f64,
    pub use_path_cache: bool,
}

pub struct PathfinderFlags {
    pub allow_destroy_friendly_entities: Option<bool>,
    pub allow_paths_through_own_entities: Option<bool>,
    pub cache: Option<bool>,
    pub low_priority: Option<bool>,
    pub no_break: Option<bool>,
    pub prefer_straight_paths: Option<bool>,
}

pub struct PathfinderWaypoint {
    pub needs_destroy_to_reach: bool,
    pub position: MapPosition,
}

pub struct PlaceAsTileResult {
    pub condition: CollisionMask,
    pub condition_size: u32,
    pub result: LuaTilePrototype,
}

pub enum PlayerIdentification {
    Uint(u32),
    String(String),
    LuaPlayer(LuaPlayer),
}

pub struct PollutionMapSettings {
    pub ageing: f64,
    pub diffusion_ratio: f64,
    pub enabled: bool,
    pub enemy_attack_pollution_consumption_modifier: f64,
    pub expected_max_per_chunk: f64,
    pub max_pollution_to_restore_trees: f64,
    pub min_pollution_to_damage_trees: f64,
    pub min_to_diffuse: f64,
    pub min_to_show_per_chunk: f64,
    pub pollution_per_tree_damage: f64,
    pub pollution_restored_per_tree_damage: f64,
    pub pollution_with_max_forest_damage: f64,
}

pub enum ProductAmountMaxUnion {
    Uint(u32),
    Double(f64),
}

pub enum ProductAmountMinUnion {
    Uint(u32),
    Double(f64),
}

pub enum ProductCatalystAmountUnion {
    Uint(u32),
    Double(f64),
}

pub struct Product {
    pub amount: Option<f64>,
    pub amount_max: Option<ProductAmountMaxUnion>,
    pub amount_min: Option<ProductAmountMinUnion>,
    pub catalyst_amount: Option<ProductCatalystAmountUnion>,
    pub name: String,
    pub probability: Option<f64>,
    pub typ: String,
}

pub struct ProgrammableSpeakerAlertParameters {
    pub alert_message: String,
    pub icon_signal_id: SignalID,
    pub show_alert: bool,
    pub show_on_map: bool,
}

pub struct ProgrammableSpeakerCircuitParameters {
    pub instrument_id: u32,
    pub note_id: u32,
    pub signal_value_is_pitch: bool,
}

pub struct ProgrammableSpeakerInstrument {
    pub name: String,
    pub notes: Vec<String>,
}

pub struct ProgrammableSpeakerParameters {
    pub allow_polyphony: bool,
    pub playback_globally: bool,
    pub playback_volume: f64,
}

pub enum PrototypeFilterUnion {
    ItemPrototypeFilter(ItemPrototypeFilter),
    TilePrototypeFilter(TilePrototypeFilter),
    EntityPrototypeFilter(EntityPrototypeFilter),
    FluidPrototypeFilter(FluidPrototypeFilter),
    RecipePrototypeFilter(RecipePrototypeFilter),
    DecorativePrototypeFilter(DecorativePrototypeFilter),
    AchievementPrototypeFilter(AchievementPrototypeFilter),
    EquipmentPrototypeFilter(EquipmentPrototypeFilter),
    TechnologyPrototypeFilter(TechnologyPrototypeFilter),
}

type PrototypeFilter = Vec<PrototypeFilterUnion>;

pub struct PrototypeHistory {
    pub changed: Vec<String>,
    pub created: String,
}

type RealOrientation = f32;

pub struct RecipePrototypeFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub enum RenderLayer {
    String(String),
    WaterTile,
    GroundTile,
    TileTransition,
    Decals,
    LowerRadiusVisualization,
    RadiusVisualization,
    TransportBeltIntegration,
    Resource,
    BuildingSmoke,
    Decorative,
    GroundPatch,
    GroundPatchHigher,
    GroundPatchHigher2,
    Remnants,
    Floor,
    TransportBelt,
    TransportBeltEndings,
    FloorMechanicsUnderCorpse,
    Corpse,
    FloorMechanics,
    Item,
    LowerObject,
    TransportBeltCircuitConnector,
    LowerObjectAboveShadow,
    Object,
    HigherObjectUnder,
    HigherObjectAbove,
    ItemInInserterHand,
    Wires,
    WiresAbove,
    EntityInfoIcon,
    EntityInfoIconAbove,
    Explosion,
    Projectile,
    Smoke,
    AirObject,
    AirEntityInfoIcon,
    LightEffect,
    SelectionBox,
    HigherSelectionBox,
    CollisionSelectionBox,
    Arrow,
    Cursor,
}

pub struct Resistance {
    pub decrease: f32,
    pub percent: f32,
}

pub struct RidingState {
    pub acceleration: RidingAcceleration,
    pub direction: RidingDirection,
}

pub struct ScriptArea {
    pub area: BoundingBox,
    pub color: Color,
    pub id: u32,
    pub name: String,
}

pub struct ScriptPosition {
    pub color: Color,
    pub id: u32,
    pub name: String,
    pub position: MapPosition,
}

pub struct ScriptRenderTarget {
    pub entity: Option<LuaEntity>,
    pub entity_offset: Option<Vector>,
    pub position: Option<MapPosition>,
}

pub enum ScriptRenderVertexTargetTargetUnion {
    MapPosition(MapPosition),
    LuaEntity(LuaEntity),
}

pub struct ScriptRenderVertexTarget {
    pub target: ScriptRenderVertexTargetTargetUnion,
    pub target_offset: Option<Vector>,
}

pub struct SelectedPrototypeData {
    pub base_type: String,
    pub derived_type: String,
    pub name: String,
}

pub enum SelectionModeFlagsUnion {
    Blueprint,
    Deconstruct,
    CancelDeconstruct,
    Items,
    Trees,
    BuildableType,
    Nothing,
    ItemsToPlace,
    AnyEntity,
    AnyTile,
    SameForce,
    NotSameForce,
    Friend,
    Enemy,
    Upgrade,
    CancelUpgrade,
    Downgrade,
    EntityWithHealth,
    EntityWithForce,
    IsMilitaryTarget,
    EntityWithOwner,
    AvoidRollingStock,
    EntityGhost,
    TileGhost,
}

type SelectionModeFlags = HashSet<SelectionModeFlagsUnion>;

pub struct Signal {
    pub count: i32,
    pub signal: SignalID,
}

pub struct SignalID {
    pub name: Option<String>,
    pub typ: String,
}

pub enum SimpleItemStack {
    String(String),
    ItemStackDefinition(ItemStackDefinition),
}

pub struct SmokeSource {
    pub deviation: Option<MapPosition>,
    pub east_position: Option<Vector>,
    pub frequency: f64,
    pub height: f32,
    pub height_deviation: f32,
    pub name: String,
    pub north_position: Option<Vector>,
    pub offset: f64,
    pub position: Option<Vector>,
    pub slow_down_factor: u8,
    pub south_position: Option<Vector>,
    pub starting_frame: u16,
    pub starting_frame_deviation: f64,
    pub starting_frame_speed: u16,
    pub starting_frame_speed_deviation: f64,
    pub starting_vertical_speed: f32,
    pub starting_vertical_speed_deviation: f32,
    pub vertical_speed_slowdown: f32,
    pub west_position: Option<Vector>,
}

type SoundPath = String;

pub enum SoundType {
    GameEffect,
    GuiEffect,
    Ambient,
    Environment,
    Walking,
    Alert,
    Wind,
}

pub struct SpawnPointDefinition {
    pub evolution_factor: f64,
    pub weight: f64,
}

type SpritePath = String;

pub struct SteeringMapSetting {
    pub force_unit_fuzzy_goto_behavior: bool,
    pub radius: f64,
    pub separation_factor: f64,
    pub separation_force: f64,
}

pub struct SteeringMapSettings {
    pub default: SteeringMapSetting,
    pub moving: SteeringMapSetting,
}

pub enum SurfaceIdentification {
    Uint(u32),
    String(String),
    LuaSurface(LuaSurface),
}

pub struct TabAndContent {
    pub content: LuaGuiElement,
    pub tab: LuaGuiElement,
}

type Tags = HashMap<String, AnyBasic>;

pub enum TechnologyIdentification {
    String(String),
    LuaTechnology(LuaTechnology),
    LuaTechnologyPrototype(LuaTechnologyPrototype),
}

pub struct TechnologyModifier {
    pub typ: String,
}

pub struct TechnologyPrototypeFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct Tile {
    pub name: String,
    pub position: TilePosition,
}

pub struct TilePosition {
    pub x: i32,
    pub y: i32,
}

pub struct TilePrototypeFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
}

pub struct TrainSchedule {
    pub current: u32,
    pub records: Vec<TrainScheduleRecord>,
}

pub struct TrainScheduleRecord {
    pub rail: Option<LuaEntity>,
    pub rail_direction: Option<RailDirection>,
    pub station: Option<String>,
    pub temporary: Option<bool>,
    pub wait_conditions: Option<Vec<WaitCondition>>,
}

pub struct TriggerDelivery {
    pub source_effects: Vec<TriggerEffectItem>,
    pub target_effects: Vec<TriggerEffectItem>,
    pub typ: String,
}

pub struct TriggerEffectItem {
    pub affects_target: bool,
    pub repeat_count: u32,
    pub show_in_tooltip: bool,
    pub typ: String,
}

pub struct TriggerItem {
    pub action_delivery: Option<Vec<TriggerDelivery>>,
    pub collision_mask: CollisionMask,
    pub entity_flags: Option<EntityPrototypeFlags>,
    pub force: ForceCondition,
    pub ignore_collision_condition: bool,
    pub repeat_count: u32,
    pub trigger_target_mask: TriggerTargetMask,
    pub typ: String,
}

type TriggerTargetMask = HashMap<String, bool>;

pub struct UnitGroupMapSettings {
    pub max_gathering_unit_groups: u32,
    pub max_group_gathering_time: u32,
    pub max_group_member_fallback_factor: f64,
    pub max_group_radius: f64,
    pub max_group_slowdown_factor: f64,
    pub max_member_slowdown_when_ahead: f64,
    pub max_member_speedup_when_behind: f64,
    pub max_unit_group_size: u32,
    pub max_wait_time_for_late_members: u32,
    pub member_disown_distance: f64,
    pub min_group_gathering_time: u32,
    pub min_group_radius: f64,
    pub tick_tolerance_when_member_arrives: u32,
}

pub struct UnitSpawnDefinition {
    pub spawn_points: Vec<SpawnPointDefinition>,
    pub unit: String,
}

pub struct UpgradeFilter {
    pub name: Option<String>,
    pub typ: String,
}

pub struct Vector {
    pub x: f32,
    pub y: f32,
}

pub struct VehicleAutomaticTargetingParameters {
    pub auto_target_with_gunner: bool,
    pub auto_target_without_gunner: bool,
}

pub struct WaitCondition {
    pub compare_type: String,
    pub condition: Option<CircuitCondition>,
    pub ticks: Option<u32>,
    pub typ: String,
}

pub struct WireConnectionDefinition {
    pub source_circuit_id: Option<CircuitConnectorId>,
    pub source_wire_id: Option<WireConnectionId>,
    pub target_circuit_id: Option<CircuitConnectorId>,
    pub target_entity: LuaEntity,
    pub target_wire_id: Option<WireConnectionId>,
    pub wire: WireType,
}
