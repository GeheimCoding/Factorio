#[derive(Debug, serde::Deserialize)]
pub struct TileBasedParticleTints {
    // default: {1,1,1,1} (white)
    primary: Option<crate::types::Color>,
    // default: Value of `primary`
    secondary: Option<crate::types::Color>,
}
