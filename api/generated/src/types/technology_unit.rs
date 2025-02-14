#[derive(Debug, serde::Deserialize)]
pub struct TechnologyUnit {
    count: Option<u64>,
    count_formula: Option<crate::types::MathExpression>,
    ingredients: crate::vec::Vec<crate::types::ResearchIngredient>,
    time: f64,
}
