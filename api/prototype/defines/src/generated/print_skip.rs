#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum PrintSkip {
    IfRedundant = 1,
    IfVisible = 2,
    Never = 0,
}
