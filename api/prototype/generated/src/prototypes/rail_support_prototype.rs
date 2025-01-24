#[derive(serde::Deserialize)]
pub struct RailSupportPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    #[serde(default = "default_build_grid_size")]
    build_grid_size: f64,
    collision_mask_allow_on_deep_oil_ocean: Option<crate::types::CollisionMaskConnector>,
    elevated_selection_boxes: Option<Vec<crate::types::BoundingBox>>,
    graphics_set: RailSupportGraphicsSet,
    #[serde(default = "default_not_buildable_if_no_rails")]
    not_buildable_if_no_rails: bool,
    #[serde(default = "default_snap_to_spots_distance")]
    snap_to_spots_distance: f32,
    #[serde(default = "default_support_range")]
    support_range: f32,
}
fn default_build_grid_size() -> f64 {
    2.0
}
#[derive(serde::Deserialize)]
pub struct RailSupportGraphicsSet {
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    structure: crate::types::RotatedSprite,
    #[serde(default = "default_underwater_layer_offset")]
    underwater_layer_offset: i8,
    underwater_structure: Option<crate::types::RotatedSprite>,
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_underwater_layer_offset() -> i8 {
    1
}
fn default_not_buildable_if_no_rails() -> bool {
    false
}
fn default_snap_to_spots_distance() -> f32 {
    1.0
}
fn default_support_range() -> f32 {
    15.0
}
