pub struct Animation {
    base_: crate::types::AnimationParameters,
    filename: crate::types::FileName,
    filenames: Vec<crate::types::FileName>,
    layers: Vec<crate::types::Animation>,
    lines_per_file: u32,
    slice: u32,
    stripes: Vec<crate::types::Stripe>,
}
