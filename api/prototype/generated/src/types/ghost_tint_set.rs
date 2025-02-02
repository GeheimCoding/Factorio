#[derive(Debug, serde::Deserialize)]
pub struct GhostTintSet {
    ghost_delivery_tint: crate::types::Color,
    ghost_tint: crate::types::Color,
    tile_ghost_delivery_tint: crate::types::Color,
    tile_ghost_tint: crate::types::Color,
    wire_tint: crate::types::Color,
}
