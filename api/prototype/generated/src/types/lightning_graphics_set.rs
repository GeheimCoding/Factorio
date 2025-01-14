pub struct LightningGraphicsSet {
    attractor_hit_animation: crate::types::Animation,
    bolt_detail_level: u8,
    bolt_half_width: f32,
    bolt_midpoint_variance: f32,
    cloud_background: crate::types::Animation,
    cloud_detail_level: u8,
    cloud_fork_orientation_variance: f32,
    cloud_forks: u8,
    explosion: crate::types::AnimationVariations,
    fork_intensity_multiplier: f32,
    fork_orientation_variance: f32,
    ground_streamer_variance: f32,
    ground_streamers: Vec<crate::types::Animation>,
    light: crate::types::LightDefinition,
    max_bolt_offset: f32,
    max_fork_probability: f32,
    max_ground_streamer_distance: f32,
    max_relative_fork_length: f32,
    min_ground_streamer_distance: f32,
    min_relative_fork_length: f32,
    relative_cloud_fork_length: f32,
    shader_configuration: Vec<LightningShaderConfiguration>,
}
pub struct LightningShaderConfiguration {
    color: crate::types::Color,
    distortion: f32,
    power: f32,
    thickness: f32,
}
