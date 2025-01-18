#[derive(serde::Deserialize)]
pub struct AutoplaceSpecification {
    control: crate::types::AutoplaceControlID,
    #[serde(default = "default_default_enabled")]
    default_enabled: bool,
    #[serde(default = "default_force")]
    force: AutoplaceSpecificationForce,
    local_expressions: std::collections::HashMap<String, crate::types::NoiseExpression>,
    local_functions: std::collections::HashMap<String, crate::types::NoiseFunction>,
    #[serde(default = "default_order")]
    order: crate::types::Order,
    #[serde(default = "default_placement_density")]
    placement_density: u32,
    probability_expression: crate::types::NoiseExpression,
    richness_expression: crate::types::NoiseExpression,
    tile_restriction: Vec<TileIDRestriction>,
}
fn default_default_enabled() -> bool {
    true
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
fn default_force() -> AutoplaceSpecificationForce {
    AutoplaceSpecificationForce::Neutral
}
fn default_order() -> crate::types::Order {
    String::from("")
}
fn default_placement_density() -> u32 {
    1
}
#[derive(serde::Deserialize)]
pub enum TileIDRestriction {
    #[serde(untagged)]
    TileID(crate::types::TileID),
    #[serde(untagged)]
    TileIDTileID((crate::types::TileID, crate::types::TileID)),
}
