#[derive(Debug, Clone)]
pub enum WireConnectorId {
    Value1(std::collections::HashSet<Value1>),
    Value2(std::collections::HashSet<Value2>),
    CombinatorOutputRed,
    CombinatorOutputGreen,
    Value5(std::collections::HashSet<Value5>),
    PowerSwitchRightCopper,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value1 {
    CircuitRed,
    CombinatorInputRed,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value2 {
    CircuitGreen,
    CombinatorInputGreen,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value5 {
    PoleCopper,
    PowerSwitchLeftCopper,
}
impl<'de> serde::Deserialize<'de> for WireConnectorId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match <u16 as serde::Deserialize>::deserialize(deserializer)? {
            1 => Ok(WireConnectorId::Value1(std::collections::HashSet::from([
                Value1::CircuitRed,
                Value1::CombinatorInputRed,
            ]))),
            2 => Ok(WireConnectorId::Value2(std::collections::HashSet::from([
                Value2::CircuitGreen,
                Value2::CombinatorInputGreen,
            ]))),
            3 => Ok(WireConnectorId::CombinatorOutputRed),
            4 => Ok(WireConnectorId::CombinatorOutputGreen),
            5 => Ok(WireConnectorId::Value5(std::collections::HashSet::from([
                Value5::PoleCopper,
                Value5::PowerSwitchLeftCopper,
            ]))),
            6 => Ok(WireConnectorId::PowerSwitchRightCopper),
            other => Err(serde::de::Error::custom(format!(
                "unexpected value: {other}"
            ))),
        }
    }
}
