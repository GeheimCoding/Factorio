pub struct AutoplaceSpecification {
    control: AutoplaceControlID,
    default_enabled: bool,
    force: AutoplaceSpecificationForce,
    local_expressions: std::collections::HashMap<String, NoiseExpression>,
    local_functions: std::collections::HashMap<String, NoiseFunction>,
    order: Order,
    placement_density: u32,
    probability_expression: NoiseExpression,
    richness_expression: NoiseExpression,
    tile_restriction: Vec<TileIDRestriction>,
}
pub enum AutoplaceSpecificationForce {
    Enemy,
    Player,
    Neutral,
    String(String),
}
