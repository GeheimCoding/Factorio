#[derive(Debug, serde::Deserialize)]
pub struct AnimationSheet {
    #[serde(flatten)]
    base_: crate::types::AnimationParameters,
    filename: Option<crate::types::FileName>,
    filenames: Option<Vec<crate::types::FileName>>,
    // default: Value of `variation_count`
    line_length: Option<u32>,
    lines_per_file: Option<u32>,
    variation_count: u32,
}
