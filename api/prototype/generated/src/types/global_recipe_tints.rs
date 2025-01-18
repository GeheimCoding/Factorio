#[derive(serde::Deserialize)]
pub struct GlobalRecipeTints {
    // default: `{1, 1, 1, 1}`
    primary: crate::types::Color,
    // default: `{1, 1, 1, 1}`
    quaternary: crate::types::Color,
    // default: `{1, 1, 1, 1}`
    secondary: crate::types::Color,
    // default: `{1, 1, 1, 1}`
    tertiary: crate::types::Color,
}
