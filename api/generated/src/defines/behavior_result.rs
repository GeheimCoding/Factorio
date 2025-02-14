#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum BehaviorResult {
    Deleted = 3,
    Fail = 1,
    InProgress = 0,
    Success = 2,
}
