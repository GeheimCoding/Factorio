#[derive(serde::Deserialize)]
pub struct ProcessionPrototype {
    base_: crate::prototypes::Prototype,
    ground_timeline: crate::types::ProcessionTimeline,
    procession_style: ProcessionPrototypeProcessionStyle,
    timeline: crate::types::ProcessionTimeline,
    usage: ProcessionPrototypeUsage,
}
#[derive(serde::Deserialize)]
pub enum ProcessionPrototypeProcessionStyle {
    #[serde(untagged)]
    U32(u32),
    #[serde(untagged)]
    VecU32(Vec<u32>),
}
#[derive(serde::Deserialize)]
pub enum ProcessionPrototypeUsage {
    #[serde(rename = "departure")]
    Departure,
    #[serde(rename = "arrival")]
    Arrival,
    #[serde(rename = "intermezzo")]
    Intermezzo,
}
