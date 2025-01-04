#[derive(Debug, Clone)]
pub enum Riding {
    Acceleration(Acceleration),
    Direction(Direction),
}
#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum Acceleration {
    Accelerating = 1,
    Braking = 2,
    Nothing = 0,
    Reversing = 3,
}
#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum Direction {
    Left = 0,
    Right = 2,
    Straight = 1,
}
