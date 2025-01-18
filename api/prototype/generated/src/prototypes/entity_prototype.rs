#[derive(serde::Deserialize)]
pub struct EntityPrototype {
    base_: crate::prototypes::Prototype,
    additional_pastable_entities: Vec<crate::types::EntityID>,
    alert_icon_scale: f32,
    alert_icon_shift: crate::types::Vector,
    #[serde(default = "default_allow_copy_paste")]
    allow_copy_paste: bool,
    ambient_sounds: EntityPrototypeAmbientSounds,
    ambient_sounds_group: crate::types::EntityID,
    // default: nil (entity is not autoplacable)
    autoplace: crate::types::AutoplaceSpecification,
    #[serde(default = "default_build_base_evolution_requirement")]
    build_base_evolution_requirement: f64,
    #[serde(default = "default_build_grid_size")]
    build_grid_size: u8,
    build_sound: crate::types::Sound,
    close_sound: crate::types::Sound,
    // default: Empty = `{{0, 0}, {0, 0}}`
    collision_box: crate::types::BoundingBox,
    collision_mask: crate::types::CollisionMaskConnector,
    created_effect: crate::types::Trigger,
    // default: The "smoke-building"-smoke
    created_smoke: crate::types::CreateTrivialSmokeEffectItem,
    deconstruction_alternative: crate::types::EntityID,
    diagonal_tile_grid_size: crate::types::TilePosition,
    #[serde(default = "default_drawing_box_vertical_extension")]
    drawing_box_vertical_extension: f64,
    emissions_per_second: std::collections::HashMap<crate::types::AirbornePollutantID, f64>,
    enemy_map_color: crate::types::Color,
    #[serde(default = "default_fast_replaceable_group")]
    fast_replaceable_group: String,
    flags: crate::types::EntityPrototypeFlags,
    friendly_map_color: crate::types::Color,
    heating_energy: crate::types::Energy,
    // default: Empty = `{{0, 0}, {0, 0}}`
    hit_visualization_box: crate::types::BoundingBox,
    icon: crate::types::FileName,
    icon_draw_specification: crate::types::IconDrawSpecification,
    #[serde(default = "default_icon_size")]
    icon_size: crate::types::SpriteSizeType,
    icons: Vec<crate::types::IconData>,
    icons_positioning: Vec<crate::types::IconSequencePositioning>,
    #[serde(default = "default_impact_category")]
    impact_category: String,
    map_color: crate::types::Color,
    // default: The value of collision_box.
    map_generator_bounding_box: crate::types::BoundingBox,
    // default: not minable
    minable: crate::types::MinableProperties,
    mined_sound: crate::types::Sound,
    mining_sound: crate::types::Sound,
    next_upgrade: crate::types::EntityID,
    open_sound: crate::types::Sound,
    order: crate::types::Order,
    placeable_by: EntityPrototypePlaceableBy,
    placeable_position_visualization: crate::types::Sprite,
    #[serde(default = "default_protected_from_tile_building")]
    protected_from_tile_building: bool,
    radius_visualisation_specification: crate::types::RadiusVisualisationSpecification,
    remains_when_mined: EntityPrototypeRemainsWhenMined,
    #[serde(default = "default_remove_decoratives")]
    remove_decoratives: EntityPrototypeRemoveDecoratives,
    rotated_sound: crate::types::Sound,
    #[serde(default = "default_selectable_in_game")]
    selectable_in_game: bool,
    // default: Empty = `{{0, 0}, {0, 0}}`
    selection_box: crate::types::BoundingBox,
    #[serde(default = "default_selection_priority")]
    selection_priority: u8,
    shooting_cursor_size: f64,
    stateless_visualisation: crate::types::StatelessVisualisations,
    // default: The value of collision_box.
    sticker_box: crate::types::BoundingBox,
    surface_conditions: Vec<crate::types::SurfaceCondition>,
    tile_buildability_rules: Vec<crate::types::TileBuildabilityRule>,
    // default: calculated by the collision box height rounded up.
    tile_height: i32,
    // default: calculated by the collision box width rounded up.
    tile_width: i32,
    trigger_target_mask: crate::types::TriggerTargetMask,
    water_reflection: crate::types::WaterReflectionDefinition,
    working_sound: crate::types::WorkingSound,
}
fn default_allow_copy_paste() -> bool {
    true
}
#[derive(serde::Deserialize)]
pub enum EntityPrototypeAmbientSounds {
    #[serde(untagged)]
    WorldAmbientSoundDefinition(crate::types::WorldAmbientSoundDefinition),
    #[serde(untagged)]
    VecWorldAmbientSoundDefinition(Vec<crate::types::WorldAmbientSoundDefinition>),
}
fn default_build_base_evolution_requirement() -> f64 {
    0.0
}
fn default_build_grid_size() -> u8 {
    1
}
fn default_drawing_box_vertical_extension() -> f64 {
    0.0
}
fn default_fast_replaceable_group() -> String {
    String::from("")
}
fn default_icon_size() -> crate::types::SpriteSizeType {
    64
}
fn default_impact_category() -> String {
    String::from("default")
}
#[derive(serde::Deserialize)]
pub enum EntityPrototypePlaceableBy {
    #[serde(untagged)]
    ItemToPlace(Box<crate::types::ItemToPlace>),
    #[serde(untagged)]
    VecItemToPlace(Vec<crate::types::ItemToPlace>),
}
fn default_protected_from_tile_building() -> bool {
    true
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
fn default_remove_decoratives() -> EntityPrototypeRemoveDecoratives {
    EntityPrototypeRemoveDecoratives::Automatic
}
fn default_selectable_in_game() -> bool {
    true
}
fn default_selection_priority() -> u8 {
    50
}
