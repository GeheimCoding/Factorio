pub enum TrainState {
    ArriveSignal,
    ArriveStation,
    DestinationFull,
    ManualControl,
    ManualControlStop,
    NoPath,
    NoSchedule,
    OnThePath,
    WaitSignal,
    WaitStation,
}
