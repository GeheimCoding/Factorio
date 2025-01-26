#[derive(Debug, serde::Deserialize)]
pub struct RecipeTints {
    // default: no tint
    primary: Option<crate::types::Color>,
    // default: no tint
    quaternary: Option<crate::types::Color>,
    // default: no tint
    secondary: Option<crate::types::Color>,
    // default: no tint
    tertiary: Option<crate::types::Color>,
}
