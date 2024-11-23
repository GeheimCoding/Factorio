#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum Difficulty {
    Easy = 0,
    Hard = 2,
    Normal = 1,
}
