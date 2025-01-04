pub enum Sound {
    Sound {
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
        modifiers: SoundModifiers,
        preload: bool,
        priority: u8,
        speed: f32,
        speed_smoothing_window_size: u32,
        variations: SoundVariations,
        volume: f32,
    },
    VecSoundDefinition(Vec<crate::types::SoundDefinition>),
}
pub enum SoundModifiers {
    SoundModifier(Box<crate::types::SoundModifier>),
    VecSoundModifier(Vec<crate::types::SoundModifier>),
}
pub enum SoundVariations {
    SoundDefinition(crate::types::SoundDefinition),
    VecSoundDefinition(Vec<crate::types::SoundDefinition>),
}
