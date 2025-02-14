#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum RocketSiloStatus {
    ArmsAdvance = 6,
    ArmsRetract = 10,
    BuildingRocket = 0,
    CreateRocket = 1,
    DoorsClosing = 13,
    DoorsOpened = 4,
    DoorsOpening = 3,
    EngineStarting = 9,
    LaunchStarted = 14,
    LaunchStarting = 8,
    LightsBlinkingClose = 12,
    LightsBlinkingOpen = 2,
    RocketFlying = 11,
    RocketReady = 7,
    RocketRising = 5,
}
