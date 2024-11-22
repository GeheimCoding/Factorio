#[derive(Eq, Hash, PartialEq)]
pub enum Value1 {
    CircuitRed,
    CombinatorInputRed,
}
#[derive(Eq, Hash, PartialEq)]
pub enum Value2 {
    CircuitGreen,
    CombinatorInputGreen,
}
#[derive(Eq, Hash, PartialEq)]
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
            1 => {
                use std::collections::HashSet;
                let mut variants = vec![];
                variants.push(Value1::CircuitRed);
                variants.push(Value1::CombinatorInputRed);
                Ok(WireConnectorId::Value1(HashSet::from_iter(variants)))
            }
            2 => {
                use std::collections::HashSet;
                let mut variants = vec![];
                variants.push(Value2::CircuitGreen);
                variants.push(Value2::CombinatorInputGreen);
                Ok(WireConnectorId::Value2(HashSet::from_iter(variants)))
            }
            3 => Ok(WireConnectorId::CombinatorOutputRed),
            4 => Ok(WireConnectorId::CombinatorOutputGreen),
            5 => {
                use std::collections::HashSet;
                let mut variants = vec![];
                variants.push(Value5::PoleCopper);
                variants.push(Value5::PowerSwitchLeftCopper);
                Ok(WireConnectorId::Value5(HashSet::from_iter(variants)))
            }
            6 => Ok(WireConnectorId::PowerSwitchRightCopper),
            other => Err(serde::de::Error::custom(format!(
                "unexpected value: {other}"
            ))),
        }
    }
}
pub enum WireConnectorId {
    Value1(std::collections::HashSet<Value1>),
    Value2(std::collections::HashSet<Value2>),
    CombinatorOutputRed,
    CombinatorOutputGreen,
    Value5(std::collections::HashSet<Value5>),
    PowerSwitchRightCopper,
}
