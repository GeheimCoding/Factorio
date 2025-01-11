pub struct ProcessionPrototype {
    ground_timeline: crate::types::ProcessionTimeline,
    procession_style: ProcessionPrototypeProcessionStyle,
    timeline: crate::types::ProcessionTimeline,
    usage: ProcessionPrototypeUsage,
}
pub enum ProcessionPrototypeProcessionStyle {
    U32(u32),
    VecU32(Vec<u32>),
}
pub enum ProcessionPrototypeUsage {
    Departure,
    Arrival,
    Intermezzo,
}
