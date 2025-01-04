pub enum LayeredSound {
    LayeredSound { layers: Vec<crate::types::Sound> },
    Sound(crate::types::Sound),
}
