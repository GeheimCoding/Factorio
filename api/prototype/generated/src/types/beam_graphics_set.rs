#[derive(Debug, serde::Deserialize)]
pub struct BeamGraphicsSet {
    beam: Option<crate::types::BeamAnimationSet>,
    #[serde(default = "default_desired_segment_length")]
    desired_segment_length: f32,
    ground: Option<crate::types::BeamAnimationSet>,
    #[serde(default = "default_random_end_animation_rotation")]
    random_end_animation_rotation: bool,
    #[serde(default = "default_randomize_animation_per_segment")]
    randomize_animation_per_segment: bool,
    #[serde(default = "default_transparent_start_end_animations")]
    transparent_start_end_animations: bool,
}
fn default_desired_segment_length() -> f32 {
    1.0
}
fn default_random_end_animation_rotation() -> bool {
    true
}
fn default_randomize_animation_per_segment() -> bool {
    false
}
fn default_transparent_start_end_animations() -> bool {
    true
}
