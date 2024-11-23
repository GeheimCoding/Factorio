#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum SignalState {
    Closed = 1,
    Open = 0,
    Reserved = 2,
    ReservedByCircuitNetwork = 3,
}
