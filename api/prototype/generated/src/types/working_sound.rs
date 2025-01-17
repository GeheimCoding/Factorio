#[derive(serde::Deserialize)]
pub enum WorkingSound {
    #[serde(untagged)]
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
    #[serde(untagged)]
    Sound(crate::types::Sound),
}
#[derive(serde::Deserialize)]
pub enum WorkingSoundMainSounds {
    #[serde(untagged)]
    MainSound(Box<crate::types::MainSound>),
    #[serde(untagged)]
    VecMainSound(Vec<crate::types::MainSound>),
}
#[derive(serde::Deserialize)]
pub enum WorkingSoundSoundAccents {
    #[serde(untagged)]
    SoundAccent(Box<crate::types::SoundAccent>),
    #[serde(untagged)]
    VecSoundAccent(Vec<crate::types::SoundAccent>),
}
