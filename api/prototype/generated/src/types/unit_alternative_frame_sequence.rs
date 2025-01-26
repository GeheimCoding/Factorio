#[derive(Debug, serde::Deserialize)]
pub struct UnitAlternativeFrameSequence {
    attacking_animation_speed: f32,
    attacking_frame_sequence: Vec<u16>,
    back_to_walk_animation_speed: f32,
    back_to_walk_frame_sequence: Vec<u16>,
    cooldown_animation_speed: f32,
    cooldown_frame_sequence: Vec<u16>,
    prepared_animation_speed: f32,
    prepared_frame_sequence: Vec<u16>,
    #[serde(rename = "warmup2_frame_sequence")]
    warmup_2_frame_sequence: Vec<u16>,
    warmup_animation_speed: f32,
    warmup_frame_sequence: Vec<u16>,
}
