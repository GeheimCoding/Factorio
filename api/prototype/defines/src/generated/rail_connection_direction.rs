#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum RailConnectionDirection {
    Left = 0,
    None = 3,
    Right = 2,
    Straight = 1,
}
