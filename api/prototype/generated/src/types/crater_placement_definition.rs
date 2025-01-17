#[derive(serde::Deserialize)]
pub struct CraterPlacementDefinition {
    minimum_segments_to_place: u32,
    segment_probability: f32,
    segments: Vec<CraterSegment>,
}
#[derive(serde::Deserialize)]
pub struct CraterSegment {
    offset: crate::types::Vector,
    orientation: f32,
}
