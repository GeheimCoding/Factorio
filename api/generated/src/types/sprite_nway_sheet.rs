#[derive(Debug, serde::Deserialize)]
pub struct SpriteNWaySheet {
    #[serde(flatten)]
    base_: crate::types::SpriteParameters,
    // default: 4 if used in Sprite4Way, 8 if used in Sprite8Way
    frames: Option<u32>,
    #[serde(default = "default_generate_sdf")]
    generate_sdf: bool,
}
fn default_generate_sdf() -> bool {
    false
}
