#[derive(serde::Deserialize)]
pub struct HeatPipePrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    connection_sprites: crate::types::ConnectableEntityGraphics,
    heat_buffer: crate::types::HeatBuffer,
    heat_glow_sprites: crate::types::ConnectableEntityGraphics,
}
