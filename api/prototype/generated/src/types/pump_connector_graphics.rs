#[derive(Debug, serde::Deserialize)]
pub struct PumpConnectorGraphics {
    east: Vec<crate::types::PumpConnectorGraphicsAnimation>,
    north: Vec<crate::types::PumpConnectorGraphicsAnimation>,
    south: Vec<crate::types::PumpConnectorGraphicsAnimation>,
    west: Vec<crate::types::PumpConnectorGraphicsAnimation>,
}
