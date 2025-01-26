#[derive(Debug, serde::Deserialize)]
pub struct CargoHatchDefinition {
    #[serde(default = "default_busy_timeout_ticks")]
    busy_timeout_ticks: u32,
    cargo_unit_entity_to_spawn: Option<crate::types::EntityID>,
    closing_sound: Option<crate::types::InterruptibleSound>,
    #[serde(default = "default_entering_render_layer")]
    entering_render_layer: crate::types::RenderLayer,
    hatch_graphics: Option<crate::types::Animation>,
    #[serde(default = "default_hatch_opening_ticks")]
    hatch_opening_ticks: u32,
    #[serde(default = "default_hatch_render_layer")]
    hatch_render_layer: crate::types::RenderLayer,
    illumination_graphic_index: Option<u32>,
    // default: `{0, 0}`
    offset: Option<crate::types::Vector>,
    opening_sound: Option<crate::types::InterruptibleSound>,
    // default: `{0, 0}`
    pod_shadow_offset: Option<crate::types::Vector>,
    receiving_cargo_units: Option<Vec<crate::types::EntityID>>,
    #[serde(default = "default_sky_slice_height")]
    sky_slice_height: f32,
    #[serde(default = "default_slice_height")]
    slice_height: f32,
    #[serde(default = "default_travel_height")]
    travel_height: f32,
}
fn default_busy_timeout_ticks() -> u32 {
    120
}
fn default_entering_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::CargoHatch
}
fn default_hatch_opening_ticks() -> u32 {
    80
}
fn default_hatch_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::CargoHatch
}
fn default_sky_slice_height() -> f32 {
    -1.0
}
fn default_slice_height() -> f32 {
    1.0
}
fn default_travel_height() -> f32 {
    1.0
}
