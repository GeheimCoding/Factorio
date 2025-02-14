#[derive(Debug, serde::Deserialize)]
pub struct ProcessionSet {
    arrival: crate::vec::Vec<crate::types::ProcessionID>,
    departure: crate::vec::Vec<crate::types::ProcessionID>,
}
