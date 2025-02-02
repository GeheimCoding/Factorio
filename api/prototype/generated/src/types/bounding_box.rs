#[derive(Debug, serde::Deserialize)]
pub enum BoundingBox {
    #[serde(untagged)]
    BoundingBox {
        left_top: Box<crate::types::MapPosition>,
        orientation: Option<crate::types::RealOrientation>,
        right_bottom: Box<crate::types::MapPosition>,
    },
    #[serde(untagged)]
    BoxMapPositionBoxMapPosition(
        (
            Box<crate::types::MapPosition>,
            Box<crate::types::MapPosition>,
        ),
    ),
    #[serde(untagged)]
    BoxMapPositionBoxMapPositionF32(
        (
            Box<crate::types::MapPosition>,
            Box<crate::types::MapPosition>,
            f32,
        ),
    ),
}
