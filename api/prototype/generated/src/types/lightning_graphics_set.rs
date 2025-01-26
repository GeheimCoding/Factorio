#[derive(Debug, serde::Deserialize)]
pub struct LightningGraphicsSet {
    attractor_hit_animation: Option<crate::types::Animation>,
    #[serde(default = "default_bolt_detail_level")]
    bolt_detail_level: u8,
    #[serde(default = "default_bolt_half_width")]
    bolt_half_width: f32,
    #[serde(default = "default_bolt_midpoint_variance")]
    bolt_midpoint_variance: f32,
    cloud_background: Option<crate::types::Animation>,
    #[serde(default = "default_cloud_detail_level")]
    cloud_detail_level: u8,
    #[serde(default = "default_cloud_fork_orientation_variance")]
    cloud_fork_orientation_variance: f32,
    #[serde(default = "default_cloud_forks")]
    cloud_forks: u8,
    explosion: Option<crate::types::AnimationVariations>,
    #[serde(default = "default_fork_intensity_multiplier")]
    fork_intensity_multiplier: f32,
    #[serde(default = "default_fork_orientation_variance")]
    fork_orientation_variance: f32,
    #[serde(default = "default_ground_streamer_variance")]
    ground_streamer_variance: f32,
    ground_streamers: Option<Vec<crate::types::Animation>>,
    light: Option<crate::types::LightDefinition>,
    #[serde(default = "default_max_bolt_offset")]
    max_bolt_offset: f32,
    #[serde(default = "default_max_fork_probability")]
    max_fork_probability: f32,
    #[serde(default = "default_max_ground_streamer_distance")]
    max_ground_streamer_distance: f32,
    #[serde(default = "default_max_relative_fork_length")]
    max_relative_fork_length: f32,
    #[serde(default = "default_min_ground_streamer_distance")]
    min_ground_streamer_distance: f32,
    #[serde(default = "default_min_relative_fork_length")]
    min_relative_fork_length: f32,
    #[serde(default = "default_relative_cloud_fork_length")]
    relative_cloud_fork_length: f32,
    shader_configuration: Option<Vec<LightningShaderConfiguration>>,
}
fn default_bolt_detail_level() -> u8 {
    6
}
fn default_bolt_half_width() -> f32 {
    0.0
}
fn default_bolt_midpoint_variance() -> f32 {
    0.1
}
fn default_cloud_detail_level() -> u8 {
    3
}
fn default_cloud_fork_orientation_variance() -> f32 {
    0.0
}
fn default_cloud_forks() -> u8 {
    5
}
fn default_fork_intensity_multiplier() -> f32 {
    0.7
}
fn default_fork_orientation_variance() -> f32 {
    0.1
}
fn default_ground_streamer_variance() -> f32 {
    1.0
}
fn default_max_bolt_offset() -> f32 {
    0.3
}
fn default_max_fork_probability() -> f32 {
    0.9
}
fn default_max_ground_streamer_distance() -> f32 {
    4.0
}
fn default_max_relative_fork_length() -> f32 {
    0.7
}
fn default_min_ground_streamer_distance() -> f32 {
    2.0
}
fn default_min_relative_fork_length() -> f32 {
    0.5
}
fn default_relative_cloud_fork_length() -> f32 {
    0.2
}
#[derive(Debug, serde::Deserialize)]
pub struct LightningShaderConfiguration {
    color: crate::types::Color,
    distortion: f32,
    power: f32,
    thickness: f32,
}
