pub enum BoundingBox {
    BoundingBox {
        left_top: MapPosition,
        orientation: RealOrientation,
        right_bottom: MapPosition,
    },
    MapPositionMapPosition((MapPosition, MapPosition)),
}
