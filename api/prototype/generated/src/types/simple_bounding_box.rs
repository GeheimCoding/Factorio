pub enum SimpleBoundingBox {
    SimpleBoundingBox {
        left_top: MapPosition,
        right_bottom: MapPosition,
    },
    MapPositionMapPosition((MapPosition, MapPosition)),
}
