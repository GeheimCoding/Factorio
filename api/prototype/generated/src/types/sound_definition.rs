#[derive(serde::Deserialize)]
pub enum SoundDefinition {
    #[serde(untagged)]
    SoundDefinition {
        filename: crate::types::FileName,
        #[serde(default = "default_max_speed")]
        max_speed: f32,
        #[serde(default = "default_max_volume")]
        max_volume: f32,
        #[serde(default = "default_min_speed")]
        min_speed: f32,
        #[serde(default = "default_min_volume")]
        min_volume: f32,
        modifiers: Option<SoundDefinitionModifiers>,
        preload: Option<bool>,
        #[serde(default = "default_speed")]
        speed: f32,
        #[serde(default = "default_volume")]
        volume: f32,
    },
    #[serde(untagged)]
    FileName(crate::types::FileName),
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
#[derive(serde::Deserialize)]
pub enum SoundDefinitionModifiers {
    #[serde(untagged)]
    SoundModifier(Box<crate::types::SoundModifier>),
    #[serde(untagged)]
    VecSoundModifier(Vec<crate::types::SoundModifier>),
}
fn default_speed() -> f32 {
    1.0
}
fn default_volume() -> f32 {
    1.0
}
