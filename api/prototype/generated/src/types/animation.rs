#[derive(Debug, serde::Deserialize)]
pub struct Animation {
    base_: crate::types::AnimationParameters,
    filename: Option<crate::types::FileName>,
    filenames: Option<Vec<crate::types::FileName>>,
    layers: Option<Vec<crate::types::Animation>>,
    lines_per_file: Option<u32>,
    // default: Value of `frame_count`
    slice: Option<u32>,
    stripes: Option<Vec<crate::types::Stripe>>,
}
