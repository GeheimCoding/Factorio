#[derive(Debug, serde::Deserialize)]
pub struct FrequencySizeRichness {
    frequency: Option<crate::types::MapGenSize>,
    richness: Option<crate::types::MapGenSize>,
    size: Option<crate::types::MapGenSize>,
}
