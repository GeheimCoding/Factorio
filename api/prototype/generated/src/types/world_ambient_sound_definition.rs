#[derive(serde::Deserialize)]
pub enum WorldAmbientSoundDefinition {
    #[serde(untagged)]
    WorldAmbientSoundDefinition {
        #[serde(default = "default_average_pause_seconds")]
        average_pause_seconds: f64,
        #[serde(default = "default_entity_to_sound_ratio")]
        entity_to_sound_ratio: f32,
        #[serde(default = "default_max_entity_count")]
        max_entity_count: u32,
        #[serde(default = "default_min_entity_count")]
        min_entity_count: u32,
        #[serde(default = "default_radius")]
        radius: f64,
        sound: Option<crate::types::Sound>,
    },
    #[serde(untagged)]
    Sound(crate::types::Sound),
}
fn default_average_pause_seconds() -> f64 {
    0.0
}
fn default_entity_to_sound_ratio() -> f32 {
    0.2
}
fn default_max_entity_count() -> u32 {
    15
}
fn default_min_entity_count() -> u32 {
    5
}
fn default_radius() -> f64 {
    10.0
}
