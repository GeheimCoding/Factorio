#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum CargoDestination {
    Invalid = 0,
    Orbit = 5,
    SpacePlatform = 2,
    Station = 1,
    Surface = 3,
}
