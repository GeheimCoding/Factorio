#[derive(Debug, serde::Deserialize)]
pub enum Sound {
    #[serde(untagged)]
    Sound {
        advanced_volume_control: Option<Box<crate::types::AdvancedVolumeControl>>,
        aggregation: Option<Box<crate::types::AggregationSpecification>>,
        #[serde(default = "default_allow_random_repeat")]
        allow_random_repeat: bool,
        #[serde(default = "default_audible_distance_modifier")]
        audible_distance_modifier: f64,
        category: Option<crate::types::SoundType>,
        filename: Option<crate::types::FileName>,
        game_controller_vibration_data: Option<Box<crate::types::GameControllerVibrationData>>,
        #[serde(default = "default_max_speed")]
        max_speed: f32,
        #[serde(default = "default_max_volume")]
        max_volume: f32,
        #[serde(default = "default_min_speed")]
        min_speed: f32,
        #[serde(default = "default_min_volume")]
        min_volume: f32,
        modifiers: Option<SoundModifiers>,
        preload: Option<bool>,
        #[serde(default = "default_priority")]
        priority: u8,
        #[serde(default = "default_speed")]
        speed: f32,
        #[serde(default = "default_speed_smoothing_window_size")]
        speed_smoothing_window_size: u32,
        variations: Option<SoundVariations>,
        #[serde(default = "default_volume")]
        volume: f32,
    },
    #[serde(untagged)]
    VecSoundDefinition(Vec<crate::types::SoundDefinition>),
    #[serde(untagged)]
    FileName(crate::types::FileName),
}
fn default_allow_random_repeat() -> bool {
    false
}
fn default_audible_distance_modifier() -> f64 {
    1.0
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
pub enum SoundModifiers {
    #[serde(untagged)]
    SoundModifier(Box<crate::types::SoundModifier>),
    #[serde(untagged)]
    VecSoundModifier(Vec<crate::types::SoundModifier>),
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
pub enum SoundVariations {
    #[serde(untagged)]
    SoundDefinition(Box<crate::types::SoundDefinition>),
    #[serde(untagged)]
    VecSoundDefinition(Vec<crate::types::SoundDefinition>),
}
fn default_volume() -> f32 {
    1.0
}
