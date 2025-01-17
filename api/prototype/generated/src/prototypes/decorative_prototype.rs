#[derive(serde::Deserialize)]
pub struct DecorativePrototype {
    base_: crate::prototypes::Prototype,
    autoplace: crate::types::AutoplaceSpecification,
    collision_box: crate::types::BoundingBox,
    collision_mask: crate::types::CollisionMaskConnector,
    decal_overdraw_priority: u16,
    grows_through_rail_path: bool,
    minimal_separation: f64,
    pictures: crate::types::SpriteVariations,
    placed_effect: crate::types::TriggerEffect,
    render_layer: crate::types::RenderLayer,
    stateless_visualisation: crate::types::StatelessVisualisations,
    stateless_visualisation_variations: Vec<crate::types::StatelessVisualisations>,
    target_count: u16,
    tile_layer: i16,
    trigger_effect: crate::types::TriggerEffect,
    walking_sound: crate::types::Sound,
}
