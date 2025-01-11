pub struct PowerSwitchPrototype {
    circuit_wire_connection_point: crate::types::WireConnectionPoint,
    draw_circuit_wires: bool,
    draw_copper_wires: bool,
    frozen_patch: crate::types::Sprite,
    led_off: crate::types::Sprite,
    led_on: crate::types::Sprite,
    left_wire_connection_point: crate::types::WireConnectionPoint,
    overlay_loop: crate::types::Animation,
    overlay_start: crate::types::Animation,
    overlay_start_delay: u8,
    power_on_animation: crate::types::Animation,
    right_wire_connection_point: crate::types::WireConnectionPoint,
    wire_max_distance: f64,
}