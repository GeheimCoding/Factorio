#[derive(Debug, serde::Deserialize)]
pub struct HeatPipePrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    connection_sprites: Option<crate::types::ConnectableEntityGraphics>,
    heat_buffer: crate::types::HeatBuffer,
    heat_glow_sprites: Option<crate::types::ConnectableEntityGraphics>,
    #[serde(default = "default_heating_radius")]
    heating_radius: f32,
}
fn default_heating_radius() -> f32 {
    1.0
}
