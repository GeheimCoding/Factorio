pub enum LayeredSound {
    LayeredSound { layers: Vec<Sound> },
    Sound(Sound),
}
