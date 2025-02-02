#[derive(Debug, serde::Deserialize)]
pub struct SoundPrototype {
    advanced_volume_control: Option<crate::types::AdvancedVolumeControl>,
    aggregation: Option<crate::types::AggregationSpecification>,
    #[serde(default = "default_allow_random_repeat")]
    allow_random_repeat: bool,
    #[serde(default = "default_audible_distance_modifier")]
    audible_distance_modifier: f64,
    #[serde(default = "default_category")]
    category: crate::types::SoundType,
    filename: Option<crate::types::FileName>,
    game_controller_vibration_data: Option<crate::types::GameControllerVibrationData>,
    #[serde(default = "default_max_speed")]
    max_speed: f32,
    #[serde(default = "default_max_volume")]
    max_volume: f32,
    #[serde(default = "default_min_speed")]
    min_speed: f32,
    #[serde(default = "default_min_volume")]
    min_volume: f32,
    modifiers: Option<SoundPrototypeModifiers>,
    name: String,
    preload: Option<bool>,
    #[serde(default = "default_priority")]
    priority: u8,
    #[serde(default = "default_speed")]
    speed: f32,
    #[serde(default = "default_speed_smoothing_window_size")]
    speed_smoothing_window_size: u32,
    variations: Option<SoundPrototypeVariations>,
    #[serde(default = "default_volume")]
    volume: f32,
}
fn default_allow_random_repeat() -> bool {
    false
}
fn default_audible_distance_modifier() -> f64 {
    1.0
}
fn default_category() -> crate::types::SoundType {
    crate::types::SoundType::GameEffect
}
fn default_max_speed() -> f32 {
    1.0
}
fn default_max_volume() -> f32 {
    1.0
}
fn default_min_speed() -> f32 {
    1.0
}
fn default_min_volume() -> f32 {
    1.0
}
#[derive(Debug, serde::Deserialize)]
pub enum SoundPrototypeModifiers {
    #[serde(untagged)]
    SoundModifier(Box<crate::types::SoundModifier>),
    #[serde(untagged)]
    VecSoundModifier(crate::vec::Vec<crate::types::SoundModifier>),
}
fn default_priority() -> u8 {
    127
}
fn default_speed() -> f32 {
    1.0
}
fn default_speed_smoothing_window_size() -> u32 {
    0
}
#[derive(Debug, serde::Deserialize)]
pub enum SoundPrototypeVariations {
    #[serde(untagged)]
    SoundDefinition(Box<crate::types::SoundDefinition>),
    #[serde(untagged)]
    VecSoundDefinition(crate::vec::Vec<crate::types::SoundDefinition>),
}
fn default_volume() -> f32 {
    1.0
}
