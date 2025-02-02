#[derive(Debug, serde::Deserialize)]
pub struct ShiftAnimationWaypoints {
    east: crate::vec::Vec<crate::types::Vector>,
    north: crate::vec::Vec<crate::types::Vector>,
    south: crate::vec::Vec<crate::types::Vector>,
    west: crate::vec::Vec<crate::types::Vector>,
}
