#[derive(Debug, serde::Deserialize)]
pub struct NoiseFunction {
    expression: crate::types::NoiseExpression,
    local_expressions: Option<std::collections::HashMap<String, crate::types::NoiseExpression>>,
    local_functions: Option<std::collections::HashMap<String, crate::types::NoiseFunction>>,
    parameters: crate::vec::Vec<String>,
}
