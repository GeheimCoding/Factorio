pub enum SimpleBoundingBox {
    SimpleBoundingBox {
        left_top: crate::types::MapPosition,
        right_bottom: crate::types::MapPosition,
    },
    MapPositionMapPosition((crate::types::MapPosition, crate::types::MapPosition)),
}
