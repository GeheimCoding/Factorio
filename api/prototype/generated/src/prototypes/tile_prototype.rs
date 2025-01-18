#[derive(serde::Deserialize)]
pub struct TilePrototype {
    base_: crate::prototypes::Prototype,
    absorptions_per_second: std::collections::HashMap<crate::types::AirbornePollutantID, f64>,
    // default: All tiles
    allowed_neighbors: Vec<crate::types::TileID>,
    #[serde(default = "default_allows_being_covered")]
    allows_being_covered: bool,
    ambient_sounds: TilePrototypeAmbientSounds,
    ambient_sounds_group: crate::types::TileID,
    autoplace: crate::types::AutoplaceSpecification,
    bound_decoratives: TilePrototypeBoundDecoratives,
    build_animations: crate::types::Animation4Way,
    build_animations_background: crate::types::Animation4Way,
    build_sound: TilePrototypeBuildSound,
    built_animation_frame: u32,
    #[serde(default = "default_can_be_part_of_blueprint")]
    can_be_part_of_blueprint: bool,
    #[serde(default = "default_check_collision_with_entities")]
    check_collision_with_entities: bool,
    collision_mask: crate::types::CollisionMaskConnector,
    #[serde(default = "default_decorative_removal_probability")]
    decorative_removal_probability: f32,
    default_cover_tile: crate::types::TileID,
    default_destroyed_dropped_item_trigger: crate::types::Trigger,
    #[serde(default = "default_destroys_dropped_items")]
    destroys_dropped_items: bool,
    driving_sound: crate::types::Sound,
    dying_explosion: TilePrototypeDyingExplosion,
    effect: crate::types::TileEffectDefinitionID,
    // default: `{r=1, g=1, b=1, a=1} (white)`
    effect_color: crate::types::Color,
    effect_color_secondary: crate::types::Color,
    // default: true if `effect_color` alpha equals 1
    effect_is_opaque: bool,
    fluid: crate::types::FluidID,
    frozen_variant: crate::types::TileID,
    icon: crate::types::FileName,
    #[serde(default = "default_icon_size")]
    icon_size: crate::types::SpriteSizeType,
    icons: Vec<crate::types::IconData>,
    #[serde(default = "default_is_foundation")]
    is_foundation: bool,
    landing_steps_sound: crate::types::Sound,
    layer: u8,
    #[serde(default = "default_layer_group")]
    layer_group: crate::types::TileRenderLayer,
    #[serde(default = "default_lowland_fog")]
    lowland_fog: bool,
    map_color: crate::types::Color,
    #[serde(default = "default_max_health")]
    max_health: f32,
    minable: crate::types::MinableProperties,
    mined_sound: crate::types::Sound,
    #[serde(default = "default_needs_correction")]
    needs_correction: bool,
    next_direction: crate::types::TileID,
    particle_tints: crate::types::TileBasedParticleTints,
    placeable_by: TilePrototypePlaceableBy,
    scorch_mark_color: crate::types::Color,
    #[serde(default = "default_searchable")]
    searchable: bool,
    #[serde(default = "default_sprite_usage_surface")]
    sprite_usage_surface: crate::types::SpriteUsageSurfaceHint,
    thawed_variant: crate::types::TileID,
    // default: `{r=1, g=1, b=1, a=1} (white)`
    tint: crate::types::Color,
    transition_merges_with_tile: crate::types::TileID,
    #[serde(default = "default_transition_overlay_layer_offset")]
    transition_overlay_layer_offset: i8,
    transitions: Vec<crate::types::TileTransitionsToTiles>,
    transitions_between_transitions: Vec<crate::types::TileTransitionsBetweenTransitions>,
    trigger_effect: crate::types::TriggerEffect,
    variants: crate::types::TileTransitionsVariants,
    #[serde(default = "default_vehicle_friction_modifier")]
    vehicle_friction_modifier: f64,
    walking_sound: crate::types::Sound,
    #[serde(default = "default_walking_speed_modifier")]
    walking_speed_modifier: f64,
    #[serde(default = "default_weight")]
    weight: crate::types::Weight,
}
fn default_allows_being_covered() -> bool {
    true
}
#[derive(serde::Deserialize)]
pub enum TilePrototypeAmbientSounds {
    #[serde(untagged)]
    WorldAmbientSoundDefinition(crate::types::WorldAmbientSoundDefinition),
    #[serde(untagged)]
    VecWorldAmbientSoundDefinition(Vec<crate::types::WorldAmbientSoundDefinition>),
}
#[derive(serde::Deserialize)]
pub enum TilePrototypeBoundDecoratives {
    #[serde(untagged)]
    DecorativeID(crate::types::DecorativeID),
    #[serde(untagged)]
    VecDecorativeID(Vec<crate::types::DecorativeID>),
}
#[derive(serde::Deserialize)]
pub enum TilePrototypeBuildSound {
    #[serde(untagged)]
    Sound(crate::types::Sound),
    #[serde(untagged)]
    TileBuildSound(Box<TileBuildSound>),
}
#[derive(serde::Deserialize)]
pub struct TileBuildSound {
    animated: crate::types::Sound,
    large: crate::types::Sound,
    medium: crate::types::Sound,
    small: crate::types::Sound,
}
fn default_can_be_part_of_blueprint() -> bool {
    true
}
fn default_check_collision_with_entities() -> bool {
    true
}
fn default_decorative_removal_probability() -> f32 {
    0.0
}
fn default_destroys_dropped_items() -> bool {
    false
}
#[derive(serde::Deserialize)]
pub enum TilePrototypeDyingExplosion {
    #[serde(untagged)]
    ExplosionDefinition(crate::types::ExplosionDefinition),
    #[serde(untagged)]
    VecExplosionDefinition(Vec<crate::types::ExplosionDefinition>),
}
fn default_icon_size() -> crate::types::SpriteSizeType {
    64
}
fn default_is_foundation() -> bool {
    false
}
fn default_layer_group() -> crate::types::TileRenderLayer {
    crate::types::TileRenderLayer::GroundNatural
}
fn default_lowland_fog() -> bool {
    false
}
fn default_max_health() -> f32 {
    0.0
}
fn default_needs_correction() -> bool {
    false
}
#[derive(serde::Deserialize)]
pub enum TilePrototypePlaceableBy {
    #[serde(untagged)]
    ItemToPlace(Box<crate::types::ItemToPlace>),
    #[serde(untagged)]
    VecItemToPlace(Vec<crate::types::ItemToPlace>),
}
fn default_searchable() -> bool {
    false
}
fn default_sprite_usage_surface() -> crate::types::SpriteUsageSurfaceHint {
    crate::types::SpriteUsageSurfaceHint::Any
}
fn default_transition_overlay_layer_offset() -> i8 {
    0
}
fn default_vehicle_friction_modifier() -> f64 {
    1.0
}
fn default_walking_speed_modifier() -> f64 {
    1.0
}
fn default_weight() -> crate::types::Weight {
    0.0
}
