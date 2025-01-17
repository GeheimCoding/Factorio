#[derive(serde::Deserialize)]
pub enum BoundingBox {
    #[serde(untagged)]
    BoundingBox {
        left_top: crate::types::MapPosition,
        orientation: crate::types::RealOrientation,
        right_bottom: crate::types::MapPosition,
    },
    #[serde(untagged)]
    MapPositionMapPosition((crate::types::MapPosition, crate::types::MapPosition)),
}
