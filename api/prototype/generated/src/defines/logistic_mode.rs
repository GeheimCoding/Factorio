#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum LogisticMode {
    ActiveProvider = 1,
    Buffer = 5,
    None = 0,
    PassiveProvider = 4,
    Requester = 3,
    Storage = 2,
}
