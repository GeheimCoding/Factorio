pub struct RotatedAnimation {
    base_: crate::types::AnimationParameters,
    apply_projection: bool,
    axially_symmetrical: bool,
    counterclockwise: bool,
    direction_count: u32,
    filename: crate::types::FileName,
    filenames: Vec<crate::types::FileName>,
    layers: Vec<crate::types::RotatedAnimation>,
    lines_per_file: u32,
    middle_orientation: crate::types::RealOrientation,
    orientation_range: f32,
    slice: u32,
    still_frame: u32,
    stripes: Vec<crate::types::Stripe>,
}
