#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum TrainState {
    ArriveSignal = 3,
    ArriveStation = 5,
    DestinationFull = 9,
    ManualControl = 8,
    ManualControlStop = 7,
    NoPath = 2,
    NoSchedule = 1,
    OnThePath = 0,
    WaitSignal = 4,
    WaitStation = 6,
}
