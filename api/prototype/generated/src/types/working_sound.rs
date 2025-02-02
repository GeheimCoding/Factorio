#[derive(Debug, serde::Deserialize)]
pub enum WorkingSound {
    #[serde(untagged)]
    WorkingSound {
        #[serde(flatten)]
        base_: crate::types::MainSound,
        activate_sound: Option<crate::types::Sound>,
        #[serde(default = "default_apparent_volume")]
        apparent_volume: f32,
        #[serde(default = "default_audible_distance_modifier")]
        audible_distance_modifier: f64,
        deactivate_sound: Option<crate::types::Sound>,
        #[serde(default = "default_extra_sounds_ignore_limit")]
        extra_sounds_ignore_limit: bool,
        idle_sound: Option<crate::types::Sound>,
        main_sounds: Option<WorkingSoundMainSounds>,
        max_sounds_per_type: Option<u8>,
        #[serde(default = "default_persistent")]
        persistent: bool,
        sound_accents: Option<WorkingSoundSoundAccents>,
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
#[derive(Debug, serde::Deserialize)]
pub enum WorkingSoundMainSounds {
    #[serde(untagged)]
    MainSound(Box<crate::types::MainSound>),
    #[serde(untagged)]
    VecMainSound(Vec<crate::types::MainSound>),
}
fn default_persistent() -> bool {
    false
}
#[derive(Debug, serde::Deserialize)]
pub enum WorkingSoundSoundAccents {
    #[serde(untagged)]
    SoundAccent(Box<crate::types::SoundAccent>),
    #[serde(untagged)]
    VecSoundAccent(Vec<crate::types::SoundAccent>),
}
fn default_use_doppler_shift() -> bool {
    true
}
