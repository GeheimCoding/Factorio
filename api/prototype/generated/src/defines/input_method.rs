#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum InputMethod {
    GameController = 1,
    KeyboardAndMouse = 0,
}
