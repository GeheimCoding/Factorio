#[derive(serde::Deserialize)]
pub struct BeamGraphicsSet {
    beam: crate::types::BeamAnimationSet,
    desired_segment_length: f32,
    ground: crate::types::BeamAnimationSet,
    random_end_animation_rotation: bool,
    randomize_animation_per_segment: bool,
    transparent_start_end_animations: bool,
}
