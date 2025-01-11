pub struct RailSupportPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    build_grid_size: String,
    collision_mask_allow_on_deep_oil_ocean: crate::types::CollisionMaskConnector,
    elevated_selection_boxes: Vec<crate::types::BoundingBox>,
    graphics_set: RailSupportGraphicsSet,
    not_buildable_if_no_rails: bool,
    snap_to_spots_distance: f32,
    support_range: f32,
}
pub struct RailSupportGraphicsSet {
    render_layer: crate::types::RenderLayer,
    structure: crate::types::RotatedSprite,
    underwater_layer_offset: i8,
    underwater_structure: crate::types::RotatedSprite,
}
