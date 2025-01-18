#[derive(serde::Deserialize)]
pub struct CraterPlacementDefinition {
    #[serde(default = "default_minimum_segments_to_place")]
    minimum_segments_to_place: u32,
    #[serde(default = "default_segment_probability")]
    segment_probability: f32,
    segments: Vec<CraterSegment>,
}
fn default_minimum_segments_to_place() -> u32 {
    1
}
fn default_segment_probability() -> f32 {
    1.0
}
#[derive(serde::Deserialize)]
pub struct CraterSegment {
    offset: crate::types::Vector,
    orientation: f32,
}
