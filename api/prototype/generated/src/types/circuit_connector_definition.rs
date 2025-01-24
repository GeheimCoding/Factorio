#[derive(serde::Deserialize)]
pub struct CircuitConnectorDefinition {
    points: Option<crate::types::WireConnectionPoint>,
    sprites: Option<crate::types::CircuitConnectorSprites>,
}
