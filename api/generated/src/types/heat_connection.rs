#[derive(Debug, serde::Deserialize)]
pub struct HeatConnection {
    direction: crate::types::Direction,
    position: crate::types::MapPosition,
}
