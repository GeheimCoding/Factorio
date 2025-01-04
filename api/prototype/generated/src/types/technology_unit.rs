pub struct TechnologyUnit {
    count: u64,
    count_formula: crate::types::MathExpression,
    ingredients: Vec<crate::types::ResearchIngredient>,
    time: f64,
}
