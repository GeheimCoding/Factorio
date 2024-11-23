#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum BuildMode {
    Forced = 1,
    Normal = 0,
    Superforced = 2,
}
