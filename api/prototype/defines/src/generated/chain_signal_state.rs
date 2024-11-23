#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum ChainSignalState {
    AllOpen = 1,
    None = 0,
    NoneOpen = 3,
    PartiallyOpen = 2,
}
