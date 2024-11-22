#[derive(Eq, Hash, PartialEq)]
pub enum Value3 {
    LeftUndergroundLine,
    SecondaryLeftLine,
}
#[derive(Eq, Hash, PartialEq)]
pub enum Value4 {
    RightUndergroundLine,
    SecondaryRightLine,
}
impl<'de> serde::Deserialize<'de> for TransportLine {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match <u16 as serde::Deserialize>::deserialize(deserializer)? {
            1 => Ok(TransportLine::LeftLine),
            2 => Ok(TransportLine::RightLine),
            3 => {
                use std::collections::HashSet;
                let mut variants = vec![];
                variants.push(Value3::LeftUndergroundLine);
                variants.push(Value3::SecondaryLeftLine);
                Ok(TransportLine::Value3(HashSet::from_iter(variants)))
            }
            4 => {
                use std::collections::HashSet;
                let mut variants = vec![];
                variants.push(Value4::RightUndergroundLine);
                variants.push(Value4::SecondaryRightLine);
                Ok(TransportLine::Value4(HashSet::from_iter(variants)))
            }
            5 => Ok(TransportLine::LeftSplitLine),
            6 => Ok(TransportLine::RightSplitLine),
            7 => Ok(TransportLine::SecondaryLeftSplitLine),
            8 => Ok(TransportLine::SecondaryRightSplitLine),
            other => Err(serde::de::Error::custom(format!(
                "unexpected value: {other}"
            ))),
        }
    }
}
pub enum TransportLine {
    LeftLine,
    RightLine,
    Value3(std::collections::HashSet<Value3>),
    Value4(std::collections::HashSet<Value4>),
    LeftSplitLine,
    RightSplitLine,
    SecondaryLeftSplitLine,
    SecondaryRightSplitLine,
}
