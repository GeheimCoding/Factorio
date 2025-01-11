pub struct NamedNoiseFunction {
    base_: crate::prototypes::Prototype,
    expression: crate::types::NoiseExpression,
    local_expressions: std::collections::HashMap<String, crate::types::NoiseExpression>,
    local_functions: std::collections::HashMap<String, crate::types::NoiseFunction>,
    parameters: Vec<String>,
}
