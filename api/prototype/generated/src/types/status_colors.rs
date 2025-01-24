#[derive(serde::Deserialize)]
pub struct StatusColors {
    // default: idle
    disabled: Option<crate::types::Color>,
    // default: idle
    full_output: Option<crate::types::Color>,
    // default: `{1, 1, 1, 1}`
    idle: Option<crate::types::Color>,
    // default: idle
    insufficient_input: Option<crate::types::Color>,
    // default: working
    low_power: Option<crate::types::Color>,
    // default: idle
    no_minable_resources: Option<crate::types::Color>,
    // default: No color
    no_power: Option<crate::types::Color>,
    // default: `{1, 1, 1, 1}`
    working: Option<crate::types::Color>,
}
