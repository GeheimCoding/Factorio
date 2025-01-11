pub struct SoundPrototype {
    advanced_volume_control: crate::types::AdvancedVolumeControl,
    aggregation: crate::types::AggregationSpecification,
    allow_random_repeat: bool,
    audible_distance_modifier: f64,
    category: crate::types::SoundType,
    filename: crate::types::FileName,
    game_controller_vibration_data: crate::types::GameControllerVibrationData,
    max_speed: f32,
    max_volume: f32,
    min_speed: f32,
    min_volume: f32,
    modifiers: SoundPrototypeModifiers,
    name: String,
    preload: bool,
    priority: u8,
    speed: f32,
    speed_smoothing_window_size: u32,
    type_: String,
    variations: SoundPrototypeVariations,
    volume: f32,
}
pub enum SoundPrototypeModifiers {
    SoundModifier(Box<crate::types::SoundModifier>),
    VecSoundModifier(Vec<crate::types::SoundModifier>),
}
pub enum SoundPrototypeVariations {
    SoundDefinition(crate::types::SoundDefinition),
    VecSoundDefinition(Vec<crate::types::SoundDefinition>),
}
