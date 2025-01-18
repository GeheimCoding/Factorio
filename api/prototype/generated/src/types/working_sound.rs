#[derive(serde::Deserialize)]
pub enum WorkingSound {
    #[serde(untagged)]
    WorkingSound {
        base_: crate::types::MainSound,
        activate_sound: crate::types::Sound,
        #[serde(default = "default_apparent_volume")]
        apparent_volume: f32,
        #[serde(default = "default_audible_distance_modifier")]
        audible_distance_modifier: f64,
        deactivate_sound: crate::types::Sound,
        #[serde(default = "default_extra_sounds_ignore_limit")]
        extra_sounds_ignore_limit: bool,
        idle_sound: crate::types::Sound,
        main_sounds: WorkingSoundMainSounds,
        max_sounds_per_type: u8,
        #[serde(default = "default_persistent")]
        persistent: bool,
        sound_accents: WorkingSoundSoundAccents,
        #[serde(default = "default_use_doppler_shift")]
        use_doppler_shift: bool,
    },
    #[serde(untagged)]
    Sound(crate::types::Sound),
}
fn default_apparent_volume() -> f32 {
    1.0
}
fn default_audible_distance_modifier() -> f64 {
    1.0
}
fn default_extra_sounds_ignore_limit() -> bool {
    false
}
#[derive(serde::Deserialize)]
pub enum WorkingSoundMainSounds {
    #[serde(untagged)]
    MainSound(Box<crate::types::MainSound>),
    #[serde(untagged)]
    VecMainSound(Vec<crate::types::MainSound>),
}
fn default_persistent() -> bool {
    false
}
#[derive(serde::Deserialize)]
pub enum WorkingSoundSoundAccents {
    #[serde(untagged)]
    SoundAccent(Box<crate::types::SoundAccent>),
    #[serde(untagged)]
    VecSoundAccent(Vec<crate::types::SoundAccent>),
}
fn default_use_doppler_shift() -> bool {
    true
}
