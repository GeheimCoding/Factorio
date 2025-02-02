#[derive(Debug, serde::Deserialize)]
pub struct SegmentEngineSpecification {
    segments: crate::vec::Vec<crate::types::SegmentSpecification>,
}
