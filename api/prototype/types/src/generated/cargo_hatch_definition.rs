pub struct CargoHatchDefinition {
    busy_timeout_ticks: u32,
    cargo_unit_entity_to_spawn: EntityID,
    closing_sound: InterruptibleSound,
    entering_render_layer: RenderLayer,
    hatch_graphics: Animation,
    hatch_opening_ticks: u32,
    hatch_render_layer: RenderLayer,
    illumination_graphic_index: u32,
    offset: Vector,
    opening_sound: InterruptibleSound,
    pod_shadow_offset: Vector,
    receiving_cargo_units: Vec<EntityID>,
    sky_slice_height: f32,
    slice_height: f32,
    travel_height: f32,
}
