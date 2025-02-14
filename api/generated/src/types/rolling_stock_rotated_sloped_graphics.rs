#[derive(Debug, serde::Deserialize)]
pub struct RollingStockRotatedSlopedGraphics {
    rotated: crate::types::RotatedSprite,
    #[serde(default = "default_slope_angle_between_frames")]
    slope_angle_between_frames: f64,
    #[serde(default = "default_slope_back_equals_front")]
    slope_back_equals_front: bool,
    sloped: Option<crate::types::RotatedSprite>,
}
fn default_slope_angle_between_frames() -> f64 {
    1.3
}
fn default_slope_back_equals_front() -> bool {
    false
}
