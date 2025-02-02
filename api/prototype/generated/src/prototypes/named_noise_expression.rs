#[derive(Debug, serde::Deserialize)]
pub struct NamedNoiseExpression {
    #[serde(flatten)]
    base_: crate::prototypes::Prototype,
    expression: crate::types::NoiseExpression,
    intended_property: Option<String>,
    local_expressions: Option<std::collections::HashMap<String, crate::types::NoiseExpression>>,
    local_functions: Option<std::collections::HashMap<String, crate::types::NoiseFunction>>,
    order: Option<crate::types::Order>,
}
