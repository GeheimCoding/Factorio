#[derive(Debug, serde::Deserialize)]
pub enum SimpleBoundingBox {
    #[serde(untagged)]
    SimpleBoundingBox {
        left_top: crate::types::MapPosition,
        right_bottom: crate::types::MapPosition,
    },
    #[serde(untagged)]
    MapPositionMapPosition((crate::types::MapPosition, crate::types::MapPosition)),
}
