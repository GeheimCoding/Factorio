#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum CompoundCommand {
    LogicalAnd = 0,
    LogicalOr = 1,
    ReturnLast = 2,
}
