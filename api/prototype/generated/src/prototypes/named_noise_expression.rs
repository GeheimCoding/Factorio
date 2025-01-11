pub struct NamedNoiseExpression {
    base_: crate::prototypes::Prototype,
    expression: crate::types::NoiseExpression,
    intended_property: String,
    local_expressions: std::collections::HashMap<String, crate::types::NoiseExpression>,
    local_functions: std::collections::HashMap<String, crate::types::NoiseFunction>,
    order: crate::types::Order,
}
