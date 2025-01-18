#[derive(serde::Deserialize)]
pub struct StatusColors {
    // default: idle
    disabled: crate::types::Color,
    // default: idle
    full_output: crate::types::Color,
    // default: `{1, 1, 1, 1}`
    idle: crate::types::Color,
    // default: idle
    insufficient_input: crate::types::Color,
    // default: working
    low_power: crate::types::Color,
    // default: idle
    no_minable_resources: crate::types::Color,
    // default: No color
    no_power: crate::types::Color,
    // default: `{1, 1, 1, 1}`
    working: crate::types::Color,
}
