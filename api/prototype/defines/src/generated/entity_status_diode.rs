#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum EntityStatusDiode {
    Green = 0,
    Red = 1,
    Yellow = 2,
}
