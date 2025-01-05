pub struct AutoplaceSpecification {
    control: crate::types::AutoplaceControlID,
    default_enabled: bool,
    force: AutoplaceSpecificationForce,
    local_expressions: std::collections::HashMap<String, crate::types::NoiseExpression>,
    local_functions: std::collections::HashMap<String, crate::types::NoiseFunction>,
    order: crate::types::Order,
    placement_density: u32,
    probability_expression: crate::types::NoiseExpression,
    richness_expression: crate::types::NoiseExpression,
    tile_restriction: Vec<TileIDRestriction>,
}
pub enum AutoplaceSpecificationForce {
    Enemy,
    Player,
    Neutral,
    String(String),
}
pub enum TileIDRestriction {
    TileID(crate::types::TileID),
    TileIDTileID((crate::types::TileID, crate::types::TileID)),
}
