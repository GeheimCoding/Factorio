pub struct CombinatorPrototype {
    active_energy_usage: crate::types::Energy,
    activity_led_hold_time: u8,
    activity_led_light: crate::types::LightDefinition,
    activity_led_light_offsets: (
        crate::types::Vector,
        crate::types::Vector,
        crate::types::Vector,
        crate::types::Vector,
    ),
    activity_led_sprites: crate::types::Sprite4Way,
    circuit_wire_max_distance: f64,
    draw_circuit_wires: bool,
    draw_copper_wires: bool,
    emissions_per_second: std::collections::HashMap<crate::types::AirbornePollutantID, f64>,
    energy_source: CombinatorPrototypeEnergySource,
    frozen_patch: crate::types::Sprite4Way,
    input_connection_bounding_box: crate::types::BoundingBox,
    input_connection_points: (
        crate::types::WireConnectionPoint,
        crate::types::WireConnectionPoint,
        crate::types::WireConnectionPoint,
        crate::types::WireConnectionPoint,
    ),
    output_connection_bounding_box: crate::types::BoundingBox,
    output_connection_points: (
        crate::types::WireConnectionPoint,
        crate::types::WireConnectionPoint,
        crate::types::WireConnectionPoint,
        crate::types::WireConnectionPoint,
    ),
    screen_light: crate::types::LightDefinition,
    screen_light_offsets: (
        crate::types::Vector,
        crate::types::Vector,
        crate::types::Vector,
        crate::types::Vector,
    ),
    sprites: crate::types::Sprite4Way,
}
pub enum CombinatorPrototypeEnergySource {
    ElectricEnergySource(Box<crate::types::ElectricEnergySource>),
    VoidEnergySource(Box<crate::types::VoidEnergySource>),
}
