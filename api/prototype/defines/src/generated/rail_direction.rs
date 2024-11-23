#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum RailDirection {
    Back = 1,
    Front = 0,
}
