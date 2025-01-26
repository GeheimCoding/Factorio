#[derive(Debug, serde::Deserialize)]
pub struct BoilerPictures {
    fire: Option<crate::types::Animation>,
    fire_glow: Option<crate::types::Animation>,
    patch: Option<crate::types::Sprite>,
    structure: crate::types::Animation,
}
