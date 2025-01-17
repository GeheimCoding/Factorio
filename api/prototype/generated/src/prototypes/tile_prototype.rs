pub struct TilePrototype {
    base_: crate::prototypes::Prototype,
    absorptions_per_second: std::collections::HashMap<crate::types::AirbornePollutantID, f64>,
    allowed_neighbors: Vec<crate::types::TileID>,
    allows_being_covered: bool,
    ambient_sounds: TilePrototypeAmbientSounds,
    ambient_sounds_group: crate::types::TileID,
    autoplace: crate::types::AutoplaceSpecification,
    bound_decoratives: TilePrototypeBoundDecoratives,
    build_animations: crate::types::Animation4Way,
    build_animations_background: crate::types::Animation4Way,
    build_sound: TilePrototypeBuildSound,
    built_animation_frame: u32,
    can_be_part_of_blueprint: bool,
    check_collision_with_entities: bool,
    collision_mask: crate::types::CollisionMaskConnector,
    decorative_removal_probability: f32,
    default_cover_tile: crate::types::TileID,
    default_destroyed_dropped_item_trigger: crate::types::Trigger,
    destroys_dropped_items: bool,
    driving_sound: crate::types::Sound,
    dying_explosion: TilePrototypeDyingExplosion,
    effect: crate::types::TileEffectDefinitionID,
    effect_color: crate::types::Color,
    effect_color_secondary: crate::types::Color,
    effect_is_opaque: bool,
    fluid: crate::types::FluidID,
    frozen_variant: crate::types::TileID,
    icon: crate::types::FileName,
    icon_size: crate::types::SpriteSizeType,
    icons: Vec<crate::types::IconData>,
    is_foundation: bool,
    landing_steps_sound: crate::types::Sound,
    layer: u8,
    layer_group: crate::types::TileRenderLayer,
    lowland_fog: bool,
    map_color: crate::types::Color,
    max_health: f32,
    minable: crate::types::MinableProperties,
    mined_sound: crate::types::Sound,
    needs_correction: bool,
    next_direction: crate::types::TileID,
    particle_tints: crate::types::TileBasedParticleTints,
    placeable_by: TilePrototypePlaceableBy,
    scorch_mark_color: crate::types::Color,
    searchable: bool,
    sprite_usage_surface: crate::types::SpriteUsageSurfaceHint,
    thawed_variant: crate::types::TileID,
    tint: crate::types::Color,
    transition_merges_with_tile: crate::types::TileID,
    transition_overlay_layer_offset: i8,
    transitions: Vec<crate::types::TileTransitionsToTiles>,
    transitions_between_transitions: Vec<crate::types::TileTransitionsBetweenTransitions>,
    trigger_effect: crate::types::TriggerEffect,
    variants: crate::types::TileTransitionsVariants,
    vehicle_friction_modifier: f64,
    walking_sound: crate::types::Sound,
    walking_speed_modifier: f64,
    weight: crate::types::Weight,
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
pub struct TileBuildSound {
    animated: crate::types::Sound,
    large: crate::types::Sound,
    medium: crate::types::Sound,
    small: crate::types::Sound,
}
#[derive(serde::Deserialize)]
pub enum TilePrototypeDyingExplosion {
    #[serde(untagged)]
    ExplosionDefinition(crate::types::ExplosionDefinition),
    #[serde(untagged)]
    VecExplosionDefinition(Vec<crate::types::ExplosionDefinition>),
}
#[derive(serde::Deserialize)]
pub enum TilePrototypePlaceableBy {
    #[serde(untagged)]
    ItemToPlace(Box<crate::types::ItemToPlace>),
    #[serde(untagged)]
    VecItemToPlace(Vec<crate::types::ItemToPlace>),
}
