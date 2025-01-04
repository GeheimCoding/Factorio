pub enum BoundingBox {
    BoundingBox {
        left_top: crate::types::MapPosition,
        orientation: crate::types::RealOrientation,
        right_bottom: crate::types::MapPosition,
    },
    MapPositionMapPosition((crate::types::MapPosition, crate::types::MapPosition)),
}
