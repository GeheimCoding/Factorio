pub struct CraterPlacementDefinition {
    minimum_segments_to_place: u32,
    segment_probability: f32,
    segments: Vec<CraterSegment>,
}
