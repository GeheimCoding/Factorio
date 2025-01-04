#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum SpacePlatformState {
    NoPath = 5,
    NoSchedule = 4,
    OnThePath = 2,
    Paused = 7,
    StarterPackOnTheWay = 1,
    StarterPackRequested = 8,
    WaitingAtStation = 6,
    WaitingForDeparture = 3,
    WaitingForStarterPack = 0,
}
