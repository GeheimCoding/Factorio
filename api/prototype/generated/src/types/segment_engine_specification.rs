#[derive(Debug, serde::Deserialize)]
pub struct SegmentEngineSpecification {
    segments: Vec<crate::types::SegmentSpecification>,
}
