pub enum Sound {
    Sound {
        advanced_volume_control: AdvancedVolumeControl,
        aggregation: AggregationSpecification,
        allow_random_repeat: bool,
        audible_distance_modifier: f64,
        category: SoundType,
        filename: FileName,
        game_controller_vibration_data: GameControllerVibrationData,
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
    VecSoundDefinition(Vec<SoundDefinition>),
}
pub enum SoundModifiers {
    SoundModifier(SoundModifier),
    VecSoundModifier(Vec<SoundModifier>),
}
pub enum SoundVariations {
    SoundDefinition(SoundDefinition),
    VecSoundDefinition(Vec<SoundDefinition>),
}
