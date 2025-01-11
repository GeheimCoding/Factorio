pub enum WorkingSound {
    WorkingSound {
        base_: crate::types::MainSound,
        activate_sound: crate::types::Sound,
        apparent_volume: f32,
        audible_distance_modifier: f64,
        deactivate_sound: crate::types::Sound,
        extra_sounds_ignore_limit: bool,
        idle_sound: crate::types::Sound,
        main_sounds: WorkingSoundMainSounds,
        max_sounds_per_type: u8,
        persistent: bool,
        sound_accents: WorkingSoundSoundAccents,
        use_doppler_shift: bool,
    },
    Sound(crate::types::Sound),
}
pub enum WorkingSoundMainSounds {
    MainSound(Box<crate::types::MainSound>),
    VecMainSound(Vec<crate::types::MainSound>),
}
pub enum WorkingSoundSoundAccents {
    SoundAccent(Box<crate::types::SoundAccent>),
    VecSoundAccent(Vec<crate::types::SoundAccent>),
}
