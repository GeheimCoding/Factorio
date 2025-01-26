#[derive(Debug, serde::Deserialize)]
pub struct ProcessionSet {
    arrival: Vec<crate::types::ProcessionID>,
    departure: Vec<crate::types::ProcessionID>,
}
