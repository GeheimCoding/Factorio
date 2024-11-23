#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum SelectionMode {
    AltReverseSelect = 5,
    AltSelect = 2,
    ReverseSelect = 4,
    Select = 1,
}
