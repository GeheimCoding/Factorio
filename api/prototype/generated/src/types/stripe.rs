#[derive(serde::Deserialize)]
pub struct Stripe {
    filename: crate::types::FileName,
    height_in_frames: u32,
    width_in_frames: u32,
    #[serde(default = "default_x")]
    x: u32,
    #[serde(default = "default_y")]
    y: u32,
}
fn default_x() -> u32 {
    0
}
fn default_y() -> u32 {
    0
}
