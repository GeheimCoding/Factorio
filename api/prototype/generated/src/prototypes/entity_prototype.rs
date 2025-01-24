#[derive(serde::Deserialize)]
pub struct EntityPrototype {
    base_: crate::prototypes::Prototype,
    additional_pastable_entities: Option<Vec<crate::types::EntityID>>,
    alert_icon_scale: Option<f32>,
    alert_icon_shift: Option<crate::types::Vector>,
    #[serde(default = "default_allow_copy_paste")]
    allow_copy_paste: bool,
    ambient_sounds: Option<EntityPrototypeAmbientSounds>,
    ambient_sounds_group: Option<crate::types::EntityID>,
    // default: nil (entity is not autoplacable)
    autoplace: Option<crate::types::AutoplaceSpecification>,
    #[serde(default = "default_build_base_evolution_requirement")]
    build_base_evolution_requirement: f64,
    #[serde(default = "default_build_grid_size")]
    build_grid_size: u8,
    build_sound: Option<crate::types::Sound>,
    close_sound: Option<crate::types::Sound>,
    // default: Empty = `{{0, 0}, {0, 0}}`
    collision_box: Option<crate::types::BoundingBox>,
    collision_mask: Option<crate::types::CollisionMaskConnector>,
    created_effect: Option<crate::types::Trigger>,
    // default: The "smoke-building"-smoke
    created_smoke: Option<crate::types::CreateTrivialSmokeEffectItem>,
    deconstruction_alternative: Option<crate::types::EntityID>,
    diagonal_tile_grid_size: Option<crate::types::TilePosition>,
    #[serde(default = "default_drawing_box_vertical_extension")]
    drawing_box_vertical_extension: f64,
    emissions_per_second: Option<std::collections::HashMap<crate::types::AirbornePollutantID, f64>>,
    enemy_map_color: Option<crate::types::Color>,
    #[serde(default = "default_fast_replaceable_group")]
    fast_replaceable_group: String,
    flags: Option<crate::types::EntityPrototypeFlags>,
    friendly_map_color: Option<crate::types::Color>,
    heating_energy: Option<crate::types::Energy>,
    // default: Empty = `{{0, 0}, {0, 0}}`
    hit_visualization_box: Option<crate::types::BoundingBox>,
    icon: Option<crate::types::FileName>,
    icon_draw_specification: Option<crate::types::IconDrawSpecification>,
    #[serde(default = "default_icon_size")]
    icon_size: crate::types::SpriteSizeType,
    icons: Option<Vec<crate::types::IconData>>,
    icons_positioning: Option<Vec<crate::types::IconSequencePositioning>>,
    #[serde(default = "default_impact_category")]
    impact_category: String,
    map_color: Option<crate::types::Color>,
    // default: The value of collision_box.
    map_generator_bounding_box: Option<crate::types::BoundingBox>,
    // default: not minable
    minable: Option<crate::types::MinableProperties>,
    mined_sound: Option<crate::types::Sound>,
    mining_sound: Option<crate::types::Sound>,
    next_upgrade: Option<crate::types::EntityID>,
    open_sound: Option<crate::types::Sound>,
    order: Option<crate::types::Order>,
    placeable_by: Option<EntityPrototypePlaceableBy>,
    placeable_position_visualization: Option<crate::types::Sprite>,
    #[serde(default = "default_protected_from_tile_building")]
    protected_from_tile_building: bool,
    radius_visualisation_specification: Option<crate::types::RadiusVisualisationSpecification>,
    remains_when_mined: Option<EntityPrototypeRemainsWhenMined>,
    #[serde(default = "default_remove_decoratives")]
    remove_decoratives: EntityPrototypeRemoveDecoratives,
    rotated_sound: Option<crate::types::Sound>,
    #[serde(default = "default_selectable_in_game")]
    selectable_in_game: bool,
    // default: Empty = `{{0, 0}, {0, 0}}`
    selection_box: Option<crate::types::BoundingBox>,
    #[serde(default = "default_selection_priority")]
    selection_priority: u8,
    shooting_cursor_size: Option<f64>,
    stateless_visualisation: Option<crate::types::StatelessVisualisations>,
    // default: The value of collision_box.
    sticker_box: Option<crate::types::BoundingBox>,
    surface_conditions: Option<Vec<crate::types::SurfaceCondition>>,
    tile_buildability_rules: Option<Vec<crate::types::TileBuildabilityRule>>,
    // default: calculated by the collision box height rounded up.
    tile_height: Option<i32>,
    // default: calculated by the collision box width rounded up.
    tile_width: Option<i32>,
    trigger_target_mask: Option<crate::types::TriggerTargetMask>,
    water_reflection: Option<crate::types::WaterReflectionDefinition>,
    working_sound: Option<crate::types::WorkingSound>,
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
