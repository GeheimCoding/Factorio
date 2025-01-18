#[derive(serde::Deserialize)]
pub struct ElectricEnergySource {
    base_: crate::types::BaseEnergySource,
    buffer_capacity: crate::types::Energy,
    drain: crate::types::Energy,
    input_flow_limit: crate::types::Energy,
    output_flow_limit: crate::types::Energy,
    #[serde(rename = "type")]
    type_: String,
    usage_priority: crate::types::ElectricUsagePriority,
}
