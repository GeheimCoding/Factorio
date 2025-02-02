#[derive(Debug, serde::Deserialize)]
pub struct NamedNoiseFunction {
    #[serde(flatten)]
    base_: crate::prototypes::Prototype,
    expression: crate::types::NoiseExpression,
    local_expressions: Option<std::collections::HashMap<String, crate::types::NoiseExpression>>,
    local_functions: Option<std::collections::HashMap<String, crate::types::NoiseFunction>>,
    parameters: Vec<String>,
}
