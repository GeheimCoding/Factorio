#[derive(serde::Deserialize)]
pub struct ItemPrototype {
    base_: crate::prototypes::Prototype,
    #[serde(default = "default_burnt_result")]
    burnt_result: crate::types::ItemID,
    close_sound: crate::types::Sound,
    color_hint: crate::types::ColorHintSpecification,
    dark_background_icon: crate::types::FileName,
    #[serde(default = "default_dark_background_icon_size")]
    dark_background_icon_size: crate::types::SpriteSizeType,
    dark_background_icons: Vec<crate::types::IconData>,
    #[serde(default = "default_default_import_location")]
    default_import_location: crate::types::SpaceLocationID,
    destroyed_by_dropping_trigger: crate::types::Trigger,
    drop_sound: crate::types::Sound,
    flags: crate::types::ItemPrototypeFlags,
    #[serde(default = "default_fuel_acceleration_multiplier")]
    fuel_acceleration_multiplier: f64,
    fuel_acceleration_multiplier_quality_bonus: f64,
    #[serde(default = "default_fuel_category")]
    fuel_category: crate::types::FuelCategoryID,
    #[serde(default = "default_fuel_emissions_multiplier")]
    fuel_emissions_multiplier: f64,
    fuel_glow_color: crate::types::Color,
    #[serde(default = "default_fuel_top_speed_multiplier")]
    fuel_top_speed_multiplier: f64,
    fuel_top_speed_multiplier_quality_bonus: f64,
    #[serde(default = "default_fuel_value")]
    fuel_value: crate::types::Energy,
    #[serde(default = "default_has_random_tint")]
    has_random_tint: bool,
    icon: crate::types::FileName,
    #[serde(default = "default_icon_size")]
    icon_size: crate::types::SpriteSizeType,
    icons: Vec<crate::types::IconData>,
    #[serde(default = "default_ingredient_to_weight_coefficient")]
    ingredient_to_weight_coefficient: f64,
    inventory_move_sound: crate::types::Sound,
    open_sound: crate::types::Sound,
    pick_sound: crate::types::Sound,
    pictures: crate::types::SpriteVariations,
    #[serde(default = "default_place_as_equipment_result")]
    place_as_equipment_result: crate::types::EquipmentID,
    place_as_tile: PlaceAsTile,
    #[serde(default = "default_place_result")]
    place_result: crate::types::EntityID,
    plant_result: crate::types::EntityID,
    // default: Value of UtilityConstants::item_default_random_tint_strength
    random_tint_color: crate::types::Color,
    rocket_launch_products: Vec<crate::types::ItemProductPrototype>,
    #[serde(default = "default_send_to_orbit_mode")]
    send_to_orbit_mode: crate::types::SendToOrbitMode,
    spoil_result: crate::types::ItemID,
    #[serde(default = "default_spoil_ticks")]
    spoil_ticks: u32,
    spoil_to_trigger_result: crate::types::SpoilToTriggerResult,
    stack_size: crate::types::ItemCountType,
    weight: crate::types::Weight,
}
fn default_burnt_result() -> crate::types::ItemID {
    String::from("")
}
fn default_dark_background_icon_size() -> crate::types::SpriteSizeType {
    64
}
fn default_default_import_location() -> crate::types::SpaceLocationID {
    String::from("nauvis")
}
fn default_fuel_acceleration_multiplier() -> f64 {
    1.0
}
fn default_fuel_category() -> crate::types::FuelCategoryID {
    String::from("")
}
fn default_fuel_emissions_multiplier() -> f64 {
    1.0
}
fn default_fuel_top_speed_multiplier() -> f64 {
    1.0
}
fn default_fuel_value() -> crate::types::Energy {
    String::from("0J")
}
fn default_has_random_tint() -> bool {
    true
}
fn default_icon_size() -> crate::types::SpriteSizeType {
    64
}
fn default_ingredient_to_weight_coefficient() -> f64 {
    0.5
}
fn default_place_as_equipment_result() -> crate::types::EquipmentID {
    String::from("")
}
#[derive(serde::Deserialize)]
pub struct PlaceAsTile {
    condition: crate::types::CollisionMaskConnector,
    condition_size: u32,
    #[serde(default = "default_invert")]
    invert: bool,
    result: crate::types::TileID,
    tile_condition: Vec<crate::types::TileID>,
}
fn default_invert() -> bool {
    false
}
fn default_place_result() -> crate::types::EntityID {
    String::from("")
}
fn default_send_to_orbit_mode() -> crate::types::SendToOrbitMode {
    crate::types::SendToOrbitMode::NotSendable
}
fn default_spoil_ticks() -> u32 {
    0
}
