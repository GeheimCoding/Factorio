#[derive(Debug, serde::Deserialize)]
pub enum LayeredSound {
    #[serde(untagged)]
    LayeredSound { layers: Vec<crate::types::Sound> },
    #[serde(untagged)]
    Sound(crate::types::Sound),
}
