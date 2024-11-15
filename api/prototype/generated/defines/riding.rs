pub enum Direction {
    Left,
    Right,
    Straight,
}
pub enum Acceleration {
    Accelerating,
    Braking,
    Nothing,
    Reversing,
}
pub enum Riding {
    Acceleration(Acceleration),
    Direction(Direction),
}
