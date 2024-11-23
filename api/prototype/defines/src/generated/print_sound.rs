#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum PrintSound {
    Always = 1,
    Never = 0,
    UsePlayerSettings = 2,
}
