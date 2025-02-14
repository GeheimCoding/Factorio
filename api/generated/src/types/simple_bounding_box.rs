#[derive(Debug, serde::Deserialize)]
pub enum SimpleBoundingBox {
    #[serde(untagged)]
    SimpleBoundingBox {
        left_top: Box<crate::types::MapPosition>,
        right_bottom: Box<crate::types::MapPosition>,
    },
    #[serde(untagged)]
    BoxMapPositionBoxMapPosition(
        (
            Box<crate::types::MapPosition>,
            Box<crate::types::MapPosition>,
        ),
    ),
}
