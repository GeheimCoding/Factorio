#[derive(Debug, Clone)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value3 {
    LeftUndergroundLine,
    SecondaryLeftLine,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
            3 => Ok(TransportLine::Value3(std::collections::HashSet::from([
                Value3::LeftUndergroundLine,
                Value3::SecondaryLeftLine,
            ]))),
            4 => Ok(TransportLine::Value4(std::collections::HashSet::from([
                Value4::RightUndergroundLine,
                Value4::SecondaryRightLine,
            ]))),
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
