pub struct ElectricEnergySource {
    buffer_capacity: crate::types::Energy,
    drain: crate::types::Energy,
    input_flow_limit: crate::types::Energy,
    output_flow_limit: crate::types::Energy,
    type_: Electric,
    usage_priority: crate::types::ElectricUsagePriority,
}
