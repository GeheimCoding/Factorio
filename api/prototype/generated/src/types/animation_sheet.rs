#[derive(serde::Deserialize)]
pub struct AnimationSheet {
    base_: crate::types::AnimationParameters,
    filename: crate::types::FileName,
    filenames: Vec<crate::types::FileName>,
    // default: Value of `variation_count`
    line_length: u32,
    lines_per_file: u32,
    variation_count: u32,
}
