#[derive(serde::Deserialize)]
pub struct OrientedCliffPrototype {
    collision_bounding_box: crate::types::BoundingBox,
    pictures: crate::types::SpriteVariations,
    pictures_lower: crate::types::SpriteVariations,
    render_layer: crate::types::RenderLayer,
}
