pub struct ElectricEnergySource {
    buffer_capacity: Energy,
    drain: Energy,
    input_flow_limit: Energy,
    output_flow_limit: Energy,
    type_: Electric,
    usage_priority: ElectricUsagePriority,
}
