#[derive(Debug, serde::Deserialize)]
pub struct Animation {
    #[serde(flatten)]
    base_: crate::types::AnimationParameters,
    filename: Option<crate::types::FileName>,
    filenames: Option<crate::vec::Vec<crate::types::FileName>>,
    layers: Option<crate::vec::Vec<crate::types::Animation>>,
    lines_per_file: Option<u32>,
    // default: Value of `frame_count`
    slice: Option<u32>,
    stripes: Option<crate::vec::Vec<crate::types::Stripe>>,
}
