#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum WireType {
    Copper = 1,
    Green = 3,
    Red = 2,
}
