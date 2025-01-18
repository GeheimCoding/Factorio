#[derive(serde::Deserialize)]
pub struct TileBasedParticleTints {
    // default: {1,1,1,1} (white)
    primary: crate::types::Color,
    // default: Value of `primary`
    secondary: crate::types::Color,
}
