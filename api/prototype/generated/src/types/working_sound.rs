#[derive(Debug, serde::Deserialize)]
pub enum WorkingSound {
    #[serde(untagged)]
    WorkingSound {
        #[serde(flatten)]
        base_: crate::types::MainSound,
        activate_sound: Option<Box<crate::types::Sound>>,
        deactivate_sound: Option<Box<crate::types::Sound>>,
        #[serde(default = "default_extra_sounds_ignore_limit")]
        extra_sounds_ignore_limit: bool,
        idle_sound: Option<Box<crate::types::Sound>>,
        main_sounds: Option<WorkingSoundMainSounds>,
        max_sounds_per_prototype: Option<u8>,
        #[serde(default = "default_persistent")]
        persistent: bool,
        sound_accents: Option<WorkingSoundSoundAccents>,
        #[serde(default = "default_use_doppler_shift")]
        use_doppler_shift: bool,
    },
    #[serde(untagged)]
    Sound(Box<crate::types::Sound>),
}
fn default_extra_sounds_ignore_limit() -> bool {
    false
}
#[derive(Debug, serde::Deserialize)]
pub enum WorkingSoundMainSounds {
    #[serde(untagged)]
    MainSound(Box<crate::types::MainSound>),
    #[serde(untagged)]
    VecMainSound(crate::vec::Vec<crate::types::MainSound>),
}
fn default_persistent() -> bool {
    false
}
#[derive(Debug, serde::Deserialize)]
pub enum WorkingSoundSoundAccents {
    #[serde(untagged)]
    SoundAccent(Box<crate::types::SoundAccent>),
    #[serde(untagged)]
    VecSoundAccent(crate::vec::Vec<crate::types::SoundAccent>),
}
fn default_use_doppler_shift() -> bool {
    true
}
