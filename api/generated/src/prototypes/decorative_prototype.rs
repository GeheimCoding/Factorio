#[derive(Debug, serde::Deserialize)]
pub struct DecorativePrototype {
    #[serde(flatten)]
    base_: crate::prototypes::Prototype,
    autoplace: Option<crate::types::AutoplaceSpecification>,
    collision_box: Option<crate::types::BoundingBox>,
    collision_mask: Option<crate::types::CollisionMaskConnector>,
    #[serde(default = "default_decal_overdraw_priority")]
    decal_overdraw_priority: u16,
    #[serde(default = "default_grows_through_rail_path")]
    grows_through_rail_path: bool,
    #[serde(default = "default_minimal_separation")]
    minimal_separation: f64,
    pictures: crate::types::SpriteVariations,
    placed_effect: Option<crate::types::TriggerEffect>,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    stateless_visualisation: Option<crate::types::StatelessVisualisations>,
    stateless_visualisation_variations:
        Option<crate::vec::Vec<crate::types::StatelessVisualisations>>,
    #[serde(default = "default_target_count")]
    target_count: u16,
    #[serde(default = "default_tile_layer")]
    tile_layer: i16,
    trigger_effect: Option<crate::types::TriggerEffect>,
    walking_sound: Option<crate::types::Sound>,
}
fn default_decal_overdraw_priority() -> u16 {
    0
}
fn default_grows_through_rail_path() -> bool {
    false
}
fn default_minimal_separation() -> f64 {
    0.0
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Decorative
}
fn default_target_count() -> u16 {
    0
}
fn default_tile_layer() -> i16 {
    0
}
