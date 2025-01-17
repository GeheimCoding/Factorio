#[derive(serde::Deserialize)]
pub struct EntityPrototype {
    base_: crate::prototypes::Prototype,
    additional_pastable_entities: Vec<crate::types::EntityID>,
    alert_icon_scale: f32,
    alert_icon_shift: crate::types::Vector,
    allow_copy_paste: bool,
    ambient_sounds: EntityPrototypeAmbientSounds,
    ambient_sounds_group: crate::types::EntityID,
    autoplace: crate::types::AutoplaceSpecification,
    build_base_evolution_requirement: f64,
    build_grid_size: u8,
    build_sound: crate::types::Sound,
    close_sound: crate::types::Sound,
    collision_box: crate::types::BoundingBox,
    collision_mask: crate::types::CollisionMaskConnector,
    created_effect: crate::types::Trigger,
    created_smoke: crate::types::CreateTrivialSmokeEffectItem,
    deconstruction_alternative: crate::types::EntityID,
    diagonal_tile_grid_size: crate::types::TilePosition,
    drawing_box_vertical_extension: f64,
    emissions_per_second: std::collections::HashMap<crate::types::AirbornePollutantID, f64>,
    enemy_map_color: crate::types::Color,
    fast_replaceable_group: String,
    flags: crate::types::EntityPrototypeFlags,
    friendly_map_color: crate::types::Color,
    heating_energy: crate::types::Energy,
    hit_visualization_box: crate::types::BoundingBox,
    icon: crate::types::FileName,
    icon_draw_specification: crate::types::IconDrawSpecification,
    icon_size: crate::types::SpriteSizeType,
    icons: Vec<crate::types::IconData>,
    icons_positioning: Vec<crate::types::IconSequencePositioning>,
    impact_category: String,
    map_color: crate::types::Color,
    map_generator_bounding_box: crate::types::BoundingBox,
    minable: crate::types::MinableProperties,
    mined_sound: crate::types::Sound,
    mining_sound: crate::types::Sound,
    next_upgrade: crate::types::EntityID,
    open_sound: crate::types::Sound,
    order: crate::types::Order,
    placeable_by: EntityPrototypePlaceableBy,
    placeable_position_visualization: crate::types::Sprite,
    protected_from_tile_building: bool,
    radius_visualisation_specification: crate::types::RadiusVisualisationSpecification,
    remains_when_mined: EntityPrototypeRemainsWhenMined,
    remove_decoratives: EntityPrototypeRemoveDecoratives,
    rotated_sound: crate::types::Sound,
    selectable_in_game: bool,
    selection_box: crate::types::BoundingBox,
    selection_priority: u8,
    shooting_cursor_size: f64,
    stateless_visualisation: crate::types::StatelessVisualisations,
    sticker_box: crate::types::BoundingBox,
    surface_conditions: Vec<crate::types::SurfaceCondition>,
    tile_buildability_rules: Vec<crate::types::TileBuildabilityRule>,
    tile_height: i32,
    tile_width: i32,
    trigger_target_mask: crate::types::TriggerTargetMask,
    water_reflection: crate::types::WaterReflectionDefinition,
    working_sound: crate::types::WorkingSound,
}
#[derive(serde::Deserialize)]
pub enum EntityPrototypeAmbientSounds {
    #[serde(untagged)]
    WorldAmbientSoundDefinition(crate::types::WorldAmbientSoundDefinition),
    #[serde(untagged)]
    VecWorldAmbientSoundDefinition(Vec<crate::types::WorldAmbientSoundDefinition>),
}
#[derive(serde::Deserialize)]
pub enum EntityPrototypePlaceableBy {
    #[serde(untagged)]
    ItemToPlace(Box<crate::types::ItemToPlace>),
    #[serde(untagged)]
    VecItemToPlace(Vec<crate::types::ItemToPlace>),
}
#[derive(serde::Deserialize)]
pub enum EntityPrototypeRemainsWhenMined {
    #[serde(untagged)]
    EntityID(crate::types::EntityID),
    #[serde(untagged)]
    VecEntityID(Vec<crate::types::EntityID>),
}
#[derive(serde::Deserialize)]
pub enum EntityPrototypeRemoveDecoratives {
    #[serde(rename = "automatic")]
    Automatic,
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
}
