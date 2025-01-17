#[derive(serde::Deserialize)]
pub struct FrequencySizeRichness {
    frequency: crate::types::MapGenSize,
    richness: crate::types::MapGenSize,
    size: crate::types::MapGenSize,
}
