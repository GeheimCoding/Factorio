pub struct NoiseFunction {
    expression: NoiseExpression,
    local_expressions: std::collections::HashMap<String, NoiseExpression>,
    local_functions: std::collections::HashMap<String, NoiseFunction>,
    parameters: Vec<String>,
}
