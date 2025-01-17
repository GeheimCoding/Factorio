#[derive(serde::Deserialize)]
pub struct SpriteNWaySheet {
    base_: crate::types::SpriteParameters,
    frames: u32,
    generate_sdf: bool,
}
