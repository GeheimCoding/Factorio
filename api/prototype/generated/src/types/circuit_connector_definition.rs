#[derive(serde::Deserialize)]
pub struct CircuitConnectorDefinition {
    points: crate::types::WireConnectionPoint,
    sprites: crate::types::CircuitConnectorSprites,
}
