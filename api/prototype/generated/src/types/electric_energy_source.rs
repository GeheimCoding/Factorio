#[derive(Debug, serde::Deserialize)]
pub struct ElectricEnergySource {
    base_: crate::types::BaseEnergySource,
    buffer_capacity: Option<crate::types::Energy>,
    drain: Option<crate::types::Energy>,
    // default: Max `double` value
    input_flow_limit: Option<crate::types::Energy>,
    // default: Max `double` value
    output_flow_limit: Option<crate::types::Energy>,
    #[serde(rename = "type")]
    type_: String,
    usage_priority: crate::types::ElectricUsagePriority,
}
