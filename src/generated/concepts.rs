use std::collections::HashMap;
use std::collections::HashSet;

use serde::Deserialize;

use super::classes::*;
use super::defines::*;
use super::*;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AchievementPrototypeFilterAttributesTypeUnion {
    String(String),
    Array(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct AchievementPrototypeFilterAttributesType {
    /// The prototype type, or a list of acceptable types.
    #[serde(rename = "type")]
    pub typ: AchievementPrototypeFilterAttributesTypeUnion,
}

#[derive(Debug, Deserialize)]
pub enum AchievementPrototypeFilterAttributes {
    Type(AchievementPrototypeFilterAttributesType),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct AchievementPrototypeFilter {
    /// The condition to filter on. One of `"allowed-without-fight"`, `"type"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<AchievementPrototypeFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct AdvancedMapGenSettings {
    pub difficulty_settings: DifficultySettings,
    pub enemy_evolution: EnemyEvolutionMapSettings,
    pub enemy_expansion: EnemyExpansionMapSettings,
    pub pollution: PollutionMapSettings,
}

#[derive(Debug, Deserialize)]
pub struct Alert {
    /// The SignalID used for a custom alert. Only present for custom alerts.
    pub icon: Option<SignalID>,
    /// The message for a custom alert. Only present for custom alerts.
    pub message: Option<LocalisedString>,
    pub position: Option<MapPosition>,
    pub prototype: Option<MaybeCycle<LuaEntityPrototype>>,
    pub target: Option<MaybeCycle<LuaEntity>>,
    /// The tick this alert was created.
    pub tick: u32,
}

/// A [string](string) that specifies where a GUI element should be.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Alignment {
    TopLeft,
    MiddleLeft,
    /// The same as `"middle-left"`
    Left,
    BottomLeft,
    TopCenter,
    MiddleCenter,
    /// The same as `"middle-center"`
    Center,
    BottomCenter,
    TopRight,
    /// The same as `"middle-right"`
    Right,
    BottomRight,
}

#[derive(Debug, Deserialize)]
pub struct AmmoType {
    pub action: Option<Vec<TriggerItem>>,
    /// Ammo category of this ammo.
    pub category: String,
    /// When `true`, the gun will be able to shoot even when the target is out of range. Only applies when `target_type` is `position`. The gun will fire at the maximum range in the direction of the target position. Defaults to `false`.
    pub clamp_position: Option<bool>,
    pub consumption_modifier: Option<Double>,
    pub cooldown_modifier: Option<Double>,
    /// Energy consumption of a single shot, if applicable. Defaults to `0`.
    pub energy_consumption: Option<Double>,
    pub range_modifier: Option<Double>,
    /// One of `"entity"` (fires at an entity), `"position"` (fires directly at a position), or `"direction"` (fires in a direction).
    pub target_type: String,
}

/// Any basic type (string, number, boolean), table, or LuaObject.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Any {
    String(String),
    Boolean(bool),
    Number(f64),
    Table,
    LuaObject,
}

/// Any basic type (string, number, boolean) or table.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AnyBasic {
    String(String),
    Boolean(bool),
    Number(f64),
    Table,
}

#[derive(Debug, Deserialize)]
pub struct ArithmeticCombinatorParameters {
    /// Constant to use as the first argument of the operation. Has no effect when `first_signal` is set. Defaults to `0`.
    pub first_constant: Option<i32>,
    /// First signal to use in an operation. If not specified, the second argument will be the value of `first_constant`.
    pub first_signal: Option<SignalID>,
    /// Must be one of `"*"`, `"/"`, `"+"`, `"-"`, `"%"`, `"^"`, `"<<"`, `">>"`, `"AND"`, `"OR"`, `"XOR"`. When not specified, defaults to `"*"`.
    pub operation: Option<String>,
    /// Specifies the signal to output.
    pub output_signal: Option<SignalID>,
    /// Constant to use as the second argument of the operation. Has no effect when `second_signal` is set. Defaults to `0`.
    pub second_constant: Option<i32>,
    /// Second signal to use in an operation. If not specified, the second argument will be the value of `second_constant`.
    pub second_signal: Option<SignalID>,
}

#[derive(Debug, Deserialize)]
pub struct AttackParameterFluid {
    /// Multiplier applied to the damage of an attack.
    pub damage_modifier: Double,
    /// Name of the [LuaFluidPrototype](LuaFluidPrototype).
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct AttackParametersAttributesProjectile {
    pub projectile_center: Vector,
    pub projectile_creation_distance: Float,
    pub projectile_creation_parameters: Option<Vec<CircularProjectileCreationSpecification>>,
    pub projectile_orientation_offset: Float,
    pub shell_particle: Option<CircularParticleCreationSpecification>,
}

#[derive(Debug, Deserialize)]
pub struct AttackParametersAttributesStream {
    pub fluid_consumption: Float,
    pub fluids: Option<Vec<AttackParameterFluid>>,
    pub gun_barrel_length: Float,
    pub gun_center_shift: HashMap<String, Vector>,
    pub projectile_creation_parameters: Option<Vec<CircularProjectileCreationSpecification>>,
}

#[derive(Debug, Deserialize)]
pub enum AttackParametersAttributes {
    Projectile(AttackParametersAttributesProjectile),
    Stream(AttackParametersAttributesStream),
}

#[derive(Debug, Deserialize)]
pub struct AttackParameters {
    /// List of the names of compatible [LuaAmmoCategoryPrototypes](LuaAmmoCategoryPrototype).
    pub ammo_categories: Option<Vec<String>>,
    /// Multiplier applied to the ammo consumption of an attack.
    pub ammo_consumption_modifier: Float,
    pub ammo_type: Option<AmmoType>,
    /// Minimum amount of ticks between shots. If this is less than `1`, multiple shots can be performed per tick.
    pub cooldown: Float,
    /// Multiplier applied to the damage of an attack.
    pub damage_modifier: Float,
    /// When searching for the nearest enemy to attack, `fire_penalty` is added to the enemy's distance if they are on fire.
    pub fire_penalty: Float,
    /// When searching for an enemy to attack, a higher `health_penalty` will discourage targeting enemies with high health. A negative penalty will do the opposite.
    pub health_penalty: Float,
    /// If less than `range`, the entity will choose a random distance between `range` and `min_attack_distance` and attack from that distance. Used for spitters.
    pub min_attack_distance: Float,
    /// Minimum range of attack. Used with flamethrower turrets to prevent self-immolation.
    pub min_range: Float,
    pub movement_slow_down_cooldown: Float,
    pub movement_slow_down_factor: Double,
    /// Maximum range of attack.
    pub range: Float,
    /// Defines how the range is determined. Either `'center-to-center'` or `'bounding-box-to-bounding-box'`.
    pub range_mode: String,
    /// When searching for an enemy to attack, a higher `rotate_penalty` will discourage targeting enemies that would take longer to turn to face.
    pub rotate_penalty: Float,
    /// The arc that the entity can attack in as a fraction of a circle. A value of `1` means the full 360 degrees.
    pub turn_range: Float,
    /// The type of AttackParameter. One of `'projectile'`, `'stream'` or `'beam'`.
    #[serde(rename = "type")]
    pub typ: String,
    /// Number of ticks it takes for the weapon to actually shoot after it has been ordered to do so.
    pub warmup: u32,
    /// Other attributes may be specified depending on `type`:
    pub attributes: Option<AttackParametersAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct AutoplaceControl {
    /// For things that are placed as spots such as ores and enemy bases, frequency is generally proportional to number of spots placed per unit area. For continuous features such as forests, frequency is how compressed the probability function is over distance, i.e. the inverse of 'scale' (similar to terrain_segmentation). When the [LuaAutoplaceControlPrototype](LuaAutoplaceControlPrototype) is of the category `"terrain"`, then scale is shown in the map generator GUI instead of frequency.
    pub frequency: MapGenSize,
    /// Has different effects for different things, but generally affects the 'health' or density of a thing that is placed without affecting where it is placed. For trees, richness affects tree health. For ores, richness multiplies the amount of ore at any given tile in a patch. Metadata about autoplace controls (such as whether or not 'richness' does anything for them) can be found in the [LuaAutoplaceControlPrototype](LuaAutoplaceControlPrototype) by looking up `game.autoplace_control_prototypes[(control prototype name)]`, e.g. `game.autoplace_control_prototypes["enemy-base"].richness` is false, because enemy base autoplacement doesn't use richness.
    pub richness: MapGenSize,
    /// For things that are placed as spots, size is proportional to the area of each spot. For continuous features, size affects how much of the map is covered with the thing, and is called 'coverage' in the GUI.
    pub size: MapGenSize,
}

#[derive(Debug, Deserialize)]
pub struct AutoplaceSettings {
    pub settings: HashMap<String, AutoplaceControl>,
    /// Whether missing autoplace names for this type should be default enabled.
    pub treat_missing_as_default: bool,
}

#[derive(Debug, Deserialize)]
/// Specifies how probability and richness are calculated when placing something on the map. Can be specified either using `probability_expression` and `richness_expression` or by using all the other fields.
pub struct AutoplaceSpecification {
    /// Control prototype name.
    pub control: Option<String>,
    pub coverage: Option<Double>,
    pub default_enabled: Option<bool>,
    pub force: Option<String>,
    pub max_probability: Option<Double>,
    pub order: Option<String>,
    pub peaks: Option<Vec<AutoplaceSpecificationPeak>>,
    pub placement_density: Option<u32>,
    pub probability_expression: Option<NoiseExpression>,
    pub random_probability_penalty: Option<Double>,
    pub richness_base: Option<Double>,
    pub richness_expression: Option<NoiseExpression>,
    pub richness_multiplier: Option<Double>,
    pub richness_multiplier_distance_bonus: Option<Double>,
    pub sharpness: Option<Double>,
    pub starting_area_size: Option<u32>,
    pub tile_restriction: Option<Vec<AutoplaceSpecificationRestriction>>,
}

#[derive(Debug, Deserialize)]
pub struct AutoplaceSpecificationPeak {
    pub aux_max_range: Option<Double>,
    pub aux_optimal: Option<Double>,
    pub aux_range: Option<Double>,
    pub aux_top_property_limit: Option<Double>,
    pub distance_max_range: Option<Double>,
    pub distance_optimal: Option<Double>,
    pub distance_range: Option<Double>,
    pub distance_top_property_limit: Option<Double>,
    pub elevation_max_range: Option<Double>,
    pub elevation_optimal: Option<Double>,
    pub elevation_range: Option<Double>,
    pub elevation_top_property_limit: Option<Double>,
    pub influence: Option<Double>,
    pub max_influence: Option<Double>,
    pub min_influence: Option<Double>,
    #[serde(rename = "noisePersistence")]
    pub noise_persistence: Option<Double>,
    /// Prototype name of the noise layer.
    pub noise_layer: Option<String>,
    pub noise_octaves_difference: Option<Double>,
    pub richness_influence: Option<Double>,
    pub starting_area_weight_max_range: Option<Double>,
    pub starting_area_weight_optimal: Option<Double>,
    pub starting_area_weight_range: Option<Double>,
    pub starting_area_weight_top_property_limit: Option<Double>,
    pub temperature_max_range: Option<Double>,
    pub temperature_optimal: Option<Double>,
    pub temperature_range: Option<Double>,
    pub temperature_top_property_limit: Option<Double>,
    pub tier_from_start_max_range: Option<Double>,
    pub tier_from_start_optimal: Option<Double>,
    pub tier_from_start_range: Option<Double>,
    pub tier_from_start_top_property_limit: Option<Double>,
    pub water_max_range: Option<Double>,
    pub water_optimal: Option<Double>,
    pub water_range: Option<Double>,
    pub water_top_property_limit: Option<Double>,
}

#[derive(Debug, Deserialize)]
pub struct AutoplaceSpecificationRestriction {
    /// Tile prototype name
    pub first: Option<String>,
    /// Second prototype name
    pub second: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BeamTarget {
    /// The target entity.
    pub entity: Option<MaybeCycle<LuaEntity>>,
    /// The target position.
    pub position: Option<MapPosition>,
}

#[derive(Debug, Deserialize)]
pub struct BlueprintCircuitConnection;

#[derive(Debug, Deserialize)]
pub struct BlueprintControlBehavior;

#[derive(Debug, Deserialize)]
/// The representation of an entity inside of a blueprint. It has at least these fields, but can contain additional ones depending on the kind of entity.
pub struct BlueprintEntity {
    /// The circuit network connections of the entity, if there are any. Only relevant for entities that support circuit connections.
    pub connections: Option<BlueprintCircuitConnection>,
    /// The control behavior of the entity, if it has one. The format of the control behavior depends on the entity's type. Only relevant for entities that support control behaviors.
    pub control_behavior: Option<BlueprintControlBehavior>,
    /// The direction the entity is facing. Only present for entities that can face in different directions and when the entity is not facing north.
    pub direction: Option<Direction>,
    /// The entity's unique identifier in the blueprint.
    pub entity_number: u32,
    /// The items that the entity will request when revived, if there are any. It's a mapping of prototype names to amounts. Only relevant for entity ghosts.
    pub items: Option<HashMap<String, u32>>,
    /// The prototype name of the entity.
    pub name: String,
    /// The position of the entity.
    pub position: MapPosition,
    /// The schedule of the entity, if it has one. Only relevant for locomotives.
    pub schedule: Option<Vec<TrainScheduleRecord>>,
    /// The entity tags of the entity, if there are any. Only relevant for entity ghosts.
    pub tags: Option<Tags>,
}

#[derive(Debug, Deserialize)]
pub struct BlueprintSignalIcon {
    /// Index of the icon in the blueprint icons slots. Has to be an integer in the range [1, 4].
    pub index: u32,
    /// The icon to use. It can be any item icon as well as any virtual signal icon.
    pub signal: SignalID,
}

#[derive(Debug, Deserialize)]
/// Two positions, specifying the top-left and bottom-right corner of the box respectively. Like with [MapPosition](MapPosition), the names of the members may be omitted. When read from the game, the third member `orientation` is present if it is non-zero, however it is ignored when provided to the game.
///
/// # Examples
///
/// * Explicit definition:
/// ```text
/// {left_top = {x = -2, y = -3}, right_bottom = {x = 5, y = 8}}
/// ```
/// * Shorthand:
/// ```text
/// {{-2, -3}, {5, 8}}
/// ```
pub struct BoundingBox {
    pub left_top: MapPosition,
    pub orientation: Option<RealOrientation>,
    pub right_bottom: MapPosition,
}

#[derive(Debug, Deserialize)]
pub struct CapsuleActionAttributesArtilleryRemote {
    /// Name of the [flare prototype](LuaEntityPrototype).
    pub flare: String,
}

#[derive(Debug, Deserialize)]
pub struct CapsuleActionAttributesDestroyCliffs {
    pub attack_parameters: AttackParameters,
    pub radius: Float,
    pub timeout: u32,
}

#[derive(Debug, Deserialize)]
pub struct CapsuleActionAttributesEquipmentRemote {
    /// Name of the [LuaEquipmentPrototype](LuaEquipmentPrototype).
    pub equipment: String,
}

#[derive(Debug, Deserialize)]
pub struct CapsuleActionAttributesThrow {
    pub attack_parameters: AttackParameters,
    /// Whether using the capsule consumes an item from the stack.
    pub uses_stack: bool,
}

#[derive(Debug, Deserialize)]
pub struct CapsuleActionAttributesUseOnSelf {
    pub attack_parameters: AttackParameters,
}

#[derive(Debug, Deserialize)]
pub enum CapsuleActionAttributes {
    ArtilleryRemote(CapsuleActionAttributesArtilleryRemote),
    DestroyCliffs(CapsuleActionAttributesDestroyCliffs),
    EquipmentRemote(CapsuleActionAttributesEquipmentRemote),
    Throw(CapsuleActionAttributesThrow),
    UseOnSelf(CapsuleActionAttributesUseOnSelf),
}

#[derive(Debug, Deserialize)]
pub struct CapsuleAction {
    /// One of `"throw"`, `"equipment-remote"`, `"use-on-self"`, `"artillery-remote"`, `"destroy-cliffs"`.
    #[serde(rename = "type")]
    pub typ: String,
    /// Other attributes may be specified depending on `type`:
    pub attributes: Option<CapsuleActionAttributes>,
}

#[derive(Debug, Deserialize)]
/// # Notes
///
/// * Either `icon`, `text`, or both must be provided.
pub struct ChartTagSpec {
    pub icon: Option<SignalID>,
    pub last_user: Option<PlayerIdentification>,
    pub position: MapPosition,
    pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
/// Coordinates of a chunk in a [LuaSurface](LuaSurface) where each integer `x`/`y` represents a different chunk. This uses the same format as [MapPosition](MapPosition), meaning it can be specified either with or without explicit keys. A [MapPosition](MapPosition) can be translated to a ChunkPosition by dividing the `x`/`y` values by 32.
pub struct ChunkPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Deserialize)]
/// A [ChunkPosition](ChunkPosition) with an added bounding box for the area of the chunk.
pub struct ChunkPositionAndArea {
    pub area: BoundingBox,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Deserialize)]
pub struct CircuitCondition {
    /// Specifies how the inputs should be compared. If not specified, defaults to `"<"`.
    pub comparator: Option<ComparatorString>,
    /// Constant to compare `first_signal` to. Has no effect when `second_signal` is set. When neither `second_signal` nor `constant` are specified, the effect is as though `constant` were specified with the value `0`.
    pub constant: Option<i32>,
    /// Defaults to blank
    pub first_signal: Option<SignalID>,
    /// What to compare `first_signal` to. If not specified, `first_signal` will be compared to `constant`.
    pub second_signal: Option<SignalID>,
}

#[derive(Debug, Deserialize)]
pub struct CircuitConditionDefinition {
    pub condition: CircuitCondition,
    /// Whether the condition is currently fulfilled
    pub fulfilled: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CircuitConnectionDefinition {
    pub source_circuit_id: CircuitConnectorId,
    pub target_circuit_id: CircuitConnectorId,
    pub target_entity: MaybeCycle<LuaEntity>,
    /// Wire color, either [defines.wire_type.red](defines.wire_type.red) or [defines.wire_type.green](defines.wire_type.green).
    pub wire: WireType,
}

#[derive(Debug, Deserialize)]
pub struct CircularParticleCreationSpecification {
    /// This vector is a table with `x` and `y` keys instead of an array.
    pub center: Vector,
    pub creation_distance: Double,
    pub creation_distance_orientation: Double,
    pub direction: Float,
    pub direction_deviation: Float,
    pub height: Float,
    pub height_deviation: Float,
    /// Name of the [LuaEntityPrototype](LuaEntityPrototype)
    pub name: String,
    pub speed: Float,
    pub speed_deviation: Float,
    pub starting_frame_speed: Float,
    pub starting_frame_speed_deviation: Float,
    pub use_source_position: bool,
    pub vertical_speed: Float,
    pub vertical_speed_deviation: Float,
}

#[derive(Debug, Deserialize)]
pub struct CircularProjectileCreationSpecification {
    #[serde(rename = "_")]
    pub field_0: RealOrientation,
    #[serde(rename = "_")]
    pub field_1: Vector,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
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

#[derive(Debug, Deserialize)]
pub struct CliffPlacementSettings {
    /// Elevation at which the first row of cliffs is placed. The default is `10`, and this cannot be set from the map generation GUI.
    pub cliff_elevation_0: Float,
    /// Elevation difference between successive rows of cliffs. This is inversely proportional to 'frequency' in the map generation GUI. Specifically, when set from the GUI the value is `40 / frequency`.
    pub cliff_elevation_interval: Float,
    /// Name of the cliff prototype.
    pub name: String,
    /// Corresponds to 'continuity' in the GUI. This value is not used directly, but is used by the 'cliffiness' noise expression, which in combination with elevation and the two cliff elevation properties drives cliff placement (cliffs are placed when elevation crosses the elevation contours defined by `cliff_elevation_0` and `cliff_elevation_interval` when 'cliffiness' is greater than `0.5`). The default 'cliffiness' expression interprets this value such that larger values result in longer unbroken walls of cliffs, and smaller values (between `0` and `1`) result in larger gaps in cliff walls.
    pub richness: MapGenSize,
}

/// A set of flags. Active flags are in the dictionary as `true`, while inactive flags aren't present at all.
pub type CollisionMask = HashMap<CollisionMaskLayer, bool>;

/// A [string](string) specifying a collision mask layer.
///
/// In addition to the listed layers, there is `"layer-13"` through `"layer-55"`. These layers are currently unused by the game but may change. If a mod is going to use one of the unused layers it's recommended to start at the higher layers because the base game will take from the lower ones.
// ================================
// ========= MANUAL PATCH =========

#[derive(Debug, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum CollisionMaskLayerVariants {
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

#[derive(Debug, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum CollisionMaskLayer {
    Variant(CollisionMaskLayerVariants),
    String(String),
}

// TODO: find a solution with serde?

// ========= MANUAL PATCH =========
// ================================

// ================================
// ========= MANUAL PATCH =========

#[derive(Debug, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum CollisionMaskFlags {
    /// Any two entities that both have this option enabled on their prototype and have an identical collision mask layers list will not collide. Other collision mask options are not included in the identical layer list check. This does mean that two different prototypes with the same collision mask layers and this option enabled will not collide.
    NotCollidingWithItself,
    /// Uses the prototypes position rather than its collision box when doing collision checks with tile prototypes. Allows the prototype to overlap colliding tiles up until its center point. This is only respected for character movement and cars driven by players.
    ConsiderTileTransitions,
    /// Any prototype with this collision option will only be checked for collision with other prototype's collision masks if they are a tile.
    CollidingWithTilesOnly,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum CollisionMaskWithFlagsUnion {
    CollisionMaskLayer(CollisionMaskLayer),
    CollisionMaskFlags(CollisionMaskFlags),
}

// TODO: find a solution with serde?

// ========= MANUAL PATCH =========
// ================================

/// A [CollisionMask](CollisionMask) which also includes any flags this mask has.
pub type CollisionMaskWithFlags = HashMap<CollisionMaskWithFlagsUnion, bool>;

#[derive(Debug, Deserialize)]
/// Red, green, blue and alpha values, all in range [0, 1] or all in range [0, 255] if any value is > 1. All values here are optional. Color channels default to `0`, the alpha channel defaults to `1`.
///
/// Similar to [MapPosition](MapPosition), Color allows the short-hand notation of passing an array of exactly 3 or 4 numbers. The game usually expects colors to be in pre-multiplied form (color channels are pre-multiplied by alpha).
///
/// # Examples
///
/// * ```text
/// red1 = {r = 0.5, g = 0, b = 0, a = 0.5}  -- Half-opacity red
/// red2 = {r = 0.5, a = 0.5}                -- Same color as red1
/// black = {}                               -- All channels omitted: black
/// red1_short = {0.5, 0, 0, 0.5}            -- Same color as red1 in short-hand notation
/// ```
pub struct Color {
    pub a: Option<Float>,
    pub b: Option<Float>,
    pub g: Option<Float>,
    pub r: Option<Float>,
}

#[derive(Debug, Deserialize)]
/// Same as [Color](Color), but red, green, blue and alpha values can be any floating point number, without any special handling of the range [1, 255].
pub struct ColorModifier {
    pub a: Option<Float>,
    pub b: Option<Float>,
    pub g: Option<Float>,
    pub r: Option<Float>,
}

#[derive(Debug, Deserialize)]
pub struct CommandAttributesDefinesCommandAttack {
    /// Defaults to `defines.distraction.by_enemy`.
    pub distraction: Option<Distraction>,
    pub target: MaybeCycle<LuaEntity>,
}

#[derive(Debug, Deserialize)]
pub struct CommandAttributesDefinesCommandAttackArea {
    /// Center of the attack area.
    pub destination: MapPosition,
    /// Defaults to `defines.distraction.by_enemy`.
    pub distraction: Option<Distraction>,
    /// Radius of the attack area.
    pub radius: Double,
}

#[derive(Debug, Deserialize)]
pub struct CommandAttributesDefinesCommandBuildBase {
    /// Where to build the base.
    pub destination: MapPosition,
    /// Defaults to `defines.distraction.by_enemy`.
    pub distraction: Option<Distraction>,
    /// Whether the units should ignore expansion candidate chunks. When `false`, they will obey and not build a base in a non-candidate chunk. Defaults to `false`.
    pub ignore_planner: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CommandAttributesDefinesCommandCompound {
    /// The sub-commands.
    pub commands: Vec<Command>,
    /// How the commands should be chained together.
    pub structure_type: CompoundCommand,
}

#[derive(Debug, Deserialize)]
pub struct CommandAttributesDefinesCommandFlee {
    /// Defaults to `defines.distraction.by_enemy`.
    pub distraction: Option<Distraction>,
    /// The entity to flee from
    pub from: MaybeCycle<LuaEntity>,
}

#[derive(Debug, Deserialize)]
pub struct CommandAttributesDefinesCommandGoToLocation {
    /// The position to path to. Either this or `destination_entity` need to be specified. If both are, `destination_entity` is used.
    pub destination: Option<MapPosition>,
    /// The entity to path to. Either this or `destination` need to be specified. If both are, `destination_entity` is used.
    pub destination_entity: Option<MaybeCycle<LuaEntity>>,
    /// Defaults to `defines.distraction.by_enemy`.
    pub distraction: Option<Distraction>,
    /// Flags that affect pathfinder behavior.
    pub pathfind_flags: Option<PathfinderFlags>,
    /// How close the pathfinder needs to get to its destination (in tiles). Defaults to `3`.
    pub radius: Option<Double>,
}

#[derive(Debug, Deserialize)]
pub struct CommandAttributesDefinesCommandGroup {
    /// Defaults to `defines.distraction.by_enemy`.
    pub distraction: Option<Distraction>,
    /// The group whose command to follow.
    pub group: MaybeCycle<LuaUnitGroup>,
    /// Whether the unit will use the group distraction or the commands distraction. Defaults to true.
    pub use_group_distraction: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CommandAttributesDefinesCommandStop {
    /// Defaults to `defines.distraction.by_enemy`.
    pub distraction: Option<Distraction>,
    /// Ticks to wander before successfully completing the command. Default is max uint, which means stop forever.
    pub ticks_to_wait: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct CommandAttributesDefinesCommandWander {
    /// Defaults to `defines.distraction.by_enemy`.
    pub distraction: Option<Distraction>,
    /// Defaults to 10. Does not apply when `wander_in_group` is `true`.
    pub radius: Option<Double>,
    /// Ticks to wander before successfully completing the command. Default is max uint, which means wander forever.
    pub ticks_to_wait: Option<u32>,
    /// When commanding a group, defines how the group will wander. When `true`, the units in the group will wander around inside the group's radius, just like gathering biters. When `false`, the units will wander as a group, ie they will all walk together in the same random direction. Default is true for groups. Passing true for a single unit is an error.
    pub wander_in_group: Option<bool>,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
/// Commands can be given to enemies and unit groups.
pub struct Command {
    /// Type of command. The remaining fields depend on the value of this field.
    #[serde(rename = "type")]
    pub typ: CommandDefine,
    /// Other attributes may be specified depending on `type`:
    pub attributes: Option<CommandAttributes>,
}

/// A string that specifies how the inputs should be compared
///
/// # Notes
///
/// * While the API accepts both versions for `"less/greater than or equal to"` and `"not equal"`, it'll always return `"≥"`, `"≤"` or `"≠"` respectively when reading them back.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComparatorString {
    /// "equal to"
    EqualTo,
    /// "greater than"
    GreaterThan,
    /// "lesser than"
    LesserThan,
    /// "greater than or equal to"
    /// "greater than or equal to"
    GreaterThanOrEqualTo,
    /// "lesser than or equal to"
    /// "lesser than or equal to"
    LesserThanOrEqualTo,
    /// "not equal to"
    /// "not equal to"
    NotEqualTo,
}

#[derive(Debug, Deserialize)]
pub struct ConfigurationChangedData {
    /// `true` when mod prototype migrations have been applied since the last time this save was loaded.
    pub migration_applied: bool,
    /// Dictionary of mod changes. It is indexed by mod name.
    pub mod_changes: HashMap<String, ModChangeData>,
    /// `true` when mod startup settings have changed since the last time this save was loaded.
    pub mod_startup_settings_changed: bool,
    /// New version of the map. Present only when loading map version other than the current version.
    pub new_version: Option<String>,
    /// Old version of the map. Present only when loading map version other than the current version.
    pub old_version: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ConstantCombinatorParameters {
    /// Value of the signal to emit.
    pub count: i32,
    /// Index of the constant combinator's slot to set this signal to.
    pub index: u32,
    /// Signal to emit.
    pub signal: SignalID,
}

#[derive(Debug, Deserialize)]
pub struct CraftingQueueItem {
    /// The amount of items being crafted.
    pub count: u32,
    /// The index of the item in the crafting queue.
    pub index: u32,
    /// The item is a prerequisite for another item in the queue.
    pub prerequisite: bool,
    /// The recipe being crafted.
    pub recipe: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CursorBoxRenderType {
    /// Yellow box.
    Entity,
    /// Red box.
    NotAllowed,
    /// Light blue box.
    Electricity,
    /// Light blue box.
    Pair,
    /// Green box.
    Copy,
    /// White box.
    TrainVisualization,
    /// Light blue box.
    Logistics,
    /// Green box.
    BlueprintSnapRectangle,
}

#[derive(Debug, Deserialize)]
pub struct CustomCommandData {
    /// The name of the command.
    pub name: String,
    /// The parameter passed after the command, if there is one.
    pub parameter: Option<String>,
    /// The player who issued the command, or `nil` if it was issued from the server console.
    pub player_index: Option<u32>,
    /// The tick the command was used in.
    pub tick: u32,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CutsceneWaypointTargetUnion {
    LuaEntity(MaybeCycle<LuaEntity>),
    LuaUnitGroup(MaybeCycle<LuaUnitGroup>),
}

#[derive(Debug, Deserialize)]
pub struct CutsceneWaypoint {
    /// Position to pan the camera to.
    pub position: Option<MapPosition>,
    /// Entity or unit group to pan the camera to.
    pub target: Option<CutsceneWaypointTargetUnion>,
    /// Time in ticks to wait before moving to the next waypoint.
    pub time_to_wait: u32,
    /// How many ticks it will take to reach this waypoint from the previous one.
    pub transition_time: u32,
    /// Zoom level to be set when the waypoint is reached. When not specified, the previous waypoint's zoom is used.
    pub zoom: Option<Double>,
}

#[derive(Debug, Deserialize)]
pub struct DeciderCombinatorParameters {
    /// Specifies how the inputs should be compared. If not specified, defaults to `"<"`.
    pub comparator: Option<ComparatorString>,
    /// Constant to use as the second argument of operation. Defaults to `0`.
    pub constant: Option<u32>,
    /// Defaults to `true`. When `false`, will output a value of `1` for the given `output_signal`.
    pub copy_count_from_input: Option<bool>,
    /// Defaults to blank.
    pub first_signal: Option<SignalID>,
    /// Defaults to blank.
    pub output_signal: Option<SignalID>,
    /// Second signal to use in an operation, if any. If this is not specified, the second argument to a decider combinator's operation is assumed to be the value of `constant`.
    pub second_signal: Option<SignalID>,
}

#[derive(Debug, Deserialize)]
pub struct Decorative {
    pub amount: u8,
    /// The name of the decorative prototype.
    pub name: String,
    pub position: TilePosition,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum DecorativePrototypeFilterAttributesMaskUnion {
    CollisionMask(CollisionMask),
    CollisionMaskWithFlags(CollisionMaskWithFlags),
}

#[derive(Debug, Deserialize)]
pub struct DecorativePrototypeFilterAttributesCollisionMask {
    pub mask: DecorativePrototypeFilterAttributesMaskUnion,
    /// How to filter: `"collides"`, `"layers-equals"`, `"contains-any"` or `"contains-all"`
    pub mask_mode: String,
}

#[derive(Debug, Deserialize)]
pub enum DecorativePrototypeFilterAttributes {
    CollisionMask(DecorativePrototypeFilterAttributesCollisionMask),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct DecorativePrototypeFilter {
    /// The condition to filter on. One of `"decal"`, `"autoplace"`, `"collision-mask"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<DecorativePrototypeFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct DecorativeResult {
    pub amount: u32,
    pub decorative: MaybeCycle<LuaDecorativePrototype>,
    pub position: TilePosition,
}

#[derive(Debug, Deserialize)]
/// Technology and recipe difficulty settings. Updating any of the attributes will immediately take effect in the game engine.
pub struct DifficultySettings {
    pub recipe_difficulty: DifficultySettingsRecipeDifficulty,
    /// Either `"after-victory"`, `"always"` or `"never"`. Changing this to `"always"` or `"after-victory"` does not automatically unlock the research queue. See [LuaForce](LuaForce) for that.
    pub research_queue_setting: String,
    pub technology_difficulty: DifficultySettingsTechnologyDifficulty,
    /// A value in range [0.001, 1000].
    pub technology_price_multiplier: Double,
}

#[derive(Debug, Deserialize)]
pub struct DisplayResolution {
    pub height: u32,
    pub width: u32,
}

#[derive(Debug, Deserialize)]
pub struct DragTarget {
    /// If the wire being dragged is a circuit wire this is the connector id.
    pub target_circuit_id: Option<CircuitConnectorId>,
    pub target_entity: MaybeCycle<LuaEntity>,
    /// If the wire being dragged is copper wire this is the wire id.
    pub target_wire_id: Option<WireConnectionId>,
}

#[derive(Debug, Deserialize)]
/// These values represent a percentual increase in evolution. This means a value of `0.1` would increase evolution by 10%.
pub struct EnemyEvolutionMapSettings {
    /// The amount evolution progresses for every destroyed spawner. Defaults to `0.002`.
    pub destroy_factor: Double,
    /// Whether enemy evolution is enabled at all.
    pub enabled: bool,
    /// The amount evolution progresses for every unit of pollution. Defaults to `0.0000009`.
    pub pollution_factor: Double,
    /// The amount evolution naturally progresses by every second. Defaults to `0.000004`.
    pub time_factor: Double,
}

#[derive(Debug, Deserialize)]
/// Candidate chunks are given scores to determine which one of them should be expanded into. This score takes into account various settings noted below. The iteration is over a square region centered around the chunk for which the calculation is done, and includes the central chunk as well. Distances are calculated as [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry).
///
/// The pseudocode algorithm to determine a chunk's score is as follows:
///
/// ```text
/// player = 0
/// for neighbour in all chunks within enemy_building_influence_radius from chunk:
///   player += number of player buildings on neighbour
///           * building_coefficient
///           * neighbouring_chunk_coefficient^distance(chunk, neighbour)
///
/// base = 0
/// for neighbour in all chunk within friendly_base_influence_radius from chunk:
///   base += num of enemy bases on neighbour
///           * other_base_coefficient
///           * neighbouring_base_chunk_coefficient^distance(chunk, neighbour)
///
/// score(chunk) = 1 / (1 + player + base)
/// ```
pub struct EnemyExpansionMapSettings {
    /// Defaults to `0.1`.
    pub building_coefficient: Double,
    /// Whether enemy expansion is enabled at all.
    pub enabled: bool,
    /// Defaults to `2`.
    pub enemy_building_influence_radius: u32,
    /// Defaults to `2`.
    pub friendly_base_influence_radius: u32,
    /// A chunk has to have at most this high of a percentage of unbuildable tiles for it to be considered a candidate to avoid chunks full of water as candidates. Defaults to `0.9`, or 90%.
    pub max_colliding_tiles_coefficient: Double,
    /// The maximum time between expansions in ticks. The actual cooldown is adjusted to the current evolution levels. Defaults to `60*3,600=216,000` ticks.
    pub max_expansion_cooldown: u32,
    /// Distance in chunks from the furthest base around to prevent expansions from reaching too far into the player's territory. Defaults to `7`.
    pub max_expansion_distance: u32,
    /// The minimum time between expansions in ticks. The actual cooldown is adjusted to the current evolution levels. Defaults to `4*3,600=14,400` ticks.
    pub min_expansion_cooldown: u32,
    /// Defaults to `0.4`.
    pub neighbouring_base_chunk_coefficient: Double,
    /// Defaults to `0.5`.
    pub neighbouring_chunk_coefficient: Double,
    /// Defaults to `2.0`.
    pub other_base_coefficient: Double,
    /// The maximum size of a biter group that goes to build a new base. This is multiplied by the evolution factor. Defaults to `20`.
    pub settler_group_max_size: u32,
    /// The minimum size of a biter group that goes to build a new base. This is multiplied by the evolution factor. Defaults to `5`.
    pub settler_group_min_size: u32,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum EntityPrototypeFilterAttributesMaskUnion {
    CollisionMask(CollisionMask),
    CollisionMaskWithFlags(CollisionMaskWithFlags),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum EntityPrototypeFilterAttributesNameUnion {
    String(String),
    Array(Vec<String>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum EntityPrototypeFilterAttributesTypeUnion {
    String(String),
    Array(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct EntityPrototypeFilterAttributesBuildBaseEvolutionRequirement {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: Double,
}

#[derive(Debug, Deserialize)]
pub struct EntityPrototypeFilterAttributesCollisionMask {
    pub mask: EntityPrototypeFilterAttributesMaskUnion,
    /// How to filter: `"collides"`, `"layers-equals"`, `"contains-any"` or `"contains-all"`
    pub mask_mode: String,
}

#[derive(Debug, Deserialize)]
pub struct EntityPrototypeFilterAttributesCraftingCategory {
    /// Matches if the prototype is for a crafting machine with this crafting category.
    pub crafting_category: String,
}

#[derive(Debug, Deserialize)]
pub struct EntityPrototypeFilterAttributesEmissions {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: Double,
}

#[derive(Debug, Deserialize)]
pub struct EntityPrototypeFilterAttributesFlag {
    /// One of the values in [EntityPrototypeFlags](EntityPrototypeFlags).
    pub flag: String,
}

#[derive(Debug, Deserialize)]
pub struct EntityPrototypeFilterAttributesName {
    /// The prototype name, or list of acceptable names.
    pub name: EntityPrototypeFilterAttributesNameUnion,
}

#[derive(Debug, Deserialize)]
pub struct EntityPrototypeFilterAttributesSelectionPriority {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: u8,
}

#[derive(Debug, Deserialize)]
pub struct EntityPrototypeFilterAttributesType {
    /// The prototype type, or a list of acceptable types.
    #[serde(rename = "type")]
    pub typ: EntityPrototypeFilterAttributesTypeUnion,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct EntityPrototypeFilter {
    /// The condition to filter on. One of `"flying-robot"`, `"robot-with-logistics-interface"`, `"rail"`, `"ghost"`, `"explosion"`, `"vehicle"`, `"crafting-machine"`, `"rolling-stock"`, `"turret"`, `"transport-belt-connectable"`, `"wall-connectable"`, `"buildable"`, `"placable-in-editor"`, `"clonable"`, `"selectable"`, `"hidden"`, `"entity-with-health"`, `"building"`, `"fast-replaceable"`, `"uses-direction"`, `"minable"`, `"circuit-connectable"`, `"autoplace"`, `"blueprintable"`, `"item-to-place"`, `"name"`, `"type"`, `"collision-mask"`, `"flag"`, `"build-base-evolution-requirement"`, `"selection-priority"`, `"emissions"`, `"crafting-category"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<EntityPrototypeFilterAttributes>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum EntityPrototypeFlagsUnion {
    /// Prevents the entity from being rotated before or after placement.
    NotRotatable,
    /// Determines the default force when placing entities in the map editor and using the "AUTO" option for the force.
    PlaceableNeutral,
    /// Determines the default force when placing entities in the map editor and using the "AUTO" option for the force.
    PlaceablePlayer,
    /// Determines the default force when placing entities in the map editor and using the "AUTO" option for the force.
    PlaceableEnemy,
    /// Determines whether the entity needs to be aligned with the invisible grid within the world. Most entities are confined in this way, with a few exceptions such as trees and land mines.
    PlaceableOffGrid,
    /// Makes it possible to blueprint, deconstruct, and repair the entity (which can be turned off again using the specific flags). Makes it possible for the biter AI to target the entity as a distraction. Enables dust to automatically be created when building the entity. If the entity does not have a `map_color` set, this flag makes the entity appear on the map with the default color specified by the UtilityConstants.
    PlayerCreation,
    /// Uses 45 degree angle increments when selecting direction.
    #[serde(rename = "building-direction-8-way")]
    BuildingDirection8Way,
    /// Used to automatically detect the proper direction of the entity if possible. Used by the pump, train stop, and train signal by default.
    FilterDirections,
    /// Fast replace will not apply when building while moving.
    FastReplaceableNoBuildWhileMoving,
    /// Used to specify that the entity breathes air, and is thus affected by poison.
    BreathsAir,
    /// Used to specify that the entity can not be 'healed' by repair packs.
    NotRepairable,
    /// Prevents the entity from being drawn on the map.
    NotOnMap,
    /// Prevents the entity from being deconstructed.
    NotDeconstructable,
    /// Prevents the entity from being part of a blueprint.
    NotBlueprintable,
    /// Hides the entity from the bonus GUI and from the "made in"-property of recipe tooltips.
    Hidden,
    /// Hides the alt-info of this entity when in alt-mode.
    HideAltInfo,
    /// Does not fast replace this entity over other entity types when building while moving.
    FastReplaceableNoCrossTypeWhileMoving,
    NoGapFillWhileBuilding,
    /// Does not apply fire stickers to the entity.
    NotFlammable,
    /// Prevents inserters and loaders from taking items from this entity.
    NoAutomatedItemRemoval,
    /// Prevents inserters and loaders from inserting items into this entity.
    NoAutomatedItemInsertion,
    /// Prevents the entity from being copy-pasted.
    NoCopyPaste,
    /// Disallows selection of the entity even when a selection box is specified for other reasons. For example, selection boxes are used to determine the size of outlines to be shown when highlighting entities inside electric pole ranges.
    NotSelectableInGame,
    /// Prevents the entity from being selected by the upgrade planner.
    NotUpgradable,
    /// Prevents the entity from being shown in the kill statistics.
    NotInKillStatistics,
    /// Prevents the entity from being shown in the "made in" list in recipe tooltips.
    NotInMadeIn,
}

/// A set of flags. Active flags are in the dictionary as `true`, while inactive flags aren't present at all.
///
/// By default, none of these flags are set.
pub type EntityPrototypeFlags = HashMap<EntityPrototypeFlagsUnion, bool>;

/// An entity prototype may be specified in one of three ways.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum EntityPrototypeIdentification {
    /// The entity.
    LuaEntity(MaybeCycle<LuaEntity>),
    /// The entity prototype.
    LuaEntityPrototype(MaybeCycle<LuaEntityPrototype>),
    /// The prototype name.
    String(String),
}

#[derive(Debug, Deserialize)]
/// A table used to define a manual shape for a piece of equipment.
pub struct EquipmentPoint {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Deserialize)]
/// Position inside an equipment grid. This uses the same format as [MapPosition](MapPosition), meaning it can be specified either with or without explicit keys.
///
/// # Examples
///
/// * Explicit definition:
/// ```text
/// {x = 5, y = 2}
/// {y = 2, x = 5}
/// ```
/// * Shorthand:
/// ```text
/// {1, 2}
/// ```
pub struct EquipmentPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum EquipmentPrototypeFilterAttributesTypeUnion {
    String(String),
    Array(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct EquipmentPrototypeFilterAttributesType {
    /// The prototype type, or a list of acceptable types.
    #[serde(rename = "type")]
    pub typ: EquipmentPrototypeFilterAttributesTypeUnion,
}

#[derive(Debug, Deserialize)]
pub enum EquipmentPrototypeFilterAttributes {
    Type(EquipmentPrototypeFilterAttributesType),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct EquipmentPrototypeFilter {
    /// The condition to filter on. One of `"item-to-place"`, `"type"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<EquipmentPrototypeFilterAttributes>,
}

#[derive(Debug, Deserialize)]
/// Information about the event that has been raised. The table can also contain other fields depending on the type of event. See [the list of Factorio events](events.html) for more information on these.
pub struct EventData {
    /// The name of the mod that raised the event if it was raised using [LuaBootstrap::raise_event](LuaBootstrap::raise_event).
    pub mod_name: Option<String>,
    /// The identifier of the event this handler was registered to.
    pub name: Events,
    /// The tick during which the event happened.
    pub tick: u32,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
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

/// Used to filter out irrelevant event callbacks in a performant way.
///
/// # Notes
///
/// * Filters are always used as an array of filters of a specific type. Every filter can only be used with its corresponding event, and different types of event filters can not be mixed.
pub type EventFilter = Vec<EventFilterUnion>;

#[derive(Debug, Deserialize)]
pub struct Fluid {
    /// Amount of the fluid.
    pub amount: Double,
    /// Fluid prototype name of the fluid.
    pub name: String,
    /// The temperature. When reading from [LuaFluidBox](LuaFluidBox), this field will always be present. It is not necessary to specify it when writing, however. When not specified, the fluid will be set to the fluid's default temperature as specified in the fluid's prototype.
    pub temperature: Option<Double>,
}

#[derive(Debug, Deserialize)]
/// A definition of a fluidbox connection point.
pub struct FluidBoxConnection {
    /// The maximum tile distance this underground connection can connect at if this is an underground pipe.
    pub max_underground_distance: Option<u32>,
    /// The 4 cardinal direction connection points for this pipe. This vector is a table with `x` and `y` keys instead of an array.
    pub positions: Vec<Vector>,
    /// The connection type: "input", "output", or "input-output".
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct FluidBoxFilter {
    /// The maximum temperature allowed into the fluidbox.
    pub maximum_temperature: Double,
    /// The minimum temperature allowed into the fluidbox.
    pub minimum_temperature: Double,
    /// Fluid prototype name of the filtered fluid.
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct FluidBoxFilterSpec {
    /// Force the filter to be set, regardless of current fluid content.
    pub force: Option<bool>,
    /// The maximum temperature allowed into the fluidbox.
    pub maximum_temperature: Option<Double>,
    /// The minimum temperature allowed into the fluidbox.
    pub minimum_temperature: Option<Double>,
    /// Fluid prototype name of the filtered fluid.
    pub name: String,
}

/// A fluid may be specified in one of three ways.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum FluidIdentification {
    /// The fluid name.
    String(String),
    /// The fluid prototype.
    LuaFluidPrototype(MaybeCycle<LuaFluidPrototype>),
    /// The fluid.
    Fluid(Fluid),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum FluidPrototypeFilterAttributesNameUnion {
    String(String),
    Array(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct FluidPrototypeFilterAttributesDefaultTemperature {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: Double,
}

#[derive(Debug, Deserialize)]
pub struct FluidPrototypeFilterAttributesEmissionsMultiplier {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: Double,
}

#[derive(Debug, Deserialize)]
pub struct FluidPrototypeFilterAttributesFuelValue {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: Double,
}

#[derive(Debug, Deserialize)]
pub struct FluidPrototypeFilterAttributesGasTemperature {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: Double,
}

#[derive(Debug, Deserialize)]
pub struct FluidPrototypeFilterAttributesHeatCapacity {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: Double,
}

#[derive(Debug, Deserialize)]
pub struct FluidPrototypeFilterAttributesMaxTemperature {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: Double,
}

#[derive(Debug, Deserialize)]
pub struct FluidPrototypeFilterAttributesName {
    /// The prototype name, or list of acceptable names.
    pub name: FluidPrototypeFilterAttributesNameUnion,
}

#[derive(Debug, Deserialize)]
pub struct FluidPrototypeFilterAttributesSubgroup {
    /// A [LuaGroup](LuaGroup) (subgroup) name
    pub subgroup: String,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct FluidPrototypeFilter {
    /// The condition to filter on. One of `"hidden"`, `"name"`, `"subgroup"`, `"default-temperature"`, `"max-temperature"`, `"heat-capacity"`, `"fuel-value"`, `"emissions-multiplier"`, `"gas-temperature"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<FluidPrototypeFilterAttributes>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ForceCondition {
    /// All forces pass.
    All,
    /// Forces which will attack pass.
    Enemy,
    /// Forces which won't attack pass.
    Ally,
    /// Forces which are friends pass.
    Friend,
    /// Forces which are not friends pass.
    NotFriend,
    /// The same force pass.
    Same,
    /// The non-same forces pass.
    NotSame,
}

/// A force may be specified in one of three ways.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ForceIdentification {
    /// The force index.
    Uint8(u8),
    /// The force name.
    String(String),
    /// A reference to [LuaForce](LuaForce) may be passed directly.
    LuaForce(MaybeCycle<LuaForce>),
}

#[derive(Debug, Deserialize)]
/// Parameters that affect the look and control of the game. Updating any of the member attributes here will immediately take effect in the game engine.
pub struct GameViewSettings {
    /// Show the flashing alert icons next to the player's toolbar.
    pub show_alert_gui: bool,
    /// Show the controller GUI elements. This includes the toolbar, the selected tool slot, the armour slot, and the gun and ammunition slots.
    pub show_controller_gui: bool,
    /// Show overlay icons on entities. Also known as "alt-mode".
    pub show_entity_info: bool,
    /// Shows or hides the view options when map is opened.
    pub show_map_view_options: bool,
    /// Show the chart in the upper right-hand corner of the screen.
    pub show_minimap: bool,
    /// Shows or hides quickbar of shortcuts.
    pub show_quickbar: bool,
    /// When `true` (`false` is default), the rails will always show the rail block visualisation.
    pub show_rail_block_visualisation: bool,
    /// Show research progress and name in the upper right-hand corner of the screen.
    pub show_research_info: bool,
    /// Shows or hides the shortcut bar.
    pub show_shortcut_bar: bool,
    /// Shows or hides the buttons row.
    pub show_side_menu: bool,
    /// When `true` (the default), mousing over an entity will select it. Otherwise, moving the mouse won't update entity selection.
    pub update_entity_selection: bool,
}

#[derive(Debug, Deserialize)]
pub struct GuiAnchor {
    pub gui: RelativeGuiType,
    /// If provided, only anchors the GUI element when the opened thing matches the name. `name` takes precedence over `names`.
    pub name: Option<String>,
    /// If provided, only anchors the GUI element when the opened thing matches one of the names. When reading an anchor, `names` is always populated.
    pub names: Option<Vec<String>>,
    pub position: RelativeGuiPosition,
    /// If provided, only anchors the GUI element when the opened things type matches the type.
    #[serde(rename = "type")]
    pub typ: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GuiArrowSpecificationAttributesCraftingQueue {
    /// Index in the crafting queue to point to.
    pub crafting_queueindex: u32,
}

#[derive(Debug, Deserialize)]
pub struct GuiArrowSpecificationAttributesEntity {
    pub entity: MaybeCycle<LuaEntity>,
}

#[derive(Debug, Deserialize)]
pub struct GuiArrowSpecificationAttributesItemStack {
    /// Which inventory the stack is in.
    pub inventory_index: Inventory,
    /// Which stack to point to.
    pub item_stack_index: u32,
    /// Must be either `"player"`, `"target"`, `"player-quickbar"` or `"player-equipment-bar"`.
    pub source: String,
}

#[derive(Debug, Deserialize)]
pub struct GuiArrowSpecificationAttributesPosition {
    pub position: MapPosition,
}

#[derive(Debug, Deserialize)]
pub enum GuiArrowSpecificationAttributes {
    CraftingQueue(GuiArrowSpecificationAttributesCraftingQueue),
    Entity(GuiArrowSpecificationAttributesEntity),
    ItemStack(GuiArrowSpecificationAttributesItemStack),
    Position(GuiArrowSpecificationAttributesPosition),
}

#[derive(Debug, Deserialize)]
/// Used for specifying where a GUI arrow should point to.
pub struct GuiArrowSpecification {
    /// This determines which of the following fields will be required. Must be one of `"nowhere"` (will remove the arrow entirely), `"goal"` (will point to the current goal), `"entity_info"`, `"active_window"`, `"entity"`, `"position"`, `"crafting_queue"` or `"item_stack"` (will point to a given item stack in an inventory). Depending on this value, other fields may have to be specified.
    #[serde(rename = "type")]
    pub typ: String,
    /// Other attributes may be specified depending on `type`:
    pub attributes: Option<GuiArrowSpecificationAttributes>,
}

#[derive(Debug, Deserialize)]
/// Screen coordinates of a GUI element in a [LuaGui](LuaGui). This uses the same format as [TilePosition](TilePosition), meaning it can be specified either with or without explicit keys.
pub struct GuiLocation {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Deserialize)]
pub struct HeatConnection {
    pub direction: Direction,
    pub position: Vector,
}

#[derive(Debug, Deserialize)]
/// The settings used by a heat-interface type entity.
pub struct HeatSetting {
    /// `"at-least"`, `"at-most"`, `"exactly"`, `"add"`, or `"remove"`. Defaults to `"at-least"`.
    pub mode: Option<String>,
    /// The target temperature. Defaults to the minimum temperature of the heat buffer.
    pub temperature: Option<Double>,
}

#[derive(Debug, Deserialize)]
/// A single filter used by an infinity-filters instance.
pub struct InfinityInventoryFilter {
    /// The count of the filter.
    pub count: Option<u32>,
    /// The index of this filter in the filters list. Not required when writing a filter.
    pub index: u32,
    /// `"at-least"`, `"at-most"`, or `"exactly"`. Defaults to `"at-least"`.
    pub mode: Option<String>,
    /// Name of the item.
    pub name: String,
}

#[derive(Debug, Deserialize)]
/// A single filter used by an infinity-pipe type entity.
pub struct InfinityPipeFilter {
    /// `"at-least"`, `"at-most"`, `"exactly"`, `"add"`, or `"remove"`. Defaults to `"at-least"`.
    pub mode: Option<String>,
    /// Name of the fluid.
    pub name: String,
    /// The fill percentage the pipe (e.g. 0.5 for 50%). Can't be negative.
    pub percentage: Option<Double>,
    /// The temperature of the fluid. Defaults to the default/minimum temperature of the fluid.
    pub temperature: Option<Double>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum IngredientCatalystAmountUnion {
    Uint(u32),
    Double(Double),
}

#[derive(Debug, Deserialize)]
pub struct IngredientAttributesFluid {
    /// The maximum fluid temperature allowed.
    pub maximum_temperature: Option<Double>,
    /// The minimum fluid temperature required.
    pub minimum_temperature: Option<Double>,
}

#[derive(Debug, Deserialize)]
pub enum IngredientAttributes {
    Fluid(IngredientAttributesFluid),
}

#[derive(Debug, Deserialize)]
pub struct Ingredient {
    /// Amount of the item or fluid.
    pub amount: Double,
    /// How much of this ingredient is a catalyst.
    pub catalyst_amount: Option<IngredientCatalystAmountUnion>,
    /// Prototype name of the required item or fluid.
    pub name: String,
    /// `"item"` or `"fluid"`.
    #[serde(rename = "type")]
    pub typ: String,
    /// Other attributes may be specified depending on `type`:
    pub attributes: Option<IngredientAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct InserterCircuitConditions {
    pub circuit: Option<CircuitCondition>,
    pub logistics: Option<CircuitCondition>,
}

#[derive(Debug, Deserialize)]
pub struct InventoryFilter {
    /// Position of the corresponding filter slot.
    pub index: u32,
    /// Item prototype name of the item to filter.
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ItemPrototypeFilterAttributesNameUnion {
    String(String),
    Array(Vec<String>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ItemPrototypeFilterAttributesTypeUnion {
    String(String),
    Array(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct ItemPrototypeFilterAttributesBurntResult {
    /// Filters for the burnt result.
    pub elem_filters: Option<Vec<ItemPrototypeFilter>>,
}

#[derive(Debug, Deserialize)]
pub struct ItemPrototypeFilterAttributesDefaultRequestAmount {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: u32,
}

#[derive(Debug, Deserialize)]
pub struct ItemPrototypeFilterAttributesFlag {
    /// One of the values in [ItemPrototypeFlags](ItemPrototypeFlags).
    pub flag: String,
}

#[derive(Debug, Deserialize)]
pub struct ItemPrototypeFilterAttributesFuelAccelerationMultiplier {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: Double,
}

#[derive(Debug, Deserialize)]
pub struct ItemPrototypeFilterAttributesFuelCategory {
    /// A [LuaFuelCategoryPrototype](LuaFuelCategoryPrototype) name
    pub fuel_category: String,
}

#[derive(Debug, Deserialize)]
pub struct ItemPrototypeFilterAttributesFuelEmissionsMultiplier {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: Double,
}

#[derive(Debug, Deserialize)]
pub struct ItemPrototypeFilterAttributesFuelTopSpeedMultiplier {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: Double,
}

#[derive(Debug, Deserialize)]
pub struct ItemPrototypeFilterAttributesFuelValue {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: Double,
}

#[derive(Debug, Deserialize)]
pub struct ItemPrototypeFilterAttributesName {
    /// The prototype name, or list of acceptable names.
    pub name: ItemPrototypeFilterAttributesNameUnion,
}

#[derive(Debug, Deserialize)]
pub struct ItemPrototypeFilterAttributesPlaceAsTile {
    /// Filters for the placed tile.
    pub elem_filters: Option<Vec<TilePrototypeFilter>>,
}

#[derive(Debug, Deserialize)]
pub struct ItemPrototypeFilterAttributesPlaceResult {
    /// Filters for the place result.
    pub elem_filters: Option<Vec<EntityPrototypeFilter>>,
}

#[derive(Debug, Deserialize)]
pub struct ItemPrototypeFilterAttributesPlacedAsEquipmentResult {
    /// Filters for the placed equipment.
    pub elem_filters: Option<Vec<EquipmentPrototypeFilter>>,
}

#[derive(Debug, Deserialize)]
pub struct ItemPrototypeFilterAttributesStackSize {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: u32,
}

#[derive(Debug, Deserialize)]
pub struct ItemPrototypeFilterAttributesSubgroup {
    /// A [LuaGroup](LuaGroup) (subgroup) name
    pub subgroup: String,
}

#[derive(Debug, Deserialize)]
pub struct ItemPrototypeFilterAttributesType {
    /// The prototype type, or a list of acceptable types.
    #[serde(rename = "type")]
    pub typ: ItemPrototypeFilterAttributesTypeUnion,
}

#[derive(Debug, Deserialize)]
pub struct ItemPrototypeFilterAttributesWireCount {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: u32,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct ItemPrototypeFilter {
    /// The condition to filter on. One of `"tool"`, `"mergeable"`, `"item-with-inventory"`, `"selection-tool"`, `"item-with-label"`, `"has-rocket-launch-products"`, `"fuel"`, `"place-result"`, `"burnt-result"`, `"place-as-tile"`, `"placed-as-equipment-result"`, `"name"`, `"type"`, `"flag"`, `"subgroup"`, `"fuel-category"`, `"stack-size"`, `"default-request-amount"`, `"wire-count"`, `"fuel-value"`, `"fuel-acceleration-multiplier"`, `"fuel-top-speed-multiplier"`, `"fuel-emissions-multiplier"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<ItemPrototypeFilterAttributes>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ItemPrototypeFlagsUnion {
    /// Determines whether the logistics areas of roboports should be drawn when holding this item. Used by the deconstruction planner by default.
    DrawLogisticOverlay,
    /// Hides the item in the logistic requests and filters GUIs (among others).
    Hidden,
    /// Always shows the item in the logistic requests and filters GUIs (among others) even when the recipe for that item is locked.
    AlwaysShow,
    /// Hides the item from the bonus GUI.
    HideFromBonusGui,
    /// Hides the item from the tooltip that's shown when hovering over a burner inventory.
    HideFromFuelTooltip,
    /// Prevents the item from being stacked. It also prevents the item from stacking in assembling machine input slots, which can otherwise exceed the item stack size if required by the recipe. Additionally, the item does not show an item count when in the cursor.
    NotStackable,
    /// Makes the item act as an extension to the inventory that it is placed in. Only has an effect for items with inventory.
    CanExtendInventory,
    /// Makes construction bots prefer this item when building the entity specified by its `place_result`.
    PrimaryPlaceResult,
    /// Allows the item to be opened by the player, firing the `on_mod_item_opened` event. Only has an effect for selection tool items.
    ModOpenable,
    /// Makes it so the item is deleted when clearing the cursor, instead of being put into the player's inventory. The copy-paste tools use this by default, for example.
    OnlyInCursor,
    /// Allows the item to be spawned by a quickbar shortcut or custom input.
    Spawnable,
}

/// A set of flags. Active flags are in the dictionary as `true`, while inactive flags aren't present at all.
///
/// By default, none of these flags are set.
pub type ItemPrototypeFlags = HashMap<ItemPrototypeFlagsUnion, bool>;

/// An item prototype may be specified in one of three ways.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ItemPrototypeIdentification {
    /// The item.
    LuaItemStack(MaybeCycle<LuaItemStack>),
    /// The item prototype.
    LuaItemPrototype(MaybeCycle<LuaItemPrototype>),
    /// The prototype name.
    String(String),
}

#[derive(Debug, Deserialize)]
pub struct ItemStackDefinition {
    /// Amount of ammo in the ammo items in the stack.
    pub ammo: Option<Double>,
    /// Number of items the stack holds. If not specified, defaults to `1`.
    pub count: Option<u32>,
    /// Durability of the tool items in the stack.
    pub durability: Option<Double>,
    /// Health of the items in the stack. Defaults to `1.0`.
    pub health: Option<Float>,
    /// Prototype name of the item the stack holds.
    pub name: String,
    /// Tags of the items with tags in the stack.
    pub tags: Option<Vec<String>>,
}

/// An item may be specified in one of two ways.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ItemStackIdentification {
    SimpleItemStack(SimpleItemStack),
    LuaItemStack(MaybeCycle<LuaItemStack>),
}

#[derive(Debug, Deserialize)]
pub struct ItemStackLocation {
    pub inventory: Inventory,
    pub slot: u32,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum LocalisedStringUnion {
    String(String),
    LocalisedString(LocalisedString),
}

/// Localised strings are a way to support translation of in-game text. It is an array where the first element is the key and the remaining elements are parameters that will be substituted for placeholders in the template designated by the key.
///
/// The key identifies the string template. For example, `"gui-alert-tooltip.attack"` (for the template `"__1__
///     objects are being damaged"`; see the file `data/core/locale/en.cfg`).
///
/// The template can contain placeholders such as `__1__` or `__2__`. These will be replaced by the respective parameter in the LocalisedString. The parameters themselves can be other localised strings, which will be processed recursively in the same fashion. Localised strings can not be recursed deeper than 20 levels and can not have more than 20 parameters.
///
/// There are two special flags for the localised string, indicated by the key being a particular string. First, if the key is the empty string (`""`), then all parameters will be concatenated (after processing, if any are localised strings themselves). Second, if the key is a question mark (`"?"`), then the first valid parameter will be used. A parameter can be invalid if its name doesn't match any string template. If no parameters are valid, the last one is returned. This is useful to implement a fallback for missing locale templates.
///
/// Furthermore, when an API function expects a localised string, it will also accept a regular string (i.e. not a table) which will not be translated, as well as a number, boolean or `nil`, which will be converted to their textual representation.
///
/// # Examples
///
/// * In the English translation, this will print `"No ammo"`; in the Czech translation, it will print `"Bez munice"`:
/// ```text
/// game.player.print({"description.no-ammo"})
/// ```
///  The `description.no-ammo` template contains no placeholders, so no further parameters are necessary.
/// * In the English translation, this will print `"Durability: 5/9"`; in the Japanese one, it will print `"耐久度: 5/9"`:
/// ```text
/// game.player.print({"description.durability", 5, 9})
/// ```
/// * This will print `"hello"` in all translations:
/// ```text
/// game.player.print({"", "hello"})
/// ```
/// * This will print `"Iron plate: 60"` in the English translation and `"Eisenplatte: 60"` in the German translation.
/// ```text
/// game.print({"", {"item-name.iron-plate"}, ": ", 60})
/// ```
/// * As an example of a localised string with fallback, consider this:
/// ```text
/// {"?", {"", {"entity-description.furnace"}, "\n"}, {"item-description.furnace"}, "optional fallback"}
/// ```
///  If `entity-description.furnace` exists, it is concatenated with `"\n"` and returned. Otherwise, if `item-description.furnace` exists, it is returned as-is. Otherwise, `"optional fallback"` is returned. If this value wasn't specified, the translation result would be `"Unknown key: 'item-description.furnace'"`.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum LocalisedString {
    String(String),
    Number(f64),
    Boolean(bool),
    LuaObject,
    Nil,
    Array(Vec<LocalisedStringUnion>),
}

#[derive(Debug, Deserialize)]
pub struct LogisticFilter {
    /// The count for this filter.
    pub count: u32,
    /// The index this filter applies to.
    pub index: u32,
    /// The item name for this filter.
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LogisticParameters {
    pub max: Option<u32>,
    pub min: Option<u32>,
    /// The item. `nil` clears the filter.
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Loot {
    /// Maximum amount of loot to drop.
    pub count_max: Double,
    /// Minimum amount of loot to drop.
    pub count_min: Double,
    /// Item prototype name of the result.
    pub item: String,
    /// Probability that any loot at all will drop, as a number in range [0, 1].
    pub probability: Double,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityClonedEventFilterAttributesGhostName {
    /// The ghost prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityClonedEventFilterAttributesGhostType {
    /// The ghost prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityClonedEventFilterAttributesName {
    /// The prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityClonedEventFilterAttributesType {
    /// The prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub enum LuaEntityClonedEventFilterAttributes {
    GhostName(LuaEntityClonedEventFilterAttributesGhostName),
    GhostType(LuaEntityClonedEventFilterAttributesGhostType),
    Name(LuaEntityClonedEventFilterAttributesName),
    Type(LuaEntityClonedEventFilterAttributesType),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct LuaEntityClonedEventFilter {
    /// The condition to filter on. One of `"ghost"`, `"rail"`, `"rail-signal"`, `"rolling-stock"`, `"robot-with-logistics-interface"`, `"vehicle"`, `"turret"`, `"crafting-machine"`, `"wall-connectable"`, `"transport-belt-connectable"`, `"circuit-network-connectable"`, `"type"`, `"name"`, `"ghost_type"`, `"ghost_name"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<LuaEntityClonedEventFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityDamagedEventFilterAttributesDamageType {
    /// A [LuaDamagePrototype](LuaDamagePrototype) name
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityDamagedEventFilterAttributesFinalDamageAmount {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: Float,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityDamagedEventFilterAttributesFinalHealth {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: Float,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityDamagedEventFilterAttributesGhostName {
    /// The ghost prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityDamagedEventFilterAttributesGhostType {
    /// The ghost prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityDamagedEventFilterAttributesName {
    /// The prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityDamagedEventFilterAttributesOriginalDamageAmount {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: Float,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityDamagedEventFilterAttributesType {
    /// The prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct LuaEntityDamagedEventFilter {
    /// The condition to filter on. One of `"ghost"`, `"rail"`, `"rail-signal"`, `"rolling-stock"`, `"robot-with-logistics-interface"`, `"vehicle"`, `"turret"`, `"crafting-machine"`, `"wall-connectable"`, `"transport-belt-connectable"`, `"circuit-network-connectable"`, `"type"`, `"name"`, `"ghost_type"`, `"ghost_name"`, `"original-damage-amount"`, `"final-damage-amount"`, `"damage-type"`, `"final-health"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<LuaEntityDamagedEventFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityDeconstructionCancelledEventFilterAttributesGhostName {
    /// The ghost prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityDeconstructionCancelledEventFilterAttributesGhostType {
    /// The ghost prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityDeconstructionCancelledEventFilterAttributesName {
    /// The prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityDeconstructionCancelledEventFilterAttributesType {
    /// The prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub enum LuaEntityDeconstructionCancelledEventFilterAttributes {
    GhostName(LuaEntityDeconstructionCancelledEventFilterAttributesGhostName),
    GhostType(LuaEntityDeconstructionCancelledEventFilterAttributesGhostType),
    Name(LuaEntityDeconstructionCancelledEventFilterAttributesName),
    Type(LuaEntityDeconstructionCancelledEventFilterAttributesType),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct LuaEntityDeconstructionCancelledEventFilter {
    /// The condition to filter on. One of `"ghost"`, `"rail"`, `"rail-signal"`, `"rolling-stock"`, `"robot-with-logistics-interface"`, `"vehicle"`, `"turret"`, `"crafting-machine"`, `"wall-connectable"`, `"transport-belt-connectable"`, `"circuit-network-connectable"`, `"type"`, `"name"`, `"ghost_type"`, `"ghost_name"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<LuaEntityDeconstructionCancelledEventFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityDiedEventFilterAttributesGhostName {
    /// The ghost prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityDiedEventFilterAttributesGhostType {
    /// The ghost prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityDiedEventFilterAttributesName {
    /// The prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityDiedEventFilterAttributesType {
    /// The prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub enum LuaEntityDiedEventFilterAttributes {
    GhostName(LuaEntityDiedEventFilterAttributesGhostName),
    GhostType(LuaEntityDiedEventFilterAttributesGhostType),
    Name(LuaEntityDiedEventFilterAttributesName),
    Type(LuaEntityDiedEventFilterAttributesType),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct LuaEntityDiedEventFilter {
    /// The condition to filter on. One of `"ghost"`, `"rail"`, `"rail-signal"`, `"rolling-stock"`, `"robot-with-logistics-interface"`, `"vehicle"`, `"turret"`, `"crafting-machine"`, `"wall-connectable"`, `"transport-belt-connectable"`, `"circuit-network-connectable"`, `"type"`, `"name"`, `"ghost_type"`, `"ghost_name"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<LuaEntityDiedEventFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityMarkedForDeconstructionEventFilterAttributesGhostName {
    /// The ghost prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityMarkedForDeconstructionEventFilterAttributesGhostType {
    /// The ghost prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityMarkedForDeconstructionEventFilterAttributesName {
    /// The prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityMarkedForDeconstructionEventFilterAttributesType {
    /// The prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub enum LuaEntityMarkedForDeconstructionEventFilterAttributes {
    GhostName(LuaEntityMarkedForDeconstructionEventFilterAttributesGhostName),
    GhostType(LuaEntityMarkedForDeconstructionEventFilterAttributesGhostType),
    Name(LuaEntityMarkedForDeconstructionEventFilterAttributesName),
    Type(LuaEntityMarkedForDeconstructionEventFilterAttributesType),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct LuaEntityMarkedForDeconstructionEventFilter {
    /// The condition to filter on. One of `"ghost"`, `"rail"`, `"rail-signal"`, `"rolling-stock"`, `"robot-with-logistics-interface"`, `"vehicle"`, `"turret"`, `"crafting-machine"`, `"wall-connectable"`, `"transport-belt-connectable"`, `"circuit-network-connectable"`, `"type"`, `"name"`, `"ghost_type"`, `"ghost_name"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<LuaEntityMarkedForDeconstructionEventFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityMarkedForUpgradeEventFilterAttributesGhostName {
    /// The ghost prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityMarkedForUpgradeEventFilterAttributesGhostType {
    /// The ghost prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityMarkedForUpgradeEventFilterAttributesName {
    /// The prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaEntityMarkedForUpgradeEventFilterAttributesType {
    /// The prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub enum LuaEntityMarkedForUpgradeEventFilterAttributes {
    GhostName(LuaEntityMarkedForUpgradeEventFilterAttributesGhostName),
    GhostType(LuaEntityMarkedForUpgradeEventFilterAttributesGhostType),
    Name(LuaEntityMarkedForUpgradeEventFilterAttributesName),
    Type(LuaEntityMarkedForUpgradeEventFilterAttributesType),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct LuaEntityMarkedForUpgradeEventFilter {
    /// The condition to filter on. One of `"ghost"`, `"rail"`, `"rail-signal"`, `"rolling-stock"`, `"robot-with-logistics-interface"`, `"vehicle"`, `"turret"`, `"crafting-machine"`, `"wall-connectable"`, `"transport-belt-connectable"`, `"circuit-network-connectable"`, `"type"`, `"name"`, `"ghost_type"`, `"ghost_name"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<LuaEntityMarkedForUpgradeEventFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct LuaPlayerBuiltEntityEventFilterAttributesForce {
    /// The entity force
    pub force: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaPlayerBuiltEntityEventFilterAttributesGhostName {
    /// The ghost prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaPlayerBuiltEntityEventFilterAttributesGhostType {
    /// The ghost prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaPlayerBuiltEntityEventFilterAttributesName {
    /// The prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaPlayerBuiltEntityEventFilterAttributesType {
    /// The prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub enum LuaPlayerBuiltEntityEventFilterAttributes {
    Force(LuaPlayerBuiltEntityEventFilterAttributesForce),
    GhostName(LuaPlayerBuiltEntityEventFilterAttributesGhostName),
    GhostType(LuaPlayerBuiltEntityEventFilterAttributesGhostType),
    Name(LuaPlayerBuiltEntityEventFilterAttributesName),
    Type(LuaPlayerBuiltEntityEventFilterAttributesType),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct LuaPlayerBuiltEntityEventFilter {
    /// The condition to filter on. One of `"ghost"`, `"rail"`, `"rail-signal"`, `"rolling-stock"`, `"robot-with-logistics-interface"`, `"vehicle"`, `"turret"`, `"crafting-machine"`, `"wall-connectable"`, `"transport-belt-connectable"`, `"circuit-network-connectable"`, `"type"`, `"name"`, `"ghost_type"`, `"ghost_name"`, `"force"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<LuaPlayerBuiltEntityEventFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct LuaPlayerMinedEntityEventFilterAttributesGhostName {
    /// The ghost prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaPlayerMinedEntityEventFilterAttributesGhostType {
    /// The ghost prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaPlayerMinedEntityEventFilterAttributesName {
    /// The prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaPlayerMinedEntityEventFilterAttributesType {
    /// The prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub enum LuaPlayerMinedEntityEventFilterAttributes {
    GhostName(LuaPlayerMinedEntityEventFilterAttributesGhostName),
    GhostType(LuaPlayerMinedEntityEventFilterAttributesGhostType),
    Name(LuaPlayerMinedEntityEventFilterAttributesName),
    Type(LuaPlayerMinedEntityEventFilterAttributesType),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct LuaPlayerMinedEntityEventFilter {
    /// The condition to filter on. One of `"ghost"`, `"rail"`, `"rail-signal"`, `"rolling-stock"`, `"robot-with-logistics-interface"`, `"vehicle"`, `"turret"`, `"crafting-machine"`, `"wall-connectable"`, `"transport-belt-connectable"`, `"circuit-network-connectable"`, `"type"`, `"name"`, `"ghost_type"`, `"ghost_name"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<LuaPlayerMinedEntityEventFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct LuaPlayerRepairedEntityEventFilterAttributesGhostName {
    /// The ghost prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaPlayerRepairedEntityEventFilterAttributesGhostType {
    /// The ghost prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaPlayerRepairedEntityEventFilterAttributesName {
    /// The prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaPlayerRepairedEntityEventFilterAttributesType {
    /// The prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub enum LuaPlayerRepairedEntityEventFilterAttributes {
    GhostName(LuaPlayerRepairedEntityEventFilterAttributesGhostName),
    GhostType(LuaPlayerRepairedEntityEventFilterAttributesGhostType),
    Name(LuaPlayerRepairedEntityEventFilterAttributesName),
    Type(LuaPlayerRepairedEntityEventFilterAttributesType),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct LuaPlayerRepairedEntityEventFilter {
    /// The condition to filter on. One of `"ghost"`, `"rail"`, `"rail-signal"`, `"rolling-stock"`, `"robot-with-logistics-interface"`, `"vehicle"`, `"turret"`, `"crafting-machine"`, `"wall-connectable"`, `"transport-belt-connectable"`, `"circuit-network-connectable"`, `"type"`, `"name"`, `"ghost_type"`, `"ghost_name"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<LuaPlayerRepairedEntityEventFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct LuaPostEntityDiedEventFilterAttributesType {
    /// The prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub enum LuaPostEntityDiedEventFilterAttributes {
    Type(LuaPostEntityDiedEventFilterAttributesType),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct LuaPostEntityDiedEventFilter {
    /// The condition to filter on. Can only be `"type"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<LuaPostEntityDiedEventFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct LuaPreGhostDeconstructedEventFilterAttributesGhostName {
    /// The ghost prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaPreGhostDeconstructedEventFilterAttributesGhostType {
    /// The ghost prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaPreGhostDeconstructedEventFilterAttributesName {
    /// The prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaPreGhostDeconstructedEventFilterAttributesType {
    /// The prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub enum LuaPreGhostDeconstructedEventFilterAttributes {
    GhostName(LuaPreGhostDeconstructedEventFilterAttributesGhostName),
    GhostType(LuaPreGhostDeconstructedEventFilterAttributesGhostType),
    Name(LuaPreGhostDeconstructedEventFilterAttributesName),
    Type(LuaPreGhostDeconstructedEventFilterAttributesType),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct LuaPreGhostDeconstructedEventFilter {
    /// The condition to filter on. One of `"ghost"`, `"rail"`, `"rail-signal"`, `"rolling-stock"`, `"robot-with-logistics-interface"`, `"vehicle"`, `"turret"`, `"crafting-machine"`, `"wall-connectable"`, `"transport-belt-connectable"`, `"circuit-network-connectable"`, `"type"`, `"name"`, `"ghost_type"`, `"ghost_name"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<LuaPreGhostDeconstructedEventFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct LuaPreGhostUpgradedEventFilterAttributesGhostName {
    /// The ghost prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaPreGhostUpgradedEventFilterAttributesGhostType {
    /// The ghost prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaPreGhostUpgradedEventFilterAttributesName {
    /// The prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaPreGhostUpgradedEventFilterAttributesType {
    /// The prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub enum LuaPreGhostUpgradedEventFilterAttributes {
    GhostName(LuaPreGhostUpgradedEventFilterAttributesGhostName),
    GhostType(LuaPreGhostUpgradedEventFilterAttributesGhostType),
    Name(LuaPreGhostUpgradedEventFilterAttributesName),
    Type(LuaPreGhostUpgradedEventFilterAttributesType),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct LuaPreGhostUpgradedEventFilter {
    /// The condition to filter on. One of `"ghost"`, `"rail"`, `"rail-signal"`, `"rolling-stock"`, `"robot-with-logistics-interface"`, `"vehicle"`, `"turret"`, `"crafting-machine"`, `"wall-connectable"`, `"transport-belt-connectable"`, `"circuit-network-connectable"`, `"type"`, `"name"`, `"ghost_type"`, `"ghost_name"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<LuaPreGhostUpgradedEventFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct LuaPrePlayerMinedEntityEventFilterAttributesGhostName {
    /// The ghost prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaPrePlayerMinedEntityEventFilterAttributesGhostType {
    /// The ghost prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaPrePlayerMinedEntityEventFilterAttributesName {
    /// The prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaPrePlayerMinedEntityEventFilterAttributesType {
    /// The prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub enum LuaPrePlayerMinedEntityEventFilterAttributes {
    GhostName(LuaPrePlayerMinedEntityEventFilterAttributesGhostName),
    GhostType(LuaPrePlayerMinedEntityEventFilterAttributesGhostType),
    Name(LuaPrePlayerMinedEntityEventFilterAttributesName),
    Type(LuaPrePlayerMinedEntityEventFilterAttributesType),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct LuaPrePlayerMinedEntityEventFilter {
    /// The condition to filter on. One of `"ghost"`, `"rail"`, `"rail-signal"`, `"rolling-stock"`, `"robot-with-logistics-interface"`, `"vehicle"`, `"turret"`, `"crafting-machine"`, `"wall-connectable"`, `"transport-belt-connectable"`, `"circuit-network-connectable"`, `"type"`, `"name"`, `"ghost_type"`, `"ghost_name"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<LuaPrePlayerMinedEntityEventFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct LuaPreRobotMinedEntityEventFilterAttributesGhostName {
    /// The ghost prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaPreRobotMinedEntityEventFilterAttributesGhostType {
    /// The ghost prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaPreRobotMinedEntityEventFilterAttributesName {
    /// The prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaPreRobotMinedEntityEventFilterAttributesType {
    /// The prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub enum LuaPreRobotMinedEntityEventFilterAttributes {
    GhostName(LuaPreRobotMinedEntityEventFilterAttributesGhostName),
    GhostType(LuaPreRobotMinedEntityEventFilterAttributesGhostType),
    Name(LuaPreRobotMinedEntityEventFilterAttributesName),
    Type(LuaPreRobotMinedEntityEventFilterAttributesType),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct LuaPreRobotMinedEntityEventFilter {
    /// The condition to filter on. One of `"ghost"`, `"rail"`, `"rail-signal"`, `"rolling-stock"`, `"robot-with-logistics-interface"`, `"vehicle"`, `"turret"`, `"crafting-machine"`, `"wall-connectable"`, `"transport-belt-connectable"`, `"circuit-network-connectable"`, `"type"`, `"name"`, `"ghost_type"`, `"ghost_name"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<LuaPreRobotMinedEntityEventFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct LuaRobotBuiltEntityEventFilterAttributesForce {
    /// The entity force
    pub force: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaRobotBuiltEntityEventFilterAttributesGhostName {
    /// The ghost prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaRobotBuiltEntityEventFilterAttributesGhostType {
    /// The ghost prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaRobotBuiltEntityEventFilterAttributesName {
    /// The prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaRobotBuiltEntityEventFilterAttributesType {
    /// The prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub enum LuaRobotBuiltEntityEventFilterAttributes {
    Force(LuaRobotBuiltEntityEventFilterAttributesForce),
    GhostName(LuaRobotBuiltEntityEventFilterAttributesGhostName),
    GhostType(LuaRobotBuiltEntityEventFilterAttributesGhostType),
    Name(LuaRobotBuiltEntityEventFilterAttributesName),
    Type(LuaRobotBuiltEntityEventFilterAttributesType),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct LuaRobotBuiltEntityEventFilter {
    /// The condition to filter on. One of `"ghost"`, `"rail"`, `"rail-signal"`, `"rolling-stock"`, `"robot-with-logistics-interface"`, `"vehicle"`, `"turret"`, `"crafting-machine"`, `"wall-connectable"`, `"transport-belt-connectable"`, `"circuit-network-connectable"`, `"type"`, `"name"`, `"ghost_type"`, `"ghost_name"`, `"force"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<LuaRobotBuiltEntityEventFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct LuaRobotMinedEntityEventFilterAttributesGhostName {
    /// The ghost prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaRobotMinedEntityEventFilterAttributesGhostType {
    /// The ghost prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaRobotMinedEntityEventFilterAttributesName {
    /// The prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaRobotMinedEntityEventFilterAttributesType {
    /// The prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub enum LuaRobotMinedEntityEventFilterAttributes {
    GhostName(LuaRobotMinedEntityEventFilterAttributesGhostName),
    GhostType(LuaRobotMinedEntityEventFilterAttributesGhostType),
    Name(LuaRobotMinedEntityEventFilterAttributesName),
    Type(LuaRobotMinedEntityEventFilterAttributesType),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct LuaRobotMinedEntityEventFilter {
    /// The condition to filter on. One of `"ghost"`, `"rail"`, `"rail-signal"`, `"rolling-stock"`, `"robot-with-logistics-interface"`, `"vehicle"`, `"turret"`, `"crafting-machine"`, `"wall-connectable"`, `"transport-belt-connectable"`, `"circuit-network-connectable"`, `"type"`, `"name"`, `"ghost_type"`, `"ghost_name"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<LuaRobotMinedEntityEventFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct LuaScriptRaisedBuiltEventFilterAttributesGhostName {
    /// The ghost prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaScriptRaisedBuiltEventFilterAttributesGhostType {
    /// The ghost prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaScriptRaisedBuiltEventFilterAttributesName {
    /// The prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaScriptRaisedBuiltEventFilterAttributesType {
    /// The prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub enum LuaScriptRaisedBuiltEventFilterAttributes {
    GhostName(LuaScriptRaisedBuiltEventFilterAttributesGhostName),
    GhostType(LuaScriptRaisedBuiltEventFilterAttributesGhostType),
    Name(LuaScriptRaisedBuiltEventFilterAttributesName),
    Type(LuaScriptRaisedBuiltEventFilterAttributesType),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct LuaScriptRaisedBuiltEventFilter {
    /// The condition to filter on. One of `"ghost"`, `"rail"`, `"rail-signal"`, `"rolling-stock"`, `"robot-with-logistics-interface"`, `"vehicle"`, `"turret"`, `"crafting-machine"`, `"wall-connectable"`, `"transport-belt-connectable"`, `"circuit-network-connectable"`, `"type"`, `"name"`, `"ghost_type"`, `"ghost_name"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<LuaScriptRaisedBuiltEventFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct LuaScriptRaisedDestroyEventFilterAttributesGhostName {
    /// The ghost prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaScriptRaisedDestroyEventFilterAttributesGhostType {
    /// The ghost prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaScriptRaisedDestroyEventFilterAttributesName {
    /// The prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaScriptRaisedDestroyEventFilterAttributesType {
    /// The prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub enum LuaScriptRaisedDestroyEventFilterAttributes {
    GhostName(LuaScriptRaisedDestroyEventFilterAttributesGhostName),
    GhostType(LuaScriptRaisedDestroyEventFilterAttributesGhostType),
    Name(LuaScriptRaisedDestroyEventFilterAttributesName),
    Type(LuaScriptRaisedDestroyEventFilterAttributesType),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct LuaScriptRaisedDestroyEventFilter {
    /// The condition to filter on. One of `"ghost"`, `"rail"`, `"rail-signal"`, `"rolling-stock"`, `"robot-with-logistics-interface"`, `"vehicle"`, `"turret"`, `"crafting-machine"`, `"wall-connectable"`, `"transport-belt-connectable"`, `"circuit-network-connectable"`, `"type"`, `"name"`, `"ghost_type"`, `"ghost_name"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<LuaScriptRaisedDestroyEventFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct LuaScriptRaisedReviveEventFilterAttributesGhostName {
    /// The ghost prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaScriptRaisedReviveEventFilterAttributesGhostType {
    /// The ghost prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaScriptRaisedReviveEventFilterAttributesName {
    /// The prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaScriptRaisedReviveEventFilterAttributesType {
    /// The prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub enum LuaScriptRaisedReviveEventFilterAttributes {
    GhostName(LuaScriptRaisedReviveEventFilterAttributesGhostName),
    GhostType(LuaScriptRaisedReviveEventFilterAttributesGhostType),
    Name(LuaScriptRaisedReviveEventFilterAttributesName),
    Type(LuaScriptRaisedReviveEventFilterAttributesType),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct LuaScriptRaisedReviveEventFilter {
    /// The condition to filter on. One of `"ghost"`, `"rail"`, `"rail-signal"`, `"rolling-stock"`, `"robot-with-logistics-interface"`, `"vehicle"`, `"turret"`, `"crafting-machine"`, `"wall-connectable"`, `"transport-belt-connectable"`, `"circuit-network-connectable"`, `"type"`, `"name"`, `"ghost_type"`, `"ghost_name"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<LuaScriptRaisedReviveEventFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct LuaScriptRaisedTeleportedEventFilterAttributesGhostName {
    /// The ghost prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaScriptRaisedTeleportedEventFilterAttributesGhostType {
    /// The ghost prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaScriptRaisedTeleportedEventFilterAttributesName {
    /// The prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaScriptRaisedTeleportedEventFilterAttributesType {
    /// The prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub enum LuaScriptRaisedTeleportedEventFilterAttributes {
    GhostName(LuaScriptRaisedTeleportedEventFilterAttributesGhostName),
    GhostType(LuaScriptRaisedTeleportedEventFilterAttributesGhostType),
    Name(LuaScriptRaisedTeleportedEventFilterAttributesName),
    Type(LuaScriptRaisedTeleportedEventFilterAttributesType),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct LuaScriptRaisedTeleportedEventFilter {
    /// The condition to filter on. One of `"ghost"`, `"rail"`, `"rail-signal"`, `"rolling-stock"`, `"robot-with-logistics-interface"`, `"vehicle"`, `"turret"`, `"crafting-machine"`, `"wall-connectable"`, `"transport-belt-connectable"`, `"circuit-network-connectable"`, `"type"`, `"name"`, `"ghost_type"`, `"ghost_name"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<LuaScriptRaisedTeleportedEventFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct LuaSectorScannedEventFilterAttributesGhostName {
    /// The ghost prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaSectorScannedEventFilterAttributesGhostType {
    /// The ghost prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaSectorScannedEventFilterAttributesName {
    /// The prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaSectorScannedEventFilterAttributesType {
    /// The prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub enum LuaSectorScannedEventFilterAttributes {
    GhostName(LuaSectorScannedEventFilterAttributesGhostName),
    GhostType(LuaSectorScannedEventFilterAttributesGhostType),
    Name(LuaSectorScannedEventFilterAttributesName),
    Type(LuaSectorScannedEventFilterAttributesType),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct LuaSectorScannedEventFilter {
    /// The condition to filter on. One of `"ghost"`, `"rail"`, `"rail-signal"`, `"rolling-stock"`, `"robot-with-logistics-interface"`, `"vehicle"`, `"turret"`, `"crafting-machine"`, `"wall-connectable"`, `"transport-belt-connectable"`, `"circuit-network-connectable"`, `"type"`, `"name"`, `"ghost_type"`, `"ghost_name"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<LuaSectorScannedEventFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct LuaUpgradeCancelledEventFilterAttributesGhostName {
    /// The ghost prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaUpgradeCancelledEventFilterAttributesGhostType {
    /// The ghost prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaUpgradeCancelledEventFilterAttributesName {
    /// The prototype name
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LuaUpgradeCancelledEventFilterAttributesType {
    /// The prototype type
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub enum LuaUpgradeCancelledEventFilterAttributes {
    GhostName(LuaUpgradeCancelledEventFilterAttributesGhostName),
    GhostType(LuaUpgradeCancelledEventFilterAttributesGhostType),
    Name(LuaUpgradeCancelledEventFilterAttributesName),
    Type(LuaUpgradeCancelledEventFilterAttributesType),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct LuaUpgradeCancelledEventFilter {
    /// The condition to filter on. One of `"ghost"`, `"rail"`, `"rail-signal"`, `"rolling-stock"`, `"robot-with-logistics-interface"`, `"vehicle"`, `"turret"`, `"crafting-machine"`, `"wall-connectable"`, `"transport-belt-connectable"`, `"circuit-network-connectable"`, `"type"`, `"name"`, `"ghost_type"`, `"ghost_name"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<LuaUpgradeCancelledEventFilterAttributes>,
}

#[derive(Debug, Deserialize)]
/// All regular [MapSettings](MapSettings) plus an additional table that contains the [DifficultySettings](DifficultySettings).
pub struct MapAndDifficultySettings {
    pub difficulty_settings: DifficultySettings,
    pub enemy_evolution: EnemyEvolutionMapSettings,
    pub enemy_expansion: EnemyExpansionMapSettings,
    /// If a behavior fails this many times, the enemy (or enemy group) is destroyed. This solves biters getting stuck within their own base.
    pub max_failed_behavior_count: u32,
    pub path_finder: PathFinderMapSettings,
    pub pollution: PollutionMapSettings,
    pub steering: SteeringMapSettings,
    pub unit_group: UnitGroupMapSettings,
}

#[derive(Debug, Deserialize)]
/// The data that can be extracted from a map exchange string, as a plain table.
pub struct MapExchangeStringData {
    pub map_gen_settings: MapGenSettings,
    pub map_settings: MapAndDifficultySettings,
}

#[derive(Debug, Deserialize)]
pub struct MapGenPreset {
    pub advanced_settings: Option<AdvancedMapGenSettings>,
    pub basic_settings: Option<MapGenSettings>,
    /// Whether this is the preset that is selected by default.
    pub default: Option<bool>,
    /// The string used to alphabetically sort the presets. It is a simple string that has no additional semantic meaning.
    pub order: String,
}

#[derive(Debug, Deserialize)]
/// The 'map type' dropdown in the map generation GUI is actually a selector for elevation generator. The base game sets `property_expression_names.elevation` to `"0_16-elevation"` to reproduce terrain from 0.16 or to `"0_17-island"` for the island preset. If generators are available for other properties, the 'map type' dropdown in the GUI will be renamed to 'elevation' and shown along with selectors for the other selectable properties.
///
/// # Examples
///
/// * Assuming a NamedNoiseExpression with the name "my-alternate-grass1-probability" is defined
/// ```text
/// local surface = game.player.surface
/// local mgs = surface.map_gen_settings
/// mgs.property_expression_names["tile:grass1:probability"] = "my-alternate-grass1-probability"
/// surface.map_gen_settings = mgs
/// ```
///  would override the probability of grass1 being placed at any given point on the current surface.
/// * To make there be no deep water on (newly generated chunks) a surface:
/// ```text
/// local surface = game.player.surface
/// local mgs = surface.map_gen_settings
/// mgs.property_expression_names["tile:deepwater:probability"] = -1000
/// surface.map_gen_settings = mgs
/// ```
///  This does not require a NamedNoiseExpression to be defined, since literal numbers (and strings naming literal numbers, e.g. `"123"`) are understood to stand for constant value expressions.
pub struct MapGenSettings {
    /// Indexed by autoplace control prototype name.
    pub autoplace_controls: HashMap<String, AutoplaceControl>,
    /// Each setting in this dictionary maps the string type to the settings for that type. Valid types are `"entity"`, `"tile"` and `"decorative"`.
    pub autoplace_settings: HashMap<String, AutoplaceSettings>,
    /// Map generation settings for entities of the type "cliff".
    pub cliff_settings: CliffPlacementSettings,
    /// Whether undefined `autoplace_controls` should fall back to the default controls or not. Defaults to `true`.
    pub default_enable_all_autoplace_controls: Option<bool>,
    /// Height in tiles. If `0`, the map has 'infinite' height, with the actual limitation being one million tiles in each direction from the center.
    pub height: u32,
    /// Whether peaceful mode is enabled for this map.
    pub peaceful_mode: bool,
    /// Overrides for tile property value generators. Values either name a NamedNoiseExpression or can be literal numbers, stored as strings (e.g. `"5"`). All other controls can be overridden by a property expression names. Notable properties:
    /// - `moisture` - a value between 0 and 1 that determines whether a tile becomes sandy (low moisture) or grassy (high moisture).
    /// - `aux` - a value between 0 and 1 that determines whether low-moisture tiles become sand or red desert.
    /// - `temperature` - provides a value (vaguely representing degrees Celsius, varying between -20 and 50) that is used (together with moisture and aux) as part of tree and decorative placement.
    /// - `elevation` - tiles values less than zero become water. Cliffs are placed along certain contours according to [CliffPlacementSettings](CliffPlacementSettings).
    /// - `cliffiness` - determines whether (when >0.5) or not (when <0.5) a cliff will be placed at an otherwise suitable (according to [CliffPlacementSettings](CliffPlacementSettings)) location.
    /// - `enemy-base-intensity` - a number that is referenced by both `enemy-base-frequency` and `enemy-base-radius`. i.e. if this is overridden, enemy base frequency and size will both be affected and do something reasonable. By default, this expression returns a value proportional to distance from any starting point, clamped at about 7.
    /// - `enemy-base-frequency` - a number representing average number of enemy bases per tile for a region, by default in terms of `enemy-base-intensity`.
    /// - `enemy-base-radius` - a number representing the radius of an enemy base, if one were to be placed on the given tile, by default proportional to a constant plus `enemy-base-intensity`. Climate controls ('Moisture' and 'Terrain type' at the bottom of the Terrain tab in the map generator GUI) don't have their own dedicated structures in MapGenSettings. Instead, their values are stored as property expression overrides with long names:
    /// - `control-setting:moisture:frequency:multiplier` - frequency (inverse of scale) multiplier for moisture noise. Default is 1.
    /// - `control-setting:moisture:bias` - global bias for moisture (which normally varies between 0 and 1). Default is 0.
    /// - `control-setting:aux:frequency:multiplier` - frequency (inverse of scale) multiplier for aux (called 'terrain type' in the GUI) noise. Default is 1.
    /// - `control-setting:aux:bias` - global bias for aux/terrain type (which normally varies between 0 and 1). Default is 0. All other MapGenSettings feed into named noise expressions, and therefore placement can be overridden by including the name of a property in this dictionary. The probability and richness functions for placing specific tiles, entities, and decoratives can be overridden by including an entry named `{tile|entity|decorative}:(prototype name):{probability|richness}`.
    pub property_expression_names: HashMap<String, String>,
    /// The random seed used to generated this map.
    pub seed: u32,
    /// Size of the starting area.
    pub starting_area: MapGenSize,
    /// Positions of the starting areas.
    pub starting_points: Vec<MapPosition>,
    /// The inverse of 'water scale' in the map generator GUI. Lower `terrain_segmentation` increases the scale of elevation features (lakes, continents, etc). This behavior can be overridden with alternate elevation generators (see `property_expression_names`, below).
    pub terrain_segmentation: MapGenSize,
    /// The equivalent to 'water coverage' in the map generator GUI. Specifically, when this value is non-zero, `water_level = 10 * log2` (the value of this field), and the elevation generator subtracts water level from elevation before adding starting lakes. If water is set to 'none', elevation is clamped to a small positive value before adding starting lakes. This behavior can be overridden with alternate elevation generators (see `property_expression_names`, below).
    pub water: MapGenSize,
    /// Width in tiles. If `0`, the map has 'infinite' width, with the actual limitation being one million tiles in each direction from the center.
    pub width: u32,
}

/// A floating point number specifying an amount.
///
/// For backwards compatibility, MapGenSizes can also be specified as one of the following strings, which will be converted to a number (when queried, a number will always be returned):
///
/// # Notes
///
/// * The map generation algorithm officially supports the range of values the in-game map generation screen shows (specifically `0` and values from `1/6` to `6`). Values outside this range are not guaranteed to work as expected.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MapGenSize {
    /// Specifying a map gen dimension.
    Float(Float),
    /// equivalent to `0`.
    None,
    /// equivalent to `1/2`.
    VeryLow,
    /// equivalent to `1/2`.
    VerySmall,
    /// equivalent to `1/2`.
    VeryPoor,
    /// equivalent to `1/sqrt(2)`.
    Low,
    /// equivalent to `1/sqrt(2)`.
    Small,
    /// equivalent to `1/sqrt(2)`.
    Poor,
    /// equivalent to `1`.
    Normal,
    /// equivalent to `1`.
    Medium,
    /// equivalent to `1`.
    Regular,
    /// equivalent to `sqrt(2)`.
    High,
    /// equivalent to `sqrt(2)`.
    Big,
    /// equivalent to `sqrt(2)`.
    Good,
    /// equivalent to `2`.
    VeryHigh,
    /// equivalent to `2`.
    VeryBig,
    /// equivalent to `2`.
    VeryGood,
}

// ================================
// ========= MANUAL PATCH =========

#[derive(Debug, Deserialize)]
pub struct MapPositionTable {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Deserialize)]
/// Coordinates on a surface, for example of an entity. MapPositions may be specified either as a dictionary with `x`, `y` as keys, or simply as an array with two elements.
///
/// The coordinates are saved as a fixed-size 32 bit integer, with 8 bits reserved for decimal precision, meaning the smallest value step is `1/2^8 = 0.00390625` tiles.
///
/// # Examples
///
/// * Explicit definition:
/// ```text
/// {x = 5.5, y = 2}
/// {y = 2.25, x = 5.125}
/// ```
/// * Shorthand:
/// ```text
/// {1.625, 2.375}
/// ```
pub struct MapPositionTuple {
    pub position: MapPositionTable,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MapPosition {
    Table(MapPositionTable),
    Tuple(MapPositionTuple),
}

// TODO: macro for table or tuple?

// ========= MANUAL PATCH =========
// ================================

#[derive(Debug, Deserialize)]
/// Various game-related settings. Updating any of the attributes will immediately take effect in the game engine.
///
/// # Examples
///
/// * Increase the number of short paths the pathfinder can cache.
/// ```text
/// game.map_settings.path_finder.short_cache_size = 15
/// ```
pub struct MapSettings {
    pub enemy_evolution: EnemyEvolutionMapSettings,
    pub enemy_expansion: EnemyExpansionMapSettings,
    /// If a behavior fails this many times, the enemy (or enemy group) is destroyed. This solves biters getting stuck within their own base.
    pub max_failed_behavior_count: u32,
    pub path_finder: PathFinderMapSettings,
    pub pollution: PollutionMapSettings,
    pub steering: SteeringMapSettings,
    pub unit_group: UnitGroupMapSettings,
}

#[derive(Debug, Deserialize)]
/// What is shown in the map view. If a field is not given, that setting will not be changed.
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

#[derive(Debug, Deserialize)]
pub struct ModChangeData {
    /// New version of the mod. May be `nil` if the mod is no longer present (i.e. it was just removed).
    pub new_version: String,
    /// Old version of the mod. May be `nil` if the mod wasn't previously present (i.e. it was just added).
    pub old_version: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ModSettingValueUnion {
    Int(i32),
    Double(Double),
    Boolean(bool),
    String(String),
    Color(Color),
}

#[derive(Debug, Deserialize)]
pub struct ModSetting {
    /// The value of the mod setting. The type depends on the kind of setting.
    pub value: ModSettingValueUnion,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ModSettingPrototypeFilterAttributesTypeUnion {
    String(String),
    Array(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct ModSettingPrototypeFilterAttributesMod {
    /// The mod name
    #[serde(rename = "mod")]
    pub mod_name: String,
}

#[derive(Debug, Deserialize)]
pub struct ModSettingPrototypeFilterAttributesSettingType {
    /// The setting scope type (`"startup"`, `"runtime-global"`, or `"runtime-per-user"`)
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct ModSettingPrototypeFilterAttributesType {
    /// The prototype type, or a list of acceptable types.
    #[serde(rename = "type")]
    pub typ: ModSettingPrototypeFilterAttributesTypeUnion,
}

#[derive(Debug, Deserialize)]
pub enum ModSettingPrototypeFilterAttributes {
    Mod(ModSettingPrototypeFilterAttributesMod),
    SettingType(ModSettingPrototypeFilterAttributesSettingType),
    Type(ModSettingPrototypeFilterAttributesType),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct ModSettingPrototypeFilter {
    /// The condition to filter on. One of `"type"`, `"mod"`, `"setting-type"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<ModSettingPrototypeFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct ModuleEffectValue {
    /// The percentual increase of the attribute. A value of `0.6` means a 60% increase.
    pub bonus: Float,
}

#[derive(Debug, Deserialize)]
/// # Examples
///
/// * These are the effects of the vanilla Productivity Module 3 (up to floating point imprecisions):
/// ```text
/// {consumption={bonus=0.6},
///  speed={bonus=-0.15},
///  productivity={bonus=0.06},
///  pollution={bonus=0.075}}
/// ```
pub struct ModuleEffects {
    pub consumption: Option<ModuleEffectValue>,
    pub pollution: Option<ModuleEffectValue>,
    pub productivity: Option<ModuleEffectValue>,
    pub speed: Option<ModuleEffectValue>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "kebab-case")]
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

/// A set of flags. Active flags are in the dictionary as `true`, while inactive flags aren't present at all.
///
/// To write to this, use an array[[string](string)] of the mouse buttons that should be possible to use with on button. The flag `"left-and-right"` can also be set, which will set `"left"` and `"right"` to `true`.
pub type MouseButtonFlags = HashMap<MouseButtonFlagsUnion, bool>;

#[derive(Debug, Deserialize)]
/// A fragment of a functional program used to generate coherent noise, probably for purposes related to terrain generation. These can only be meaningfully written/modified during the data load phase. More detailed information is found on the [wiki](https://wiki.factorio.com/Types/NoiseExpression).
pub struct NoiseExpression {
    /// Names the type of the expression and determines what other fields are required.
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct NthTickEventData {
    /// The nth tick this handler was registered to.
    pub nth_tick: u32,
    /// The tick during which the event happened.
    pub tick: u32,
}

#[derive(Debug, Deserialize)]
/// A single offer on a market entity.
pub struct Offer {
    /// The action that will take place when a player accepts the offer. Usually a `"give-item"` modifier.
    pub offer: TechnologyModifier,
    /// List of prices.
    pub price: Vec<Ingredient>,
}

#[derive(Debug, Deserialize)]
pub struct OldTileAndPosition {
    pub old_tile: MaybeCycle<LuaTilePrototype>,
    pub position: TilePosition,
}

#[derive(Debug, Deserialize)]
pub struct PathFinderMapSettings {
    /// When looking for a path from cache, make sure it doesn't end too far from the requested end in relative terms. This is typically more lenient than the start ratio since the end target could be moving. Defaults to `0.15`.
    pub cache_accept_path_end_distance_ratio: Double,
    /// When looking for a path from cache, make sure it doesn't start too far from the requested start in relative terms. Defaults to `0.2`.
    pub cache_accept_path_start_distance_ratio: Double,
    /// When looking for a connection to a cached path, search at most for this number of steps times the original estimate. Defaults to `100`.
    pub cache_max_connect_to_cache_steps_multiplier: u32,
    /// When assigning a rating to the best path, this multiplier times end distances is considered. This value is typically higher than the start multiplier as this results in better end path quality. Defaults to `20`.
    pub cache_path_end_distance_rating_multiplier: Double,
    /// When assigning a rating to the best path, this multiplier times start distances is considered. Defaults to `10`.
    pub cache_path_start_distance_rating_multiplier: Double,
    /// The maximum direct distance in tiles before a request is no longer considered short. Defaults to `100`.
    pub direct_distance_to_consider_short_request: u32,
    /// A penalty that is applied for another unit that is too close and either not moving or has a different goal. Defaults to `30`.
    pub enemy_with_different_destination_collision_penalty: Double,
    /// The collision penalty for collisions in the extended bounding box but outside the entity's actual bounding box. Defaults to `3`.
    pub extended_collision_penalty: Double,
    /// The pathfinder performs a step of the backward search every `fwd2bwd_ratio`'th step. The minimum allowed value is `2`, which means symmetric search. The default value is `5`.
    pub fwd2bwd_ratio: u32,
    /// The general collision penalty with other units. Defaults to `10`.
    pub general_entity_collision_penalty: Double,
    /// The collision penalty for positions that require the destruction of an entity to get to. Defaults to `3`.
    pub general_entity_subsequent_collision_penalty: Double,
    /// When looking at which node to check next, their heuristic value is multiplied by this ratio. The higher it is, the more the search is directed straight at the goal. Defaults to `2`.
    pub goal_pressure_ratio: Double,
    /// The distance in tiles after which other moving units are not considered for pathfinding. Defaults to `5`.
    pub ignore_moving_enemy_collision_distance: Double,
    /// The minimal distance to the goal in tiles required to be searched in the long path cache. Defaults to `30`.
    pub long_cache_min_cacheable_distance: Double,
    /// Number of elements in the long cache. Defaults to `25`.
    pub long_cache_size: u32,
    /// The amount of path finder requests accepted per tick regardless of the requested path's length. Defaults to `10`.
    pub max_clients_to_accept_any_new_request: u32,
    /// When the `max_clients_to_accept_any_new_request` amount is exhausted, only path finder requests with a short estimate will be accepted until this amount (per tick) is reached. Defaults to `100`.
    pub max_clients_to_accept_short_new_request: u32,
    /// The maximum number of nodes that are expanded per tick. Defaults to `1,000`.
    pub max_steps_worked_per_tick: Double,
    /// The maximum amount of work each pathfinding job is allowed to do per tick. Defaults to `8,000`.
    pub max_work_done_per_tick: u32,
    /// The minimum amount of steps that are guaranteed to be performed for every request. Defaults to `2000`.
    pub min_steps_to_check_path_find_termination: u32,
    /// Same principle as `cache_accept_path_end_distance_ratio`, but used for negative cache queries. Defaults to `0.3`.
    pub negative_cache_accept_path_end_distance_ratio: Double,
    /// Same principle as `cache_accept_path_start_distance_ratio`, but used for negative cache queries. Defaults to `0.3`.
    pub negative_cache_accept_path_start_distance_ratio: Double,
    /// The delay in ticks between decrementing the score of all paths in the negative cache by one. Defaults to `20`.
    pub negative_path_cache_delay_interval: u32,
    /// The thresholds of waiting clients after each of which the per-tick work limit will be increased by the corresponding value in `overload_multipliers`. This is to avoid clients having to wait too long. Must have the same number of elements as `overload_multipliers`. Defaults to `{0, 100, 500}`.
    pub overload_levels: Vec<u32>,
    /// The multipliers to the amount of per-tick work applied after the corresponding thresholds in `overload_levels` have been reached. Must have the same number of elements as `overload_multipliers`. Defaults to `{2, 3, 4}`.
    pub overload_multipliers: Vec<Double>,
    /// The minimal number of nodes required to be searched in the short path cache. Defaults to `50`.
    pub short_cache_min_algo_steps_to_cache: u32,
    /// The minimal distance to the goal in tiles required to be searched in the short path cache. Defaults to `10`.
    pub short_cache_min_cacheable_distance: Double,
    /// Number of elements in the short cache. Defaults to `5`.
    pub short_cache_size: u32,
    /// The maximum amount of nodes a short request will traverse before being rescheduled as a long request. Defaults to `1000`.
    pub short_request_max_steps: u32,
    /// The amount of steps that are allocated to short requests each tick, as a percentage of all available steps. Defaults to `0.5`, or 50%.
    pub short_request_ratio: Double,
    /// A penalty that is applied for another unit that is on the way to the goal. This is mainly relevant for situations where a group of units has arrived at the target they are supposed to attack, making units further back circle around to reach the target. Defaults to `30`.
    pub stale_enemy_with_same_destination_collision_penalty: Double,
    /// If the actual amount of steps is higher than the initial estimate by this factor, pathfinding is terminated. Defaults to `2000.0`.
    pub start_to_goal_cost_multiplier_to_terminate_path_find: Double,
    /// Whether to cache paths at all. Defaults to `true`.
    pub use_path_cache: bool,
}

#[derive(Debug, Deserialize)]
pub struct PathfinderFlags {
    /// Allows pathing through friendly entities. Defaults to `false`.
    pub allow_destroy_friendly_entities: Option<bool>,
    /// Allows the pathfinder to path through entities of the same force. Defaults to `false`.
    pub allow_paths_through_own_entities: Option<bool>,
    /// Enables path caching. This can be more efficient, but might fail to respond to changes in the environment. Defaults to `true`.
    pub cache: Option<bool>,
    /// Sets lower priority on the path request, meaning it might take longer to find a path at the expense of speeding up others. Defaults to `false`.
    pub low_priority: Option<bool>,
    /// Makes the pathfinder not break in the middle of processing this pathfind, no matter how much work is needed. Defaults to `false`.
    pub no_break: Option<bool>,
    /// Makes the pathfinder try to path in straight lines. Defaults to `false`.
    pub prefer_straight_paths: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct PathfinderWaypoint {
    /// `true` if the path from the previous waypoint to this one goes through an entity that must be destroyed.
    pub needs_destroy_to_reach: bool,
    /// The position of the waypoint on its surface.
    pub position: MapPosition,
}

#[derive(Debug, Deserialize)]
pub struct PlaceAsTileResult {
    pub condition: CollisionMask,
    pub condition_size: u32,
    /// The tile prototype.
    pub result: MaybeCycle<LuaTilePrototype>,
}

/// A player may be specified in one of three ways.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PlayerIdentification {
    /// The player index.
    Uint(u32),
    /// The player name.
    String(String),
    /// A reference to [LuaPlayer](LuaPlayer) may be passed directly.
    LuaPlayer(MaybeCycle<LuaPlayer>),
}

#[derive(Debug, Deserialize)]
/// These values are for the time frame of one second (60 ticks).
pub struct PollutionMapSettings {
    /// The amount of pollution eaten by a chunk's tiles as a percentage of 1. Defaults to `1`.
    pub ageing: Double,
    /// The amount that is diffused to a neighboring chunk (possibly repeated for other directions as well). Defaults to `0.02`.
    pub diffusion_ratio: Double,
    /// Whether pollution is enabled at all.
    pub enabled: bool,
    /// Defaults to `1`.
    pub enemy_attack_pollution_consumption_modifier: Double,
    /// Any amount of pollution larger than this value is visualized as this value instead. Defaults to `150`.
    pub expected_max_per_chunk: Double,
    /// Defaults to `20`.
    pub max_pollution_to_restore_trees: Double,
    /// Defaults to `60`.
    pub min_pollution_to_damage_trees: Double,
    /// The amount of PUs that need to be in a chunk for it to start diffusing. Defaults to `15`.
    pub min_to_diffuse: Double,
    /// Any amount of pollution smaller than this value (but bigger than zero) is visualized as this value instead. Defaults to `50`.
    pub min_to_show_per_chunk: Double,
    /// Defaults to `50`.
    pub pollution_per_tree_damage: Double,
    /// Defaults to `10`.
    pub pollution_restored_per_tree_damage: Double,
    /// Defaults to `150`.
    pub pollution_with_max_forest_damage: Double,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ProductAmountMaxUnion {
    Uint(u32),
    Double(Double),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ProductAmountMinUnion {
    Uint(u32),
    Double(Double),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ProductCatalystAmountUnion {
    Uint(u32),
    Double(Double),
}

#[derive(Debug, Deserialize)]
pub struct ProductAttributesFluid {
    /// The fluid temperature of this product.
    pub temperature: Option<Double>,
}

#[derive(Debug, Deserialize)]
pub enum ProductAttributes {
    Fluid(ProductAttributesFluid),
}

#[derive(Debug, Deserialize)]
/// # Examples
///
/// * Products of the "steel-chest" recipe (an array of Product):
/// ```text
/// {{type="item", name="steel-chest", amount=1}}
/// ```
/// * Products of the "advanced-oil-processing" recipe:
/// ```text
/// {{type="fluid", name="heavy-oil", amount=1},
///  {type="fluid", name="light-oil", amount=4.5},
///  {type="fluid", name="petroleum-gas", amount=5.5}}
/// ```
/// * What a custom recipe would look like that had a probability of 0.5 to return a minimum amount of 1 and a maximum amount of 5:
/// ```text
/// {{type="item", name="custom-item", probability=0.5, amount_min=1, amount_max=5}}
/// ```
pub struct Product {
    /// Amount of the item or fluid to give. If not specified, `amount_min`, `amount_max` and `probability` must all be specified.
    pub amount: Option<Double>,
    /// Maximum amount of the item or fluid to give. Has no effect when `amount` is specified.
    pub amount_max: Option<ProductAmountMaxUnion>,
    /// Minimal amount of the item or fluid to give. Has no effect when `amount` is specified.
    pub amount_min: Option<ProductAmountMinUnion>,
    /// How much of this product is a catalyst.
    pub catalyst_amount: Option<ProductCatalystAmountUnion>,
    /// Prototype name of the result.
    pub name: String,
    /// A value in range [0, 1]. Item or fluid is only given with this probability; otherwise no product is produced.
    pub probability: Option<Double>,
    /// `"item"` or `"fluid"`.
    #[serde(rename = "type")]
    pub typ: String,
    /// Other attributes may be specified depending on `type`:
    pub attributes: Option<ProductAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct ProgrammableSpeakerAlertParameters {
    pub alert_message: String,
    pub icon_signal_id: SignalID,
    pub show_alert: bool,
    pub show_on_map: bool,
}

#[derive(Debug, Deserialize)]
pub struct ProgrammableSpeakerCircuitParameters {
    pub instrument_id: u32,
    pub note_id: u32,
    pub signal_value_is_pitch: bool,
}

#[derive(Debug, Deserialize)]
pub struct ProgrammableSpeakerInstrument {
    pub name: String,
    pub notes: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ProgrammableSpeakerParameters {
    pub allow_polyphony: bool,
    pub playback_globally: bool,
    pub playback_volume: Double,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PrototypeFilterUnion {
    /// for type `"item"`
    ItemPrototypeFilter(ItemPrototypeFilter),
    /// for type `"tile"`
    TilePrototypeFilter(TilePrototypeFilter),
    /// for type `"entity"`
    EntityPrototypeFilter(EntityPrototypeFilter),
    /// for type `"fluid"`
    FluidPrototypeFilter(FluidPrototypeFilter),
    /// for type `"recipe"`
    RecipePrototypeFilter(RecipePrototypeFilter),
    /// for type `"decorative"`
    DecorativePrototypeFilter(DecorativePrototypeFilter),
    /// for type `"achievement"`
    AchievementPrototypeFilter(AchievementPrototypeFilter),
    /// for type `"equipment"`
    EquipmentPrototypeFilter(EquipmentPrototypeFilter),
    /// for type `"technology"`
    TechnologyPrototypeFilter(TechnologyPrototypeFilter),
}

/// Types `"signal"` and `"item-group"` do not support filters.
///
/// # Notes
///
/// * Filters are always used as an array of filters of a specific type. Every filter can only be used with its corresponding event, and different types of event filters can not be mixed.
pub type PrototypeFilter = Vec<PrototypeFilterUnion>;

#[derive(Debug, Deserialize)]
pub struct PrototypeHistory {
    /// The mods that changed this prototype in the order they changed it.
    pub changed: Vec<String>,
    /// The mod that created this prototype.
    pub created: String,
}

/// The smooth orientation. It is a [float](float) in the range `[0, 1)` that covers a full circle, starting at the top and going clockwise. This means a value of `0` indicates "north", a value of `0.5` indicates "south".
///
/// For example then, a value of `0.625` would indicate "south-west", and a value of `0.875` would indicate "north-west".
pub type RealOrientation = Float;

#[derive(Debug, Deserialize)]
pub struct RecipePrototypeFilterAttributesCategory {
    /// A [LuaRecipeCategoryPrototype](LuaRecipeCategoryPrototype) name
    pub category: String,
}

#[derive(Debug, Deserialize)]
pub struct RecipePrototypeFilterAttributesEmissionsMultiplier {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: Double,
}

#[derive(Debug, Deserialize)]
pub struct RecipePrototypeFilterAttributesEnergy {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: Double,
}

#[derive(Debug, Deserialize)]
pub struct RecipePrototypeFilterAttributesHasIngredientFluid {
    /// Matches if at least 1 ingredient is a fluid that matches these filters.
    pub elem_filters: Option<Vec<FluidPrototypeFilter>>,
}

#[derive(Debug, Deserialize)]
pub struct RecipePrototypeFilterAttributesHasIngredientItem {
    /// Matches if at least 1 ingredient is an item that matches these filters.
    pub elem_filters: Option<Vec<ItemPrototypeFilter>>,
}

#[derive(Debug, Deserialize)]
pub struct RecipePrototypeFilterAttributesHasProductFluid {
    /// Matches if at least 1 product is a fluid that matches these filters.
    pub elem_filters: Option<Vec<FluidPrototypeFilter>>,
}

#[derive(Debug, Deserialize)]
pub struct RecipePrototypeFilterAttributesHasProductItem {
    /// Matches if at least 1 product is an item that matches these filters.
    pub elem_filters: Option<Vec<ItemPrototypeFilter>>,
}

#[derive(Debug, Deserialize)]
pub struct RecipePrototypeFilterAttributesOverloadMultiplier {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: u32,
}

#[derive(Debug, Deserialize)]
pub struct RecipePrototypeFilterAttributesRequestPasteMultiplier {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: u32,
}

#[derive(Debug, Deserialize)]
pub struct RecipePrototypeFilterAttributesSubgroup {
    /// A [LuaGroup](LuaGroup) (subgroup) name
    pub subgroup: String,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct RecipePrototypeFilter {
    /// The condition to filter on. One of `"enabled"`, `"hidden"`, `"hidden-from-flow-stats"`, `"hidden-from-player-crafting"`, `"allow-as-intermediate"`, `"allow-intermediates"`, `"allow-decomposition"`, `"always-show-made-in"`, `"always-show-products"`, `"show-amount-in-title"`, `"has-ingredients"`, `"has-products"`, `"has-ingredient-item"`, `"has-ingredient-fluid"`, `"has-product-item"`, `"has-product-fluid"`, `"subgroup"`, `"category"`, `"energy"`, `"emissions-multiplier"`, `"request-paste-multiplier"`, `"overload-multiplier"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<RecipePrototypeFilterAttributes>,
}

/// A number between 0 and 255 inclusive, represented by one of the following named strings or the string version of the number. For example `"27"` and `"decals"` are both valid. Higher values are rendered above lower values.
// ================================
// ========= MANUAL PATCH =========

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenderLayerVariants {
    WaterTile = 15,
    GroundTile = 25,
    TileTransition = 26,
    Decals = 27,
    LowerRadiusVisualization = 29,
    RadiusVisualization = 30,
    TransportBeltIntegration = 65,
    Resource = 66,
    BuildingSmoke = 67,
    Decorative = 92,
    GroundPatch = 93,
    GroundPatchHigher = 94,
    GroundPatchHigher2 = 95,
    Remnants = 112,
    Floor = 113,
    TransportBelt = 114,
    TransportBeltEndings = 115,
    FloorMechanicsUnderCorpse = 120,
    Corpse = 121,
    FloorMechanics = 122,
    Item = 123,
    LowerObject = 124,
    TransportBeltCircuitConnector = 126,
    LowerObjectAboveShadow = 127,
    Object = 129,
    HigherObjectUnder = 131,
    HigherObjectAbove = 132,
    ItemInInserterHand = 134,
    Wires = 135,
    WiresAbove = 136,
    EntityInfoIcon = 138,
    EntityInfoIconAbove = 139,
    Explosion = 142,
    Projectile = 143,
    Smoke = 144,
    AirObject = 145,
    AirEntityInfoIcon = 147,
    LightEffect = 148,
    SelectionBox = 187,
    HigherSelectionBox = 188,
    CollisionSelectionBox = 189,
    Arrow = 190,
    Cursor = 210,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RenderLayer {
    Value(u8),
    Variant(RenderLayerVariants),
}

// ========= MANUAL PATCH =========
// ================================

#[derive(Debug, Deserialize)]
pub struct Resistance {
    /// Absolute damage decrease
    pub decrease: Float,
    /// Percentual damage decrease
    pub percent: Float,
}

#[derive(Debug, Deserialize)]
pub struct RidingState {
    pub acceleration: RidingAcceleration,
    pub direction: RidingDirection,
}

#[derive(Debug, Deserialize)]
/// An area defined using the map editor.
pub struct ScriptArea {
    pub area: BoundingBox,
    pub color: Color,
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
/// A position defined using the map editor.
pub struct ScriptPosition {
    pub color: Color,
    pub id: u32,
    pub name: String,
    pub position: MapPosition,
}

#[derive(Debug, Deserialize)]
pub struct ScriptRenderTarget {
    pub entity: Option<MaybeCycle<LuaEntity>>,
    pub entity_offset: Option<Vector>,
    pub position: Option<MapPosition>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ScriptRenderVertexTargetTargetUnion {
    MapPosition(MapPosition),
    LuaEntity(MaybeCycle<LuaEntity>),
}

#[derive(Debug, Deserialize)]
/// One vertex of a ScriptRenderPolygon.
pub struct ScriptRenderVertexTarget {
    pub target: ScriptRenderVertexTargetTargetUnion,
    /// Only used if `target` is a LuaEntity.
    pub target_offset: Option<Vector>,
}

#[derive(Debug, Deserialize)]
pub struct SelectedPrototypeData {
    /// E.g. `"entity"`.
    pub base_type: String,
    /// E.g. `"tree"`.
    pub derived_type: String,
    /// E.g. `"tree-05"`.
    pub name: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum SelectionModeFlagsUnion {
    /// Selects entities and tiles as if selecting them for a blueprint.
    Blueprint,
    /// Selects entities and tiles as if selecting them for deconstruction.
    Deconstruct,
    /// Selects entities and tiles as if selecting them for deconstruction cancellation.
    CancelDeconstruct,
    /// Selects items on the ground.
    Items,
    /// Selects trees.
    Trees,
    /// Selects entities which are considered a [building](LuaEntityPrototype::is_building), plus landmines.
    BuildableType,
    /// Selects no entities or tiles, but is useful to select an area.
    Nothing,
    /// Selects entities and tiles that can be [built by an item](LuaItemPrototype::place_result).
    ItemsToPlace,
    /// Selects all entities.
    AnyEntity,
    /// Selects all tiles.
    AnyTile,
    /// Selects entities with the same force as the selecting player.
    SameForce,
    /// Selects entities with a different force as the selecting player.
    NotSameForce,
    /// Selects entities from a [friendly](LuaForce::is_friend) force.
    Friend,
    /// Selects entities from an [enemy](LuaForce::is_enemy) force.
    Enemy,
    /// Selects entities as if selecting them for upgrading.
    Upgrade,
    /// Selects entities as if selecting them for upgrade cancellation.
    CancelUpgrade,
    /// Selects entities as if selecting them for downgrading.
    Downgrade,
    /// Selects entities that are [entities with health](LuaEntity::is_entity_with_health).
    EntityWithHealth,
    /// Deprecated. Replaced by `is-military-target`.
    EntityWithForce,
    /// Selects entities that are [military targets](LuaEntity::is_military_target).
    IsMilitaryTarget,
    /// Selects entities that are [entities with owner](LuaEntity::is_entity_with_owner).
    EntityWithOwner,
    /// Selects entities that are not `rolling-stock`s.
    AvoidRollingStock,
    /// Selects entities that are `entity-ghost`s.
    EntityGhost,
    /// Selects entities that are `tile-ghost`s.
    TileGhost,
}

/// A set of flags on a selection tool that define how entities and tiles are selected. Active flags are in the dictionary as `true`, while inactive flags aren't present at all.
pub type SelectionModeFlags = HashMap<SelectionModeFlagsUnion, bool>;

#[derive(Debug, Deserialize)]
/// An actual signal transmitted by the network.
pub struct Signal {
    /// Value of the signal.
    pub count: i32,
    /// ID of the signal.
    pub signal: SignalID,
}

#[derive(Debug, Deserialize)]
pub struct SignalID {
    /// Name of the item, fluid or virtual signal.
    pub name: Option<String>,
    /// `"item"`, `"fluid"`, or `"virtual"`.
    #[serde(rename = "type")]
    pub typ: String,
}

/// An item stack may be specified in one of two ways.
///
/// # Examples
///
/// * Both of these lines specify an item stack of one iron plate:
/// ```text
/// {name="iron-plate"}
/// ```
///
/// ```text
/// {name="iron-plate", count=1}
/// ```
/// * This is a stack of 47 copper plates:
/// ```text
/// {name="copper-plate", count=47}
/// ```
/// * These are both full stacks of iron plates (for iron-plate, a full stack is 100 plates):
/// ```text
/// "iron-plate"
/// ```
///
/// ```text
/// {name="iron-plate", count=100}
/// ```
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SimpleItemStack {
    /// The name of the item, which represents a full stack of that item.
    String(String),
    /// The detailed definition of an item stack.
    ItemStackDefinition(ItemStackDefinition),
}

#[derive(Debug, Deserialize)]
/// # Notes
///
/// * The vectors for all 5 position attributes are a table with `x` and `y` keys instead of an array.
pub struct SmokeSource {
    pub deviation: Option<MapPosition>,
    pub east_position: Option<Vector>,
    pub frequency: Double,
    pub height: Float,
    pub height_deviation: Float,
    pub name: String,
    pub north_position: Option<Vector>,
    pub offset: Double,
    pub position: Option<Vector>,
    pub slow_down_factor: u8,
    pub south_position: Option<Vector>,
    pub starting_frame: u16,
    pub starting_frame_deviation: Double,
    pub starting_frame_speed: u16,
    pub starting_frame_speed_deviation: Double,
    pub starting_vertical_speed: Float,
    pub starting_vertical_speed_deviation: Float,
    pub vertical_speed_slowdown: Float,
    pub west_position: Option<Vector>,
}

/// It can be either the name of a [sound prototype](https://wiki.factorio.com/Prototype/Sound) defined in the data stage, or a path in the form `"type/name"`. The latter option can be sorted into three categories.
///
/// The validity of a SoundPath can be verified at runtime using [LuaGameScript::is_valid_sound_path](LuaGameScript::is_valid_sound_path).
///
/// The utility and ambient types each contain general use sound prototypes defined by the game itself.
/// - `"utility"` - Uses the [UtilitySounds](https://wiki.factorio.com/Prototype/UtilitySounds) prototype. Example: `"utility/wire_connect_pole"`
/// - `"ambient"` - Uses [AmbientSound](https://wiki.factorio.com/Prototype/AmbientSound) prototypes. Example: `"ambient/resource-deficiency"`
///
/// The following types can be combined with any tile name as long as its prototype defines the
///     corresponding sound.
/// - `"tile-walking"` - Uses [Tile::walking_sound](https://wiki.factorio.com/Prototype/Tile#walking_sound). Example: `"tile-walking/concrete"`
/// - `"tile-mined"` - Uses [Tile::mined_sound](https://wiki.factorio.com/Prototype/Tile#mined_sound)
/// - `"tile-build-small"` - Uses [Tile::build_sound](https://wiki.factorio.com/Prototype/Tile#build_sound). Example: `"tile-build-small/concrete"`
/// - `"tile-build-medium"` - Uses [Tile::build_sound](https://wiki.factorio.com/Prototype/Tile#build_sound)
/// - `"tile-build-large"` - Uses [Tile::build_sound](https://wiki.factorio.com/Prototype/Tile#build_sound)
///
/// The following types can be combined with any entity name as long as its prototype defines the
///     corresponding sound.
/// - `"entity-build"` - Uses [Entity::build_sound](https://wiki.factorio.com/Prototype/Entity#build_sound). Example: `"entity-build/wooden-chest"`
/// - `"entity-mined"` - Uses [Entity::mined_sound](https://wiki.factorio.com/Prototype/Entity#mined_sound)
/// - `"entity-mining"` - Uses [Entity::mining_sound](https://wiki.factorio.com/Prototype/Entity#mining_sound)
/// - `"entity-vehicle_impact"` - Uses [Entity::vehicle_impact_sound](https://wiki.factorio.com/Prototype/Entity#vehicle_impact_sound)
/// - `"entity-rotated"` - Uses [Entity::rotated_sound](https://wiki.factorio.com/Prototype/Entity#rotated_sound)
/// - `"entity-open"` - Uses [Entity::open_sound](https://wiki.factorio.com/Prototype/Entity#open_sound)
/// - `"entity-close"` - Uses [Entity::close_sound](https://wiki.factorio.com/Prototype/Entity#close_sound)
pub type SoundPath = String;

/// Defines which slider in the game's sound settings affects the volume of this sound. Furthermore, some sound types are mixed differently than others, e.g. zoom level effects are applied.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SoundType {
    GameEffect,
    GuiEffect,
    Ambient,
    Environment,
    Walking,
    Alert,
    Wind,
}

#[derive(Debug, Deserialize)]
pub struct SpawnPointDefinition {
    /// Evolution factor for which this weight applies.
    pub evolution_factor: Double,
    /// Probability of spawning this unit at this evolution factor.
    pub weight: Double,
}

/// It can be either the name of a [sprite prototype](https://wiki.factorio.com/Prototype/Sprite) defined in the data stage, or a path in form "type/name".
///
/// The validity of a SpritePath can be verified at runtime using [LuaGameScript::is_valid_sprite_path](LuaGameScript::is_valid_sprite_path).
///
/// The supported types are:
/// - `"item"` - for example "item/iron-plate" is the icon sprite of iron plate
/// - `"entity"` - for example "entity/small-biter" is the icon sprite of the small biter
/// - `"technology"`
/// - `"recipe"`
/// - `"item-group"`
/// - `"fluid"`
/// - `"tile"`
/// - `"virtual-signal"`
/// - `"achievement"`
/// - `"equipment"`
/// - `"file"` - path to an image file located inside the current scenario. This file is not preloaded so it will be slower; for frequently used sprites, it is better to define sprite prototype and use it instead.
/// - `"utility"` - sprite defined in the utility-sprites object, these are the pictures used by the game internally for the UI.
pub type SpritePath = String;

#[derive(Debug, Deserialize)]
pub struct SteeringMapSetting {
    /// Used to make steering look better for aesthetic purposes.
    pub force_unit_fuzzy_goto_behavior: bool,
    /// Does not include the radius of the unit.
    pub radius: Double,
    pub separation_factor: Double,
    pub separation_force: Double,
}

#[derive(Debug, Deserialize)]
pub struct SteeringMapSettings {
    pub default: SteeringMapSetting,
    pub moving: SteeringMapSetting,
}

/// A surface may be specified in one of three ways.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SurfaceIdentification {
    /// It will be the index of the surface. `nauvis` has index `1`, the first surface-created surface will have index `2` and so on.
    Uint(u32),
    /// It will be the surface name. E.g. `"nauvis"`.
    String(String),
    /// A reference to [LuaSurface](LuaSurface) may be passed directly.
    LuaSurface(MaybeCycle<LuaSurface>),
}

#[derive(Debug, Deserialize)]
pub struct TabAndContent {
    pub content: MaybeCycle<LuaGuiElement>,
    pub tab: MaybeCycle<LuaGuiElement>,
}

/// A dictionary of string to the four basic Lua types: `string`, `boolean`, `number`, `table`.
///
/// Note that the API returns tags as a simple table, meaning any modifications to it will not propagate back to the game. Thus, to modify a set of tags, the whole table needs to be written back to the respective property.
///
/// # Examples
///
/// * ```text
/// {a = 1, b = true, c = "three", d = {e = "f"}}
/// ```
pub type Tags = HashMap<String, AnyBasic>;

/// A technology may be specified in one of three ways.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TechnologyIdentification {
    /// The technology name.
    String(String),
    /// A reference to [LuaTechnology](LuaTechnology) may be passed directly.
    LuaTechnology(MaybeCycle<LuaTechnology>),
    /// A reference to [LuaTechnologyPrototype](LuaTechnologyPrototype) may be passed directly.
    LuaTechnologyPrototype(MaybeCycle<LuaTechnologyPrototype>),
}

#[derive(Debug, Deserialize)]
pub struct TechnologyModifierAttributesOtherTypes {
    /// Modification value. This value will be added to the variable it modifies.
    pub modifier: Double,
}

#[derive(Debug, Deserialize)]
pub struct TechnologyModifierAttributesAmmoDamage {
    /// Prototype name of the ammunition category that is affected
    pub ammo_category: String,
    /// Modification value. This will be added to the current ammo damage modifier upon researching.
    pub modifier: Double,
}

#[derive(Debug, Deserialize)]
pub struct TechnologyModifierAttributesGiveItem {
    /// Number of items to give. Defaults to `1`.
    pub count: Option<u32>,
    /// Item prototype name to give.
    pub item: String,
}

#[derive(Debug, Deserialize)]
pub struct TechnologyModifierAttributesGunSpeed {
    /// Prototype name of the ammunition category that is affected
    pub ammo_category: String,
    /// Modification value. This will be added to the current gun speed modifier upon researching.
    pub modifier: Double,
}

#[derive(Debug, Deserialize)]
pub struct TechnologyModifierAttributesNothing {
    /// Description of this nothing modifier.
    pub effect_description: LocalisedString,
}

#[derive(Debug, Deserialize)]
pub struct TechnologyModifierAttributesTurretAttack {
    /// Modification value. This will be added to the current turret damage modifier upon researching.
    pub modifier: Double,
    /// Turret prototype name this modifier will affect.
    pub turret_id: String,
}

#[derive(Debug, Deserialize)]
pub struct TechnologyModifierAttributesUnlockRecipe {
    /// Recipe prototype name to unlock.
    pub recipe: String,
}

#[derive(Debug, Deserialize)]
pub enum TechnologyModifierAttributes {
    OtherTypes(TechnologyModifierAttributesOtherTypes),
    AmmoDamage(TechnologyModifierAttributesAmmoDamage),
    GiveItem(TechnologyModifierAttributesGiveItem),
    GunSpeed(TechnologyModifierAttributesGunSpeed),
    Nothing(TechnologyModifierAttributesNothing),
    TurretAttack(TechnologyModifierAttributesTurretAttack),
    UnlockRecipe(TechnologyModifierAttributesUnlockRecipe),
}

#[derive(Debug, Deserialize)]
/// The effect that is applied when a technology is researched. It is a table that contains at least the field `type`.
pub struct TechnologyModifier {
    /// Modifier type. Specifies which of the other fields will be available. Possible values are: `"inserter-stack-size-bonus"`, `"stack-inserter-capacity-bonus"`, `"laboratory-speed"`, `"character-logistic-trash-slots"`, `"maximum-following-robots-count"`, `"worker-robot-speed"`, `"worker-robot-storage"`, `"ghost-time-to-live"`, `"turret-attack"`, `"ammo-damage"`, `"give-item"`, `"gun-speed"`, `"unlock-recipe"`, `"character-crafting-speed"`, `"character-mining-speed"`, `"character-running-speed"`, `"character-build-distance"`, `"character-item-drop-distance"`, `"character-reach-distance"`, `"character-resource-reach-distance"`, `"character-item-pickup-distance"`, `"character-loot-pickup-distance"`, `"character-inventory-slots-bonus"`, `"deconstruction-time-to-live"`, `"max-failed-attempts-per-tick-per-construction-queue"`, `"max-successful-attempts-per-tick-per-construction-queue"`, `"character-health-bonus"`, `"mining-drill-productivity-bonus"`, `"train-braking-force-bonus"`, `"zoom-to-world-enabled"`, `"zoom-to-world-ghost-building-enabled"`, `"zoom-to-world-blueprint-enabled"`, `"zoom-to-world-deconstruction-planner-enabled"`, `"zoom-to-world-upgrade-planner-enabled"`, `"zoom-to-world-selection-tool-enabled"`, `"worker-robot-battery"`, `"laboratory-productivity"`, `"follower-robot-lifetime"`, `"artillery-range"`, `"nothing"`, `"character-additional-mining-categories"`, `"character-logistic-requests"`.
    #[serde(rename = "type")]
    pub typ: String,
    /// Other attributes may be specified depending on `type`:
    pub attributes: Option<TechnologyModifierAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct TechnologyPrototypeFilterAttributesLevel {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: u32,
}

#[derive(Debug, Deserialize)]
pub struct TechnologyPrototypeFilterAttributesMaxLevel {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: u32,
}

#[derive(Debug, Deserialize)]
pub struct TechnologyPrototypeFilterAttributesResearchUnitIngredient {
    /// The research ingredient to check.
    pub ingredient: String,
}

#[derive(Debug, Deserialize)]
pub struct TechnologyPrototypeFilterAttributesTime {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: u32,
}

#[derive(Debug, Deserialize)]
pub struct TechnologyPrototypeFilterAttributesUnlocksRecipe {
    /// The recipe to check.
    pub recipe: String,
}

#[derive(Debug, Deserialize)]
pub enum TechnologyPrototypeFilterAttributes {
    Level(TechnologyPrototypeFilterAttributesLevel),
    MaxLevel(TechnologyPrototypeFilterAttributesMaxLevel),
    ResearchUnitIngredient(TechnologyPrototypeFilterAttributesResearchUnitIngredient),
    Time(TechnologyPrototypeFilterAttributesTime),
    UnlocksRecipe(TechnologyPrototypeFilterAttributesUnlocksRecipe),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct TechnologyPrototypeFilter {
    /// The condition to filter on. One of `"enabled"`, `"hidden"`, `"upgrade"`, `"visible-when-disabled"`, `"has-effects"`, `"has-prerequisites"`, `"research-unit-ingredient"`, `"unlocks-recipe"`, `"level"`, `"max-level"`, `"time"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<TechnologyPrototypeFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct Tile {
    /// The prototype name of the tile.
    pub name: String,
    /// The position of the tile.
    pub position: TilePosition,
}

#[derive(Debug, Deserialize)]
/// Coordinates of a tile on a [LuaSurface](LuaSurface) where each integer `x`/`y` represents a different tile. This uses the same format as [MapPosition](MapPosition), except it rounds any non-integer `x`/`y` down to whole numbers. It can be specified either with or without explicit keys.
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TilePrototypeFilterAttributesMaskUnion {
    CollisionMask(CollisionMask),
    CollisionMaskWithFlags(CollisionMaskWithFlags),
}

#[derive(Debug, Deserialize)]
pub struct TilePrototypeFilterAttributesCollisionMask {
    pub mask: TilePrototypeFilterAttributesMaskUnion,
    /// How to filter: `"collides"`, `"layers-equals"`, `"contains-any"` or `"contains-all"`
    pub mask_mode: String,
}

#[derive(Debug, Deserialize)]
pub struct TilePrototypeFilterAttributesDecorativeRemovalProbability {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: Float,
}

#[derive(Debug, Deserialize)]
pub struct TilePrototypeFilterAttributesEmissions {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: Double,
}

#[derive(Debug, Deserialize)]
pub struct TilePrototypeFilterAttributesVehicleFrictionModifier {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: Double,
}

#[derive(Debug, Deserialize)]
pub struct TilePrototypeFilterAttributesWalkingSpeedModifier {
    pub comparison: ComparatorString,
    /// The value to compare against.
    pub value: Double,
}

#[derive(Debug, Deserialize)]
pub enum TilePrototypeFilterAttributes {
    CollisionMask(TilePrototypeFilterAttributesCollisionMask),
    DecorativeRemovalProbability(TilePrototypeFilterAttributesDecorativeRemovalProbability),
    Emissions(TilePrototypeFilterAttributesEmissions),
    VehicleFrictionModifier(TilePrototypeFilterAttributesVehicleFrictionModifier),
    WalkingSpeedModifier(TilePrototypeFilterAttributesWalkingSpeedModifier),
}

#[derive(Debug, Deserialize)]
/// Depending on the value of `filter`, the table may take additional fields. `filter` may be one of the following:
pub struct TilePrototypeFilter {
    /// The condition to filter on. One of `"minable"`, `"autoplace"`, `"blueprintable"`, `"item-to-place"`, `"collision-mask"`, `"walking-speed-modifier"`, `"vehicle-friction-modifier"`, `"decorative-removal-probability"`, `"emissions"`.
    pub filter: String,
    /// Inverts the condition. Default is `false`.
    pub invert: Option<bool>,
    /// How to combine this with the previous filter. Must be `"or"` or `"and"`. Defaults to `"or"`. When evaluating the filters, `"and"` has higher precedence than `"or"`.
    pub mode: Option<String>,
    /// Other attributes may be specified depending on `filter`:
    pub attributes: Option<TilePrototypeFilterAttributes>,
}

#[derive(Debug, Deserialize)]
pub struct TrainSchedule {
    /// Index of the currently active record
    pub current: u32,
    pub records: Vec<TrainScheduleRecord>,
}

#[derive(Debug, Deserialize)]
pub struct TrainScheduleRecord {
    /// Rail to path to. Ignored if `station` is present.
    pub rail: Option<MaybeCycle<LuaEntity>>,
    /// When a train is allowed to reach rail target from any direction it will be `nil`. If rail has to be reached from specific direction, this value allows to choose the direction. This value corresponds to [LuaEntity::connected_rail_direction](LuaEntity::connected_rail_direction) of a TrainStop.
    pub rail_direction: Option<RailDirection>,
    /// Name of the station.
    pub station: Option<String>,
    /// Only present when the station is temporary, the value is then always `true`.
    pub temporary: Option<bool>,
    pub wait_conditions: Option<Vec<WaitCondition>>,
}

#[derive(Debug, Deserialize)]
pub struct TriggerDelivery {
    pub source_effects: Option<Vec<TriggerEffectItem>>,
    pub target_effects: Option<Vec<TriggerEffectItem>>,
    /// One of `"instant"`, `"projectile"`, `"flame-thrower"`, `"beam"`, `"stream"`, `"artillery"`.
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct TriggerEffectItem {
    pub affects_target: Option<bool>,
    pub repeat_count: Option<u32>,
    pub show_in_tooltip: Option<bool>,
    /// One of`"damage"`, `"create-entity"`, `"create-explosion"`, `"create-fire"`, `"create-smoke"`, `"create-trivial-smoke"`, `"create-particle"`, `"create-sticker"`, `"nested-result"`, `"play-sound"`, `"push-back"`, `"destroy-cliffs"`, `"show-explosion-on-chart"`, `"insert-item"`, `"script"`.
    #[serde(rename = "type")]
    pub typ: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TriggerItem {
    pub action_delivery: Option<Vec<TriggerDelivery>>,
    /// The trigger will only affect entities that would collide with given collision mask.
    pub collision_mask: CollisionMask,
    /// The trigger will only affect entities that contain any of these flags.
    pub entity_flags: Option<EntityPrototypeFlags>,
    /// If `"enemy"`, the trigger will only affect entities whose force is different from the attacker's and for which there is no cease-fire set. `"ally"` is the opposite of `"enemy"`.
    pub force: ForceCondition,
    pub ignore_collision_condition: bool,
    pub repeat_count: u32,
    pub trigger_target_mask: TriggerTargetMask,
    /// One of `"direct"`, `"area"`, `"line"`, `"cluster"`.
    #[serde(rename = "type")]
    pub typ: String,
}

/// A set of trigger target masks.
pub type TriggerTargetMask = HashMap<String, bool>;

#[derive(Debug, Deserialize)]
pub struct UnitGroupMapSettings {
    /// The maximum number of automatically created unit groups gathering for attack at any time. Defaults to `30`.
    pub max_gathering_unit_groups: u32,
    /// The maximum amount of time in ticks a group will spend gathering before setting off. The actual time is a random time between the minimum and maximum times. Defaults to `10*3,600=36,000` ticks.
    pub max_group_gathering_time: u32,
    /// When a member of a group falls back more than this factor times the group radius, the group will slow down to its `max_group_slowdown_factor` speed to let them catch up. Defaults to `3`.
    pub max_group_member_fallback_factor: Double,
    /// The maximum group radius in tiles. The actual radius is adjusted based on the number of members. Defaults to `30.0`.
    pub max_group_radius: Double,
    /// The minimum speed as a percentage of its maximum speed that a group will slow down to so members that fell behind can catch up. Defaults to `0.3`, or 30%.
    pub max_group_slowdown_factor: Double,
    /// The minimum speed a percentage of its regular speed that a group member can slow down to when ahead of the group. Defaults to `0.6`, or 60%.
    pub max_member_slowdown_when_ahead: Double,
    /// The maximum speed a percentage of its regular speed that a group member can speed up to when catching up with the group. Defaults to `1.4`, or 140%.
    pub max_member_speedup_when_behind: Double,
    /// The maximum number of members for an attack unit group. This only affects automatically created unit groups, manual groups created through the API are unaffected. Defaults to `200`.
    pub max_unit_group_size: u32,
    /// After gathering has finished, the group is allowed to wait this long in ticks for delayed members. New members are not accepted anymore however. Defaults to `2*3,600=7,200` ticks.
    pub max_wait_time_for_late_members: u32,
    /// When a member of a group falls back more than this factor times the group radius, it will be dropped from the group. Defaults to `10`.
    pub member_disown_distance: Double,
    /// The minimum amount of time in ticks a group will spend gathering before setting off. The actual time is a random time between the minimum and maximum times. Defaults to `3,600` ticks.
    pub min_group_gathering_time: u32,
    /// The minimum group radius in tiles. The actual radius is adjusted based on the number of members. Defaults to `5.0`.
    pub min_group_radius: Double,
    pub tick_tolerance_when_member_arrives: u32,
}

#[derive(Debug, Deserialize)]
pub struct UnitSpawnDefinition {
    /// The points at which to spawn the unit.
    pub spawn_points: Vec<SpawnPointDefinition>,
    /// Prototype name of the unit that would be spawned.
    pub unit: String,
}

#[derive(Debug, Deserialize)]
pub struct UpgradeFilter {
    /// Name of the item, or entity.
    pub name: Option<String>,
    /// `"item"`, or `"entity"`.
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
/// A vector is a two-element array containing the `x` and `y` components. In some specific cases, the vector is a table with `x` and `y` keys instead, which the documentation will point out.
///
/// # Examples
///
/// * ```text
/// right = {1.0, 0.0}
/// ```
pub struct Vector {
    pub x: Float,
    pub y: Float,
}

#[derive(Debug, Deserialize)]
pub struct VehicleAutomaticTargetingParameters {
    pub auto_target_with_gunner: bool,
    pub auto_target_without_gunner: bool,
}

#[derive(Debug, Deserialize)]
pub struct WaitCondition {
    /// Either `"and"`, or `"or"`. Tells how this condition is to be compared with the preceding conditions in the corresponding `wait_conditions` array.
    pub compare_type: String,
    /// Only present when `type` is `"item_count"`, `"circuit"` or `"fluid_count"`, and a circuit condition is configured.
    pub condition: Option<CircuitCondition>,
    /// Number of ticks to wait when `type` is `"time"`, or number of ticks of inactivity when `type` is `"inactivity"`.
    pub ticks: Option<u32>,
    /// One of `"time"`, `"inactivity"`, `"full"`, `"empty"`, `"item_count"`, `"circuit"`, `"robots_inactive"`, `"fluid_count"`, `"passenger_present"`, `"passenger_not_present"`.
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct WireConnectionDefinition {
    /// Mandatory if the source entity has more than one circuit connection using circuit wire.
    pub source_circuit_id: Option<CircuitConnectorId>,
    /// Mandatory if the source entity has more than one wire connection using copper wire.
    pub source_wire_id: Option<WireConnectionId>,
    /// Mandatory if the target entity has more than one circuit connection using circuit wire.
    pub target_circuit_id: Option<CircuitConnectorId>,
    /// The entity to (dis)connect the source entity with.
    pub target_entity: MaybeCycle<LuaEntity>,
    /// Mandatory if the target entity has more than one wire connection using copper wire.
    pub target_wire_id: Option<WireConnectionId>,
    /// The type of wire used.
    pub wire: WireType,
}
