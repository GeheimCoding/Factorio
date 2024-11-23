#[derive(Debug, Clone)]
pub enum LogisticMemberIndex {
    Value0(std::collections::HashSet<Value0>),
    Value1(std::collections::HashSet<Value1>),
    CharacterProvider,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value0 {
    CharacterRequester,
    GenericOnOffBehavior,
    LogisticContainer,
    SpidertronRequester,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value1 {
    CharacterStorage,
    VehicleStorage,
}
impl<'de> serde::Deserialize<'de> for LogisticMemberIndex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match <u16 as serde::Deserialize>::deserialize(deserializer)? {
            0 => Ok(LogisticMemberIndex::Value0(
                std::collections::HashSet::from([
                    Value0::CharacterRequester,
                    Value0::GenericOnOffBehavior,
                    Value0::LogisticContainer,
                    Value0::SpidertronRequester,
                ]),
            )),
            1 => Ok(LogisticMemberIndex::Value1(
                std::collections::HashSet::from([Value1::CharacterStorage, Value1::VehicleStorage]),
            )),
            2 => Ok(LogisticMemberIndex::CharacterProvider),
            other => Err(serde::de::Error::custom(format!(
                "unexpected value: {other}"
            ))),
        }
    }
}
