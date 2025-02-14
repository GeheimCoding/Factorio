#[derive(Debug, serde::Deserialize)]
pub struct PumpConnectorGraphics {
    east: crate::vec::Vec<crate::types::PumpConnectorGraphicsAnimation>,
    north: crate::vec::Vec<crate::types::PumpConnectorGraphicsAnimation>,
    south: crate::vec::Vec<crate::types::PumpConnectorGraphicsAnimation>,
    west: crate::vec::Vec<crate::types::PumpConnectorGraphicsAnimation>,
}
