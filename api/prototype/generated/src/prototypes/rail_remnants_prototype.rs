pub struct RailRemnantsPrototype {
    build_grid_size: String,
    collision_box: crate::types::BoundingBox,
    pictures: crate::types::RailPictureSet,
    related_rail: crate::types::EntityID,
    secondary_collision_box: crate::types::BoundingBox,
}
