#[derive(Debug, serde::Deserialize)]
pub enum LayeredSound {
    #[serde(untagged)]
    LayeredSound {
        layers: crate::vec::Vec<crate::types::Sound>,
    },
    #[serde(untagged)]
    Sound(Box<crate::types::Sound>),
}
