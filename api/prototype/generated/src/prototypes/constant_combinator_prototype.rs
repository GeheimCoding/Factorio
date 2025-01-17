#[derive(serde::Deserialize)]
pub struct ConstantCombinatorPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    activity_led_light: crate::types::LightDefinition,
    activity_led_light_offsets: (
        crate::types::Vector,
        crate::types::Vector,
        crate::types::Vector,
        crate::types::Vector,
    ),
    activity_led_sprites: crate::types::Sprite4Way,
    circuit_wire_connection_points: (
        crate::types::WireConnectionPoint,
        crate::types::WireConnectionPoint,
        crate::types::WireConnectionPoint,
        crate::types::WireConnectionPoint,
    ),
    circuit_wire_max_distance: f64,
    draw_circuit_wires: bool,
    draw_copper_wires: bool,
    sprites: crate::types::Sprite4Way,
}
