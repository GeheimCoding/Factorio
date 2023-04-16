pub struct AchievementPrototypeFilterAttributesType {
    pub typ: AchievementPrototypeFilterAttributesTypeUnion,
}

pub enum AchievementPrototypeFilterAttributes {
    Type(AchievementPrototypeFilterAttributesType),
}

pub struct AchievementPrototypeFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<AchievementPrototypeFilterAttributes>,
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
pub struct AttackParametersAttributesProjectile {
    pub projectile_center: Vector,
    pub projectile_creation_distance: f32,
    pub projectile_creation_parameters: Option<Vec<CircularProjectileCreationSpecification>>,
    pub projectile_orientation_offset: f32,
    pub shell_particle: Option<CircularParticleCreationSpecification>,
}

pub struct AttackParametersAttributesStream {
    pub fluid_consumption: f32,
    pub fluids: Option<Vec<AttackParameterFluid>>,
    pub gun_barrel_length: f32,
    pub gun_center_shift: HashMap<String, Vector>,
    pub projectile_creation_parameters: Option<Vec<CircularProjectileCreationSpecification>>,
}

pub enum AttackParametersAttributes {
    Projectile(AttackParametersAttributesProjectile),
    Stream(AttackParametersAttributesStream),
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
    pub attributes: Option<AttackParametersAttributes>,
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
pub struct CapsuleActionAttributesArtilleryRemote {
    pub flare: String,
}

pub struct CapsuleActionAttributesDestroyCliffs {
    pub attack_parameters: AttackParameters,
    pub radius: f32,
    pub timeout: u32,
}

pub struct CapsuleActionAttributesEquipmentRemote {
    pub equipment: String,
}

pub struct CapsuleActionAttributesThrow {
    pub attack_parameters: AttackParameters,
    pub uses_stack: bool,
}

pub struct CapsuleActionAttributesUseOnSelf {
    pub attack_parameters: AttackParameters,
}

pub enum CapsuleActionAttributes {
    ArtilleryRemote(CapsuleActionAttributesArtilleryRemote),
    DestroyCliffs(CapsuleActionAttributesDestroyCliffs),
    EquipmentRemote(CapsuleActionAttributesEquipmentRemote),
    Throw(CapsuleActionAttributesThrow),
    UseOnSelf(CapsuleActionAttributesUseOnSelf),
}

pub struct CapsuleAction {
    pub typ: String,
    pub attributes: Option<CapsuleActionAttributes>,
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
pub struct CommandAttributesDefinesCommandAttack {
    pub distraction: Option<Distraction>,
    pub target: LuaEntity,
}

pub struct CommandAttributesDefinesCommandAttackArea {
    pub destination: MapPosition,
    pub distraction: Option<Distraction>,
    pub radius: f64,
}

pub struct CommandAttributesDefinesCommandBuildBase {
    pub destination: MapPosition,
    pub distraction: Option<Distraction>,
    pub ignore_planner: Option<bool>,
}

pub struct CommandAttributesDefinesCommandCompound {
    pub commands: Vec<Command>,
    pub structure_type: CompoundCommand,
}

pub struct CommandAttributesDefinesCommandFlee {
    pub distraction: Option<Distraction>,
    pub from: LuaEntity,
}

pub struct CommandAttributesDefinesCommandGoToLocation {
    pub destination: Option<MapPosition>,
    pub destination_entity: Option<LuaEntity>,
    pub distraction: Option<Distraction>,
    pub pathfind_flags: Option<PathfinderFlags>,
    pub radius: Option<f64>,
}

pub struct CommandAttributesDefinesCommandGroup {
    pub distraction: Option<Distraction>,
    pub group: LuaUnitGroup,
    pub use_group_distraction: Option<bool>,
}

pub struct CommandAttributesDefinesCommandStop {
    pub distraction: Option<Distraction>,
    pub ticks_to_wait: Option<u32>,
}

pub struct CommandAttributesDefinesCommandWander {
    pub distraction: Option<Distraction>,
    pub radius: Option<f64>,
    pub ticks_to_wait: Option<u32>,
    pub wander_in_group: Option<bool>,
}

pub enum CommandAttributes {
    DefinesCommandAttack(CommandAttributesDefinesCommandAttack),
    DefinesCommandAttackArea(CommandAttributesDefinesCommandAttackArea),
    DefinesCommandBuildBase(CommandAttributesDefinesCommandBuildBase),
    DefinesCommandCompound(CommandAttributesDefinesCommandCompound),
    DefinesCommandFlee(CommandAttributesDefinesCommandFlee),
    DefinesCommandGoToLocation(CommandAttributesDefinesCommandGoToLocation),
    DefinesCommandGroup(CommandAttributesDefinesCommandGroup),
    DefinesCommandStop(CommandAttributesDefinesCommandStop),
    DefinesCommandWander(CommandAttributesDefinesCommandWander),
}

pub struct Command {
    pub typ: Command,
    pub attributes: Option<CommandAttributes>,
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
pub struct DecorativePrototypeFilterAttributesCollisionMask {
    pub mask: DecorativePrototypeFilterAttributesMaskUnion,
    pub mask_mode: String,
}

pub enum DecorativePrototypeFilterAttributes {
    CollisionMask(DecorativePrototypeFilterAttributesCollisionMask),
}

pub struct DecorativePrototypeFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<DecorativePrototypeFilterAttributes>,
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
pub struct EntityPrototypeFilterAttributesBuildBaseEvolutionRequirement {
    pub comparison: ComparatorString,
    pub value: f64,
}

pub struct EntityPrototypeFilterAttributesCollisionMask {
    pub mask: EntityPrototypeFilterAttributesMaskUnion,
    pub mask_mode: String,
}

pub struct EntityPrototypeFilterAttributesCraftingCategory {
    pub crafting_category: String,
}

pub struct EntityPrototypeFilterAttributesEmissions {
    pub comparison: ComparatorString,
    pub value: f64,
}

pub struct EntityPrototypeFilterAttributesFlag {
    pub flag: String,
}

pub struct EntityPrototypeFilterAttributesName {
    pub name: EntityPrototypeFilterAttributesNameUnion,
}

pub struct EntityPrototypeFilterAttributesSelectionPriority {
    pub comparison: ComparatorString,
    pub value: u8,
}

pub struct EntityPrototypeFilterAttributesType {
    pub typ: EntityPrototypeFilterAttributesTypeUnion,
}

pub enum EntityPrototypeFilterAttributes {
    BuildBaseEvolutionRequirement(EntityPrototypeFilterAttributesBuildBaseEvolutionRequirement),
    CollisionMask(EntityPrototypeFilterAttributesCollisionMask),
    CraftingCategory(EntityPrototypeFilterAttributesCraftingCategory),
    Emissions(EntityPrototypeFilterAttributesEmissions),
    Flag(EntityPrototypeFilterAttributesFlag),
    Name(EntityPrototypeFilterAttributesName),
    SelectionPriority(EntityPrototypeFilterAttributesSelectionPriority),
    Type(EntityPrototypeFilterAttributesType),
}

pub struct EntityPrototypeFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<EntityPrototypeFilterAttributes>,
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
pub struct EquipmentPrototypeFilterAttributesType {
    pub typ: EquipmentPrototypeFilterAttributesTypeUnion,
}

pub enum EquipmentPrototypeFilterAttributes {
    Type(EquipmentPrototypeFilterAttributesType),
}

pub struct EquipmentPrototypeFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<EquipmentPrototypeFilterAttributes>,
}
pub struct EventData {
    pub mod_name: Option<String>,
    pub name: Events,
    pub tick: u32,
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
pub struct FluidPrototypeFilterAttributesDefaultTemperature {
    pub comparison: ComparatorString,
    pub value: f64,
}

pub struct FluidPrototypeFilterAttributesEmissionsMultiplier {
    pub comparison: ComparatorString,
    pub value: f64,
}

pub struct FluidPrototypeFilterAttributesFuelValue {
    pub comparison: ComparatorString,
    pub value: f64,
}

pub struct FluidPrototypeFilterAttributesGasTemperature {
    pub comparison: ComparatorString,
    pub value: f64,
}

pub struct FluidPrototypeFilterAttributesHeatCapacity {
    pub comparison: ComparatorString,
    pub value: f64,
}

pub struct FluidPrototypeFilterAttributesMaxTemperature {
    pub comparison: ComparatorString,
    pub value: f64,
}

pub struct FluidPrototypeFilterAttributesName {
    pub name: FluidPrototypeFilterAttributesNameUnion,
}

pub struct FluidPrototypeFilterAttributesSubgroup {
    pub subgroup: String,
}

pub enum FluidPrototypeFilterAttributes {
    DefaultTemperature(FluidPrototypeFilterAttributesDefaultTemperature),
    EmissionsMultiplier(FluidPrototypeFilterAttributesEmissionsMultiplier),
    FuelValue(FluidPrototypeFilterAttributesFuelValue),
    GasTemperature(FluidPrototypeFilterAttributesGasTemperature),
    HeatCapacity(FluidPrototypeFilterAttributesHeatCapacity),
    MaxTemperature(FluidPrototypeFilterAttributesMaxTemperature),
    Name(FluidPrototypeFilterAttributesName),
    Subgroup(FluidPrototypeFilterAttributesSubgroup),
}

pub struct FluidPrototypeFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<FluidPrototypeFilterAttributes>,
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
pub struct GuiArrowSpecificationAttributesCraftingQueue {
    pub crafting_queueindex: u32,
}

pub struct GuiArrowSpecificationAttributesEntity {
    pub entity: LuaEntity,
}

pub struct GuiArrowSpecificationAttributesItemStack {
    pub inventory_index: Inventory,
    pub item_stack_index: u32,
    pub source: String,
}

pub struct GuiArrowSpecificationAttributesPosition {
    pub position: MapPosition,
}

pub enum GuiArrowSpecificationAttributes {
    CraftingQueue(GuiArrowSpecificationAttributesCraftingQueue),
    Entity(GuiArrowSpecificationAttributesEntity),
    ItemStack(GuiArrowSpecificationAttributesItemStack),
    Position(GuiArrowSpecificationAttributesPosition),
}

pub struct GuiArrowSpecification {
    pub typ: String,
    pub attributes: Option<GuiArrowSpecificationAttributes>,
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
pub struct IngredientAttributesFluid {
    pub maximum_temperature: Option<f64>,
    pub minimum_temperature: Option<f64>,
}

pub enum IngredientAttributes {
    Fluid(IngredientAttributesFluid),
}

pub struct Ingredient {
    pub amount: f64,
    pub catalyst_amount: Option<IngredientCatalystAmountUnion>,
    pub name: String,
    pub typ: String,
    pub attributes: Option<IngredientAttributes>,
}
pub struct InserterCircuitConditions {
    pub circuit: Option<CircuitCondition>,
    pub logistics: Option<CircuitCondition>,
}
pub struct InventoryFilter {
    pub index: u32,
    pub name: String,
}
pub struct ItemPrototypeFilterAttributesBurntResult {
    pub elem_filters: Option<Vec<ItemPrototypeFilter>>,
}

pub struct ItemPrototypeFilterAttributesDefaultRequestAmount {
    pub comparison: ComparatorString,
    pub value: u32,
}

pub struct ItemPrototypeFilterAttributesFlag {
    pub flag: String,
}

pub struct ItemPrototypeFilterAttributesFuelAccelerationMultiplier {
    pub comparison: ComparatorString,
    pub value: f64,
}

pub struct ItemPrototypeFilterAttributesFuelCategory {
    pub fuel_category: String,
}

pub struct ItemPrototypeFilterAttributesFuelEmissionsMultiplier {
    pub comparison: ComparatorString,
    pub value: f64,
}

pub struct ItemPrototypeFilterAttributesFuelTopSpeedMultiplier {
    pub comparison: ComparatorString,
    pub value: f64,
}

pub struct ItemPrototypeFilterAttributesFuelValue {
    pub comparison: ComparatorString,
    pub value: f64,
}

pub struct ItemPrototypeFilterAttributesName {
    pub name: ItemPrototypeFilterAttributesNameUnion,
}

pub struct ItemPrototypeFilterAttributesPlaceAsTile {
    pub elem_filters: Option<Vec<TilePrototypeFilter>>,
}

pub struct ItemPrototypeFilterAttributesPlaceResult {
    pub elem_filters: Option<Vec<EntityPrototypeFilter>>,
}

pub struct ItemPrototypeFilterAttributesPlacedAsEquipmentResult {
    pub elem_filters: Option<Vec<EquipmentPrototypeFilter>>,
}

pub struct ItemPrototypeFilterAttributesStackSize {
    pub comparison: ComparatorString,
    pub value: u32,
}

pub struct ItemPrototypeFilterAttributesSubgroup {
    pub subgroup: String,
}

pub struct ItemPrototypeFilterAttributesType {
    pub typ: ItemPrototypeFilterAttributesTypeUnion,
}

pub struct ItemPrototypeFilterAttributesWireCount {
    pub comparison: ComparatorString,
    pub value: u32,
}

pub enum ItemPrototypeFilterAttributes {
    BurntResult(ItemPrototypeFilterAttributesBurntResult),
    DefaultRequestAmount(ItemPrototypeFilterAttributesDefaultRequestAmount),
    Flag(ItemPrototypeFilterAttributesFlag),
    FuelAccelerationMultiplier(ItemPrototypeFilterAttributesFuelAccelerationMultiplier),
    FuelCategory(ItemPrototypeFilterAttributesFuelCategory),
    FuelEmissionsMultiplier(ItemPrototypeFilterAttributesFuelEmissionsMultiplier),
    FuelTopSpeedMultiplier(ItemPrototypeFilterAttributesFuelTopSpeedMultiplier),
    FuelValue(ItemPrototypeFilterAttributesFuelValue),
    Name(ItemPrototypeFilterAttributesName),
    PlaceAsTile(ItemPrototypeFilterAttributesPlaceAsTile),
    PlaceResult(ItemPrototypeFilterAttributesPlaceResult),
    PlacedAsEquipmentResult(ItemPrototypeFilterAttributesPlacedAsEquipmentResult),
    StackSize(ItemPrototypeFilterAttributesStackSize),
    Subgroup(ItemPrototypeFilterAttributesSubgroup),
    Type(ItemPrototypeFilterAttributesType),
    WireCount(ItemPrototypeFilterAttributesWireCount),
}

pub struct ItemPrototypeFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<ItemPrototypeFilterAttributes>,
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
    Array(Vec<LocalisedStringUnion>),
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
pub struct LuaEntityClonedEventFilterAttributesGhostName {
    pub name: String,
}

pub struct LuaEntityClonedEventFilterAttributesGhostType {
    pub typ: String,
}

pub struct LuaEntityClonedEventFilterAttributesName {
    pub name: String,
}

pub struct LuaEntityClonedEventFilterAttributesType {
    pub typ: String,
}

pub enum LuaEntityClonedEventFilterAttributes {
    GhostName(LuaEntityClonedEventFilterAttributesGhostName),
    GhostType(LuaEntityClonedEventFilterAttributesGhostType),
    Name(LuaEntityClonedEventFilterAttributesName),
    Type(LuaEntityClonedEventFilterAttributesType),
}

pub struct LuaEntityClonedEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<LuaEntityClonedEventFilterAttributes>,
}
pub struct LuaEntityDamagedEventFilterAttributesDamageType {
    pub typ: String,
}

pub struct LuaEntityDamagedEventFilterAttributesFinalDamageAmount {
    pub comparison: ComparatorString,
    pub value: f32,
}

pub struct LuaEntityDamagedEventFilterAttributesFinalHealth {
    pub comparison: ComparatorString,
    pub value: f32,
}

pub struct LuaEntityDamagedEventFilterAttributesGhostName {
    pub name: String,
}

pub struct LuaEntityDamagedEventFilterAttributesGhostType {
    pub typ: String,
}

pub struct LuaEntityDamagedEventFilterAttributesName {
    pub name: String,
}

pub struct LuaEntityDamagedEventFilterAttributesOriginalDamageAmount {
    pub comparison: ComparatorString,
    pub value: f32,
}

pub struct LuaEntityDamagedEventFilterAttributesType {
    pub typ: String,
}

pub enum LuaEntityDamagedEventFilterAttributes {
    DamageType(LuaEntityDamagedEventFilterAttributesDamageType),
    FinalDamageAmount(LuaEntityDamagedEventFilterAttributesFinalDamageAmount),
    FinalHealth(LuaEntityDamagedEventFilterAttributesFinalHealth),
    GhostName(LuaEntityDamagedEventFilterAttributesGhostName),
    GhostType(LuaEntityDamagedEventFilterAttributesGhostType),
    Name(LuaEntityDamagedEventFilterAttributesName),
    OriginalDamageAmount(LuaEntityDamagedEventFilterAttributesOriginalDamageAmount),
    Type(LuaEntityDamagedEventFilterAttributesType),
}

pub struct LuaEntityDamagedEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<LuaEntityDamagedEventFilterAttributes>,
}
pub struct LuaEntityDeconstructionCancelledEventFilterAttributesGhostName {
    pub name: String,
}

pub struct LuaEntityDeconstructionCancelledEventFilterAttributesGhostType {
    pub typ: String,
}

pub struct LuaEntityDeconstructionCancelledEventFilterAttributesName {
    pub name: String,
}

pub struct LuaEntityDeconstructionCancelledEventFilterAttributesType {
    pub typ: String,
}

pub enum LuaEntityDeconstructionCancelledEventFilterAttributes {
    GhostName(LuaEntityDeconstructionCancelledEventFilterAttributesGhostName),
    GhostType(LuaEntityDeconstructionCancelledEventFilterAttributesGhostType),
    Name(LuaEntityDeconstructionCancelledEventFilterAttributesName),
    Type(LuaEntityDeconstructionCancelledEventFilterAttributesType),
}

pub struct LuaEntityDeconstructionCancelledEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<LuaEntityDeconstructionCancelledEventFilterAttributes>,
}
pub struct LuaEntityDiedEventFilterAttributesGhostName {
    pub name: String,
}

pub struct LuaEntityDiedEventFilterAttributesGhostType {
    pub typ: String,
}

pub struct LuaEntityDiedEventFilterAttributesName {
    pub name: String,
}

pub struct LuaEntityDiedEventFilterAttributesType {
    pub typ: String,
}

pub enum LuaEntityDiedEventFilterAttributes {
    GhostName(LuaEntityDiedEventFilterAttributesGhostName),
    GhostType(LuaEntityDiedEventFilterAttributesGhostType),
    Name(LuaEntityDiedEventFilterAttributesName),
    Type(LuaEntityDiedEventFilterAttributesType),
}

pub struct LuaEntityDiedEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<LuaEntityDiedEventFilterAttributes>,
}
pub struct LuaEntityMarkedForDeconstructionEventFilterAttributesGhostName {
    pub name: String,
}

pub struct LuaEntityMarkedForDeconstructionEventFilterAttributesGhostType {
    pub typ: String,
}

pub struct LuaEntityMarkedForDeconstructionEventFilterAttributesName {
    pub name: String,
}

pub struct LuaEntityMarkedForDeconstructionEventFilterAttributesType {
    pub typ: String,
}

pub enum LuaEntityMarkedForDeconstructionEventFilterAttributes {
    GhostName(LuaEntityMarkedForDeconstructionEventFilterAttributesGhostName),
    GhostType(LuaEntityMarkedForDeconstructionEventFilterAttributesGhostType),
    Name(LuaEntityMarkedForDeconstructionEventFilterAttributesName),
    Type(LuaEntityMarkedForDeconstructionEventFilterAttributesType),
}

pub struct LuaEntityMarkedForDeconstructionEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<LuaEntityMarkedForDeconstructionEventFilterAttributes>,
}
pub struct LuaEntityMarkedForUpgradeEventFilterAttributesGhostName {
    pub name: String,
}

pub struct LuaEntityMarkedForUpgradeEventFilterAttributesGhostType {
    pub typ: String,
}

pub struct LuaEntityMarkedForUpgradeEventFilterAttributesName {
    pub name: String,
}

pub struct LuaEntityMarkedForUpgradeEventFilterAttributesType {
    pub typ: String,
}

pub enum LuaEntityMarkedForUpgradeEventFilterAttributes {
    GhostName(LuaEntityMarkedForUpgradeEventFilterAttributesGhostName),
    GhostType(LuaEntityMarkedForUpgradeEventFilterAttributesGhostType),
    Name(LuaEntityMarkedForUpgradeEventFilterAttributesName),
    Type(LuaEntityMarkedForUpgradeEventFilterAttributesType),
}

pub struct LuaEntityMarkedForUpgradeEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<LuaEntityMarkedForUpgradeEventFilterAttributes>,
}
pub struct LuaPlayerBuiltEntityEventFilterAttributesForce {
    pub force: String,
}

pub struct LuaPlayerBuiltEntityEventFilterAttributesGhostName {
    pub name: String,
}

pub struct LuaPlayerBuiltEntityEventFilterAttributesGhostType {
    pub typ: String,
}

pub struct LuaPlayerBuiltEntityEventFilterAttributesName {
    pub name: String,
}

pub struct LuaPlayerBuiltEntityEventFilterAttributesType {
    pub typ: String,
}

pub enum LuaPlayerBuiltEntityEventFilterAttributes {
    Force(LuaPlayerBuiltEntityEventFilterAttributesForce),
    GhostName(LuaPlayerBuiltEntityEventFilterAttributesGhostName),
    GhostType(LuaPlayerBuiltEntityEventFilterAttributesGhostType),
    Name(LuaPlayerBuiltEntityEventFilterAttributesName),
    Type(LuaPlayerBuiltEntityEventFilterAttributesType),
}

pub struct LuaPlayerBuiltEntityEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<LuaPlayerBuiltEntityEventFilterAttributes>,
}
pub struct LuaPlayerMinedEntityEventFilterAttributesGhostName {
    pub name: String,
}

pub struct LuaPlayerMinedEntityEventFilterAttributesGhostType {
    pub typ: String,
}

pub struct LuaPlayerMinedEntityEventFilterAttributesName {
    pub name: String,
}

pub struct LuaPlayerMinedEntityEventFilterAttributesType {
    pub typ: String,
}

pub enum LuaPlayerMinedEntityEventFilterAttributes {
    GhostName(LuaPlayerMinedEntityEventFilterAttributesGhostName),
    GhostType(LuaPlayerMinedEntityEventFilterAttributesGhostType),
    Name(LuaPlayerMinedEntityEventFilterAttributesName),
    Type(LuaPlayerMinedEntityEventFilterAttributesType),
}

pub struct LuaPlayerMinedEntityEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<LuaPlayerMinedEntityEventFilterAttributes>,
}
pub struct LuaPlayerRepairedEntityEventFilterAttributesGhostName {
    pub name: String,
}

pub struct LuaPlayerRepairedEntityEventFilterAttributesGhostType {
    pub typ: String,
}

pub struct LuaPlayerRepairedEntityEventFilterAttributesName {
    pub name: String,
}

pub struct LuaPlayerRepairedEntityEventFilterAttributesType {
    pub typ: String,
}

pub enum LuaPlayerRepairedEntityEventFilterAttributes {
    GhostName(LuaPlayerRepairedEntityEventFilterAttributesGhostName),
    GhostType(LuaPlayerRepairedEntityEventFilterAttributesGhostType),
    Name(LuaPlayerRepairedEntityEventFilterAttributesName),
    Type(LuaPlayerRepairedEntityEventFilterAttributesType),
}

pub struct LuaPlayerRepairedEntityEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<LuaPlayerRepairedEntityEventFilterAttributes>,
}
pub struct LuaPostEntityDiedEventFilterAttributesType {
    pub typ: String,
}

pub enum LuaPostEntityDiedEventFilterAttributes {
    Type(LuaPostEntityDiedEventFilterAttributesType),
}

pub struct LuaPostEntityDiedEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<LuaPostEntityDiedEventFilterAttributes>,
}
pub struct LuaPreGhostDeconstructedEventFilterAttributesGhostName {
    pub name: String,
}

pub struct LuaPreGhostDeconstructedEventFilterAttributesGhostType {
    pub typ: String,
}

pub struct LuaPreGhostDeconstructedEventFilterAttributesName {
    pub name: String,
}

pub struct LuaPreGhostDeconstructedEventFilterAttributesType {
    pub typ: String,
}

pub enum LuaPreGhostDeconstructedEventFilterAttributes {
    GhostName(LuaPreGhostDeconstructedEventFilterAttributesGhostName),
    GhostType(LuaPreGhostDeconstructedEventFilterAttributesGhostType),
    Name(LuaPreGhostDeconstructedEventFilterAttributesName),
    Type(LuaPreGhostDeconstructedEventFilterAttributesType),
}

pub struct LuaPreGhostDeconstructedEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<LuaPreGhostDeconstructedEventFilterAttributes>,
}
pub struct LuaPreGhostUpgradedEventFilterAttributesGhostName {
    pub name: String,
}

pub struct LuaPreGhostUpgradedEventFilterAttributesGhostType {
    pub typ: String,
}

pub struct LuaPreGhostUpgradedEventFilterAttributesName {
    pub name: String,
}

pub struct LuaPreGhostUpgradedEventFilterAttributesType {
    pub typ: String,
}

pub enum LuaPreGhostUpgradedEventFilterAttributes {
    GhostName(LuaPreGhostUpgradedEventFilterAttributesGhostName),
    GhostType(LuaPreGhostUpgradedEventFilterAttributesGhostType),
    Name(LuaPreGhostUpgradedEventFilterAttributesName),
    Type(LuaPreGhostUpgradedEventFilterAttributesType),
}

pub struct LuaPreGhostUpgradedEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<LuaPreGhostUpgradedEventFilterAttributes>,
}
pub struct LuaPrePlayerMinedEntityEventFilterAttributesGhostName {
    pub name: String,
}

pub struct LuaPrePlayerMinedEntityEventFilterAttributesGhostType {
    pub typ: String,
}

pub struct LuaPrePlayerMinedEntityEventFilterAttributesName {
    pub name: String,
}

pub struct LuaPrePlayerMinedEntityEventFilterAttributesType {
    pub typ: String,
}

pub enum LuaPrePlayerMinedEntityEventFilterAttributes {
    GhostName(LuaPrePlayerMinedEntityEventFilterAttributesGhostName),
    GhostType(LuaPrePlayerMinedEntityEventFilterAttributesGhostType),
    Name(LuaPrePlayerMinedEntityEventFilterAttributesName),
    Type(LuaPrePlayerMinedEntityEventFilterAttributesType),
}

pub struct LuaPrePlayerMinedEntityEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<LuaPrePlayerMinedEntityEventFilterAttributes>,
}
pub struct LuaPreRobotMinedEntityEventFilterAttributesGhostName {
    pub name: String,
}

pub struct LuaPreRobotMinedEntityEventFilterAttributesGhostType {
    pub typ: String,
}

pub struct LuaPreRobotMinedEntityEventFilterAttributesName {
    pub name: String,
}

pub struct LuaPreRobotMinedEntityEventFilterAttributesType {
    pub typ: String,
}

pub enum LuaPreRobotMinedEntityEventFilterAttributes {
    GhostName(LuaPreRobotMinedEntityEventFilterAttributesGhostName),
    GhostType(LuaPreRobotMinedEntityEventFilterAttributesGhostType),
    Name(LuaPreRobotMinedEntityEventFilterAttributesName),
    Type(LuaPreRobotMinedEntityEventFilterAttributesType),
}

pub struct LuaPreRobotMinedEntityEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<LuaPreRobotMinedEntityEventFilterAttributes>,
}
pub struct LuaRobotBuiltEntityEventFilterAttributesForce {
    pub force: String,
}

pub struct LuaRobotBuiltEntityEventFilterAttributesGhostName {
    pub name: String,
}

pub struct LuaRobotBuiltEntityEventFilterAttributesGhostType {
    pub typ: String,
}

pub struct LuaRobotBuiltEntityEventFilterAttributesName {
    pub name: String,
}

pub struct LuaRobotBuiltEntityEventFilterAttributesType {
    pub typ: String,
}

pub enum LuaRobotBuiltEntityEventFilterAttributes {
    Force(LuaRobotBuiltEntityEventFilterAttributesForce),
    GhostName(LuaRobotBuiltEntityEventFilterAttributesGhostName),
    GhostType(LuaRobotBuiltEntityEventFilterAttributesGhostType),
    Name(LuaRobotBuiltEntityEventFilterAttributesName),
    Type(LuaRobotBuiltEntityEventFilterAttributesType),
}

pub struct LuaRobotBuiltEntityEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<LuaRobotBuiltEntityEventFilterAttributes>,
}
pub struct LuaRobotMinedEntityEventFilterAttributesGhostName {
    pub name: String,
}

pub struct LuaRobotMinedEntityEventFilterAttributesGhostType {
    pub typ: String,
}

pub struct LuaRobotMinedEntityEventFilterAttributesName {
    pub name: String,
}

pub struct LuaRobotMinedEntityEventFilterAttributesType {
    pub typ: String,
}

pub enum LuaRobotMinedEntityEventFilterAttributes {
    GhostName(LuaRobotMinedEntityEventFilterAttributesGhostName),
    GhostType(LuaRobotMinedEntityEventFilterAttributesGhostType),
    Name(LuaRobotMinedEntityEventFilterAttributesName),
    Type(LuaRobotMinedEntityEventFilterAttributesType),
}

pub struct LuaRobotMinedEntityEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<LuaRobotMinedEntityEventFilterAttributes>,
}
pub struct LuaScriptRaisedBuiltEventFilterAttributesGhostName {
    pub name: String,
}

pub struct LuaScriptRaisedBuiltEventFilterAttributesGhostType {
    pub typ: String,
}

pub struct LuaScriptRaisedBuiltEventFilterAttributesName {
    pub name: String,
}

pub struct LuaScriptRaisedBuiltEventFilterAttributesType {
    pub typ: String,
}

pub enum LuaScriptRaisedBuiltEventFilterAttributes {
    GhostName(LuaScriptRaisedBuiltEventFilterAttributesGhostName),
    GhostType(LuaScriptRaisedBuiltEventFilterAttributesGhostType),
    Name(LuaScriptRaisedBuiltEventFilterAttributesName),
    Type(LuaScriptRaisedBuiltEventFilterAttributesType),
}

pub struct LuaScriptRaisedBuiltEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<LuaScriptRaisedBuiltEventFilterAttributes>,
}
pub struct LuaScriptRaisedDestroyEventFilterAttributesGhostName {
    pub name: String,
}

pub struct LuaScriptRaisedDestroyEventFilterAttributesGhostType {
    pub typ: String,
}

pub struct LuaScriptRaisedDestroyEventFilterAttributesName {
    pub name: String,
}

pub struct LuaScriptRaisedDestroyEventFilterAttributesType {
    pub typ: String,
}

pub enum LuaScriptRaisedDestroyEventFilterAttributes {
    GhostName(LuaScriptRaisedDestroyEventFilterAttributesGhostName),
    GhostType(LuaScriptRaisedDestroyEventFilterAttributesGhostType),
    Name(LuaScriptRaisedDestroyEventFilterAttributesName),
    Type(LuaScriptRaisedDestroyEventFilterAttributesType),
}

pub struct LuaScriptRaisedDestroyEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<LuaScriptRaisedDestroyEventFilterAttributes>,
}
pub struct LuaScriptRaisedReviveEventFilterAttributesGhostName {
    pub name: String,
}

pub struct LuaScriptRaisedReviveEventFilterAttributesGhostType {
    pub typ: String,
}

pub struct LuaScriptRaisedReviveEventFilterAttributesName {
    pub name: String,
}

pub struct LuaScriptRaisedReviveEventFilterAttributesType {
    pub typ: String,
}

pub enum LuaScriptRaisedReviveEventFilterAttributes {
    GhostName(LuaScriptRaisedReviveEventFilterAttributesGhostName),
    GhostType(LuaScriptRaisedReviveEventFilterAttributesGhostType),
    Name(LuaScriptRaisedReviveEventFilterAttributesName),
    Type(LuaScriptRaisedReviveEventFilterAttributesType),
}

pub struct LuaScriptRaisedReviveEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<LuaScriptRaisedReviveEventFilterAttributes>,
}
pub struct LuaScriptRaisedTeleportedEventFilterAttributesGhostName {
    pub name: String,
}

pub struct LuaScriptRaisedTeleportedEventFilterAttributesGhostType {
    pub typ: String,
}

pub struct LuaScriptRaisedTeleportedEventFilterAttributesName {
    pub name: String,
}

pub struct LuaScriptRaisedTeleportedEventFilterAttributesType {
    pub typ: String,
}

pub enum LuaScriptRaisedTeleportedEventFilterAttributes {
    GhostName(LuaScriptRaisedTeleportedEventFilterAttributesGhostName),
    GhostType(LuaScriptRaisedTeleportedEventFilterAttributesGhostType),
    Name(LuaScriptRaisedTeleportedEventFilterAttributesName),
    Type(LuaScriptRaisedTeleportedEventFilterAttributesType),
}

pub struct LuaScriptRaisedTeleportedEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<LuaScriptRaisedTeleportedEventFilterAttributes>,
}
pub struct LuaSectorScannedEventFilterAttributesGhostName {
    pub name: String,
}

pub struct LuaSectorScannedEventFilterAttributesGhostType {
    pub typ: String,
}

pub struct LuaSectorScannedEventFilterAttributesName {
    pub name: String,
}

pub struct LuaSectorScannedEventFilterAttributesType {
    pub typ: String,
}

pub enum LuaSectorScannedEventFilterAttributes {
    GhostName(LuaSectorScannedEventFilterAttributesGhostName),
    GhostType(LuaSectorScannedEventFilterAttributesGhostType),
    Name(LuaSectorScannedEventFilterAttributesName),
    Type(LuaSectorScannedEventFilterAttributesType),
}

pub struct LuaSectorScannedEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<LuaSectorScannedEventFilterAttributes>,
}
pub struct LuaUpgradeCancelledEventFilterAttributesGhostName {
    pub name: String,
}

pub struct LuaUpgradeCancelledEventFilterAttributesGhostType {
    pub typ: String,
}

pub struct LuaUpgradeCancelledEventFilterAttributesName {
    pub name: String,
}

pub struct LuaUpgradeCancelledEventFilterAttributesType {
    pub typ: String,
}

pub enum LuaUpgradeCancelledEventFilterAttributes {
    GhostName(LuaUpgradeCancelledEventFilterAttributesGhostName),
    GhostType(LuaUpgradeCancelledEventFilterAttributesGhostType),
    Name(LuaUpgradeCancelledEventFilterAttributesName),
    Type(LuaUpgradeCancelledEventFilterAttributesType),
}

pub struct LuaUpgradeCancelledEventFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<LuaUpgradeCancelledEventFilterAttributes>,
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
pub struct ModSetting {
    pub value: ModSettingValueUnion,
}
pub struct ModSettingPrototypeFilterAttributesMod {
    pub mod_name: String,
}

pub struct ModSettingPrototypeFilterAttributesSettingType {
    pub typ: String,
}

pub struct ModSettingPrototypeFilterAttributesType {
    pub typ: ModSettingPrototypeFilterAttributesTypeUnion,
}

pub enum ModSettingPrototypeFilterAttributes {
    Mod(ModSettingPrototypeFilterAttributesMod),
    SettingType(ModSettingPrototypeFilterAttributesSettingType),
    Type(ModSettingPrototypeFilterAttributesType),
}

pub struct ModSettingPrototypeFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<ModSettingPrototypeFilterAttributes>,
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
pub struct ProductAttributesFluid {
    pub temperature: Option<f64>,
}

pub enum ProductAttributes {
    Fluid(ProductAttributesFluid),
}

pub struct Product {
    pub amount: Option<f64>,
    pub amount_max: Option<ProductAmountMaxUnion>,
    pub amount_min: Option<ProductAmountMinUnion>,
    pub catalyst_amount: Option<ProductCatalystAmountUnion>,
    pub name: String,
    pub probability: Option<f64>,
    pub typ: String,
    pub attributes: Option<ProductAttributes>,
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
type PrototypeFilter = Vec<PrototypeFilterUnion>;
pub struct PrototypeHistory {
    pub changed: Vec<String>,
    pub created: String,
}
type RealOrientation = f32;
pub struct RecipePrototypeFilterAttributesCategory {
    pub category: String,
}

pub struct RecipePrototypeFilterAttributesEmissionsMultiplier {
    pub comparison: ComparatorString,
    pub value: f64,
}

pub struct RecipePrototypeFilterAttributesEnergy {
    pub comparison: ComparatorString,
    pub value: f64,
}

pub struct RecipePrototypeFilterAttributesHasIngredientFluid {
    pub elem_filters: Option<Vec<FluidPrototypeFilter>>,
}

pub struct RecipePrototypeFilterAttributesHasIngredientItem {
    pub elem_filters: Option<Vec<ItemPrototypeFilter>>,
}

pub struct RecipePrototypeFilterAttributesHasProductFluid {
    pub elem_filters: Option<Vec<FluidPrototypeFilter>>,
}

pub struct RecipePrototypeFilterAttributesHasProductItem {
    pub elem_filters: Option<Vec<ItemPrototypeFilter>>,
}

pub struct RecipePrototypeFilterAttributesOverloadMultiplier {
    pub comparison: ComparatorString,
    pub value: u32,
}

pub struct RecipePrototypeFilterAttributesRequestPasteMultiplier {
    pub comparison: ComparatorString,
    pub value: u32,
}

pub struct RecipePrototypeFilterAttributesSubgroup {
    pub subgroup: String,
}

pub enum RecipePrototypeFilterAttributes {
    Category(RecipePrototypeFilterAttributesCategory),
    EmissionsMultiplier(RecipePrototypeFilterAttributesEmissionsMultiplier),
    Energy(RecipePrototypeFilterAttributesEnergy),
    HasIngredientFluid(RecipePrototypeFilterAttributesHasIngredientFluid),
    HasIngredientItem(RecipePrototypeFilterAttributesHasIngredientItem),
    HasProductFluid(RecipePrototypeFilterAttributesHasProductFluid),
    HasProductItem(RecipePrototypeFilterAttributesHasProductItem),
    OverloadMultiplier(RecipePrototypeFilterAttributesOverloadMultiplier),
    RequestPasteMultiplier(RecipePrototypeFilterAttributesRequestPasteMultiplier),
    Subgroup(RecipePrototypeFilterAttributesSubgroup),
}

pub struct RecipePrototypeFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<RecipePrototypeFilterAttributes>,
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
pub struct ScriptRenderVertexTarget {
    pub target: ScriptRenderVertexTargetTargetUnion,
    pub target_offset: Option<Vector>,
}
pub struct SelectedPrototypeData {
    pub base_type: String,
    pub derived_type: String,
    pub name: String,
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
pub struct TechnologyModifierAttributesOtherTypes {
    pub modifier: f64,
}

pub struct TechnologyModifierAttributesAmmoDamage {
    pub ammo_category: String,
    pub modifier: f64,
}

pub struct TechnologyModifierAttributesGiveItem {
    pub count: Option<u32>,
    pub item: String,
}

pub struct TechnologyModifierAttributesGunSpeed {
    pub ammo_category: String,
    pub modifier: f64,
}

pub struct TechnologyModifierAttributesNothing {
    pub effect_description: LocalisedString,
}

pub struct TechnologyModifierAttributesTurretAttack {
    pub modifier: f64,
    pub turret_id: String,
}

pub struct TechnologyModifierAttributesUnlockRecipe {
    pub recipe: String,
}

pub enum TechnologyModifierAttributes {
    OtherTypes(TechnologyModifierAttributesOtherTypes),
    AmmoDamage(TechnologyModifierAttributesAmmoDamage),
    GiveItem(TechnologyModifierAttributesGiveItem),
    GunSpeed(TechnologyModifierAttributesGunSpeed),
    Nothing(TechnologyModifierAttributesNothing),
    TurretAttack(TechnologyModifierAttributesTurretAttack),
    UnlockRecipe(TechnologyModifierAttributesUnlockRecipe),
}

pub struct TechnologyModifier {
    pub typ: String,
    pub attributes: Option<TechnologyModifierAttributes>,
}
pub struct TechnologyPrototypeFilterAttributesLevel {
    pub comparison: ComparatorString,
    pub value: u32,
}

pub struct TechnologyPrototypeFilterAttributesMaxLevel {
    pub comparison: ComparatorString,
    pub value: u32,
}

pub struct TechnologyPrototypeFilterAttributesResearchUnitIngredient {
    pub ingredient: String,
}

pub struct TechnologyPrototypeFilterAttributesTime {
    pub comparison: ComparatorString,
    pub value: u32,
}

pub struct TechnologyPrototypeFilterAttributesUnlocksRecipe {
    pub recipe: String,
}

pub enum TechnologyPrototypeFilterAttributes {
    Level(TechnologyPrototypeFilterAttributesLevel),
    MaxLevel(TechnologyPrototypeFilterAttributesMaxLevel),
    ResearchUnitIngredient(TechnologyPrototypeFilterAttributesResearchUnitIngredient),
    Time(TechnologyPrototypeFilterAttributesTime),
    UnlocksRecipe(TechnologyPrototypeFilterAttributesUnlocksRecipe),
}

pub struct TechnologyPrototypeFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<TechnologyPrototypeFilterAttributes>,
}
pub struct Tile {
    pub name: String,
    pub position: TilePosition,
}
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
}
pub struct TilePrototypeFilterAttributesCollisionMask {
    pub mask: TilePrototypeFilterAttributesMaskUnion,
    pub mask_mode: String,
}

pub struct TilePrototypeFilterAttributesDecorativeRemovalProbability {
    pub comparison: ComparatorString,
    pub value: f32,
}

pub struct TilePrototypeFilterAttributesEmissions {
    pub comparison: ComparatorString,
    pub value: f64,
}

pub struct TilePrototypeFilterAttributesVehicleFrictionModifier {
    pub comparison: ComparatorString,
    pub value: f64,
}

pub struct TilePrototypeFilterAttributesWalkingSpeedModifier {
    pub comparison: ComparatorString,
    pub value: f64,
}

pub enum TilePrototypeFilterAttributes {
    CollisionMask(TilePrototypeFilterAttributesCollisionMask),
    DecorativeRemovalProbability(TilePrototypeFilterAttributesDecorativeRemovalProbability),
    Emissions(TilePrototypeFilterAttributesEmissions),
    VehicleFrictionModifier(TilePrototypeFilterAttributesVehicleFrictionModifier),
    WalkingSpeedModifier(TilePrototypeFilterAttributesWalkingSpeedModifier),
}

pub struct TilePrototypeFilter {
    pub filter: String,
    pub invert: Option<bool>,
    pub mode: Option<String>,
    pub attributes: Option<TilePrototypeFilterAttributes>,
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
