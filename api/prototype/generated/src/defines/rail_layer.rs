#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum RailLayer {
    Elevated = 1,
    Ground = 0,
}
