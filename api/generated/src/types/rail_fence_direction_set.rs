#[derive(Debug, serde::Deserialize)]
pub struct RailFenceDirectionSet {
    east: Option<crate::types::SpriteVariations>,
    north: Option<crate::types::SpriteVariations>,
    northeast: Option<crate::types::SpriteVariations>,
    northwest: Option<crate::types::SpriteVariations>,
    south: Option<crate::types::SpriteVariations>,
    southeast: Option<crate::types::SpriteVariations>,
    southwest: Option<crate::types::SpriteVariations>,
    west: Option<crate::types::SpriteVariations>,
}
