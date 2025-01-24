#[derive(serde::Deserialize)]
pub struct HeatPipePrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    connection_sprites: Option<crate::types::ConnectableEntityGraphics>,
    heat_buffer: crate::types::HeatBuffer,
    heat_glow_sprites: Option<crate::types::ConnectableEntityGraphics>,
}
