pub enum WorkingSound {
    WorkingSound {
        activate_sound: Sound,
        apparent_volume: f32,
        audible_distance_modifier: f64,
        deactivate_sound: Sound,
        extra_sounds_ignore_limit: bool,
        idle_sound: Sound,
        main_sounds: WorkingSoundMainSounds,
        max_sounds_per_type: u8,
        persistent: bool,
        sound_accents: WorkingSoundSoundAccents,
        use_doppler_shift: bool,
    },
    Sound(Sound),
}
pub enum WorkingSoundMainSounds {
    MainSound(Box<MainSound>),
    VecMainSound(Vec<MainSound>),
}
pub enum WorkingSoundSoundAccents {
    SoundAccent(Box<SoundAccent>),
    VecSoundAccent(Vec<SoundAccent>),
}
