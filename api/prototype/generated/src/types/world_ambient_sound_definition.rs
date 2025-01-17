#[derive(serde::Deserialize)]
pub enum WorldAmbientSoundDefinition {
    #[serde(untagged)]
    WorldAmbientSoundDefinition {
        average_pause_seconds: f64,
        entity_to_sound_ratio: f32,
        max_entity_count: u32,
        min_entity_count: u32,
        radius: f64,
        sound: crate::types::Sound,
    },
    #[serde(untagged)]
    Sound(crate::types::Sound),
}
