#[derive(serde::Deserialize)]
pub struct Stripe {
    filename: crate::types::FileName,
    height_in_frames: u32,
    width_in_frames: u32,
    x: u32,
    y: u32,
}
