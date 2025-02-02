#[derive(Debug, serde::Deserialize)]
pub struct ProcessionPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::Prototype,
    ground_timeline: Option<crate::types::ProcessionTimeline>,
    procession_style: ProcessionPrototypeProcessionStyle,
    timeline: crate::types::ProcessionTimeline,
    usage: ProcessionPrototypeUsage,
}
#[derive(Debug, serde::Deserialize)]
pub enum ProcessionPrototypeProcessionStyle {
    #[serde(untagged)]
    U32(u32),
    #[serde(untagged)]
    VecU32(crate::vec::Vec<u32>),
}
#[derive(Debug, serde::Deserialize)]
pub enum ProcessionPrototypeUsage {
    #[serde(rename = "departure")]
    Departure,
    #[serde(rename = "arrival")]
    Arrival,
    #[serde(rename = "intermezzo")]
    Intermezzo,
}
