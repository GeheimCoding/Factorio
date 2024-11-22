#[derive(Eq, Hash, PartialEq)]
pub enum Value0 {
    CharacterRequester,
    GenericOnOffBehavior,
    LogisticContainer,
    SpidertronRequester,
}
#[derive(Eq, Hash, PartialEq)]
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
            0 => {
                use std::collections::HashSet;
                let mut variants = vec![];
                variants.push(Value0::CharacterRequester);
                variants.push(Value0::GenericOnOffBehavior);
                variants.push(Value0::LogisticContainer);
                variants.push(Value0::SpidertronRequester);
                Ok(LogisticMemberIndex::Value0(HashSet::from_iter(variants)))
            }
            1 => {
                use std::collections::HashSet;
                let mut variants = vec![];
                variants.push(Value1::CharacterStorage);
                variants.push(Value1::VehicleStorage);
                Ok(LogisticMemberIndex::Value1(HashSet::from_iter(variants)))
            }
            2 => Ok(LogisticMemberIndex::CharacterProvider),
            other => Err(serde::de::Error::custom(format!(
                "unexpected value: {other}"
            ))),
        }
    }
}
pub enum LogisticMemberIndex {
    Value0(std::collections::HashSet<Value0>),
    Value1(std::collections::HashSet<Value1>),
    CharacterProvider,
}
