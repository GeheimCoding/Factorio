#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum WireOrigin {
    Player = 1,
    Radars = 3,
    Script = 2,
}
