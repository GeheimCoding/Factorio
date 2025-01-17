#[derive(serde::Deserialize)]
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
#[derive(serde::Deserialize)]
pub enum AutoplaceSpecificationForce {
    #[serde(rename = "enemy")]
    Enemy,
    #[serde(rename = "player")]
    Player,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(untagged)]
    String(String),
}
#[derive(serde::Deserialize)]
pub enum TileIDRestriction {
    #[serde(untagged)]
    TileID(crate::types::TileID),
    #[serde(untagged)]
    TileIDTileID((crate::types::TileID, crate::types::TileID)),
}
