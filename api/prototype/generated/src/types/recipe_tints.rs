#[derive(serde::Deserialize)]
pub struct RecipeTints {
    // default: no tint
    primary: crate::types::Color,
    // default: no tint
    quaternary: crate::types::Color,
    // default: no tint
    secondary: crate::types::Color,
    // default: no tint
    tertiary: crate::types::Color,
}
