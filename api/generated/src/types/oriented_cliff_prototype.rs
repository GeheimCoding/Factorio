#[derive(Debug, serde::Deserialize)]
pub struct OrientedCliffPrototype {
    collision_bounding_box: crate::types::BoundingBox,
    pictures: Option<crate::types::SpriteVariations>,
    pictures_lower: Option<crate::types::SpriteVariations>,
    render_layer: Option<crate::types::RenderLayer>,
}
