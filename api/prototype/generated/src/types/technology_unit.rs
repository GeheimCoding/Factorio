#[derive(serde::Deserialize)]
pub struct TechnologyUnit {
    count: Option<u64>,
    count_formula: Option<crate::types::MathExpression>,
    ingredients: Vec<crate::types::ResearchIngredient>,
    time: f64,
}
