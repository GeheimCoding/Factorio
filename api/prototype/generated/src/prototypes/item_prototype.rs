pub struct ItemPrototype {
    burnt_result: crate::types::ItemID,
    close_sound: crate::types::Sound,
    color_hint: crate::types::ColorHintSpecification,
    dark_background_icon: crate::types::FileName,
    dark_background_icon_size: crate::types::SpriteSizeType,
    dark_background_icons: Vec<crate::types::IconData>,
    default_import_location: crate::types::SpaceLocationID,
    destroyed_by_dropping_trigger: crate::types::Trigger,
    drop_sound: crate::types::Sound,
    flags: crate::types::ItemPrototypeFlags,
    fuel_acceleration_multiplier: f64,
    fuel_acceleration_multiplier_quality_bonus: f64,
    fuel_category: crate::types::FuelCategoryID,
    fuel_emissions_multiplier: f64,
    fuel_glow_color: crate::types::Color,
    fuel_top_speed_multiplier: f64,
    fuel_top_speed_multiplier_quality_bonus: f64,
    fuel_value: crate::types::Energy,
    has_random_tint: bool,
    icon: crate::types::FileName,
    icon_size: crate::types::SpriteSizeType,
    icons: Vec<crate::types::IconData>,
    ingredient_to_weight_coefficient: f64,
    inventory_move_sound: crate::types::Sound,
    open_sound: crate::types::Sound,
    pick_sound: crate::types::Sound,
    pictures: crate::types::SpriteVariations,
    place_as_equipment_result: crate::types::EquipmentID,
    place_as_tile: PlaceAsTile,
    place_result: crate::types::EntityID,
    plant_result: crate::types::EntityID,
    random_tint_color: crate::types::Color,
    rocket_launch_products: Vec<crate::types::ItemProductPrototype>,
    send_to_orbit_mode: crate::types::SendToOrbitMode,
    spoil_result: crate::types::ItemID,
    spoil_ticks: u32,
    spoil_to_trigger_result: crate::types::SpoilToTriggerResult,
    stack_size: crate::types::ItemCountType,
    weight: crate::types::Weight,
}
pub struct PlaceAsTile {
    condition: crate::types::CollisionMaskConnector,
    condition_size: u32,
    invert: bool,
    result: crate::types::TileID,
    tile_condition: Vec<crate::types::TileID>,
}
