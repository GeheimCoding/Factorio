#[derive(serde::Deserialize)]
pub struct CargoHatchDefinition {
    busy_timeout_ticks: u32,
    cargo_unit_entity_to_spawn: crate::types::EntityID,
    closing_sound: crate::types::InterruptibleSound,
    entering_render_layer: crate::types::RenderLayer,
    hatch_graphics: crate::types::Animation,
    hatch_opening_ticks: u32,
    hatch_render_layer: crate::types::RenderLayer,
    illumination_graphic_index: u32,
    offset: crate::types::Vector,
    opening_sound: crate::types::InterruptibleSound,
    pod_shadow_offset: crate::types::Vector,
    receiving_cargo_units: Vec<crate::types::EntityID>,
    sky_slice_height: f32,
    slice_height: f32,
    travel_height: f32,
}
