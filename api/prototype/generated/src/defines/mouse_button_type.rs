#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum MouseButtonType {
    Left = 2,
    Middle = 8,
    None = 1,
    Right = 4,
}
