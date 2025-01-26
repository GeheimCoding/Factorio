#[derive(Debug, serde::Deserialize)]
pub struct GlobalRecipeTints {
    // default: `{1, 1, 1, 1}`
    primary: Option<crate::types::Color>,
    // default: `{1, 1, 1, 1}`
    quaternary: Option<crate::types::Color>,
    // default: `{1, 1, 1, 1}`
    secondary: Option<crate::types::Color>,
    // default: `{1, 1, 1, 1}`
    tertiary: Option<crate::types::Color>,
}
