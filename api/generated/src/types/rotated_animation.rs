#[derive(Debug, serde::Deserialize)]
pub struct RotatedAnimation {
    #[serde(flatten)]
    base_: crate::types::AnimationParameters,
    #[serde(default = "default_apply_projection")]
    apply_projection: bool,
    #[serde(default = "default_axially_symmetrical")]
    axially_symmetrical: bool,
    #[serde(default = "default_counterclockwise")]
    counterclockwise: bool,
    #[serde(default = "default_direction_count")]
    direction_count: u32,
    filename: Option<crate::types::FileName>,
    filenames: Option<crate::vec::Vec<crate::types::FileName>>,
    layers: Option<crate::vec::Vec<crate::types::RotatedAnimation>>,
    lines_per_file: Option<u32>,
    #[serde(default = "default_middle_orientation")]
    middle_orientation: crate::types::RealOrientation,
    #[serde(default = "default_orientation_range")]
    orientation_range: f32,
    slice: Option<u32>,
    #[serde(default = "default_still_frame")]
    still_frame: u32,
    stripes: Option<crate::vec::Vec<crate::types::Stripe>>,
}
fn default_apply_projection() -> bool {
    true
}
fn default_axially_symmetrical() -> bool {
    false
}
fn default_counterclockwise() -> bool {
    false
}
fn default_direction_count() -> u32 {
    1
}
fn default_middle_orientation() -> crate::types::RealOrientation {
    0.5
}
fn default_orientation_range() -> f32 {
    1.0
}
fn default_still_frame() -> u32 {
    0
}
