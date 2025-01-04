#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum MovingState {
    Adaptive = 2,
    Moving = 1,
    Stale = 0,
    Stuck = 3,
}
