#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum FlowPrecisionIndex {
    FiftyHours = 5,
    FiveSeconds = 0,
    OneHour = 3,
    OneMinute = 1,
    OneThousandHours = 7,
    TenHours = 4,
    TenMinutes = 2,
    TwoHundredFiftyHours = 6,
}
