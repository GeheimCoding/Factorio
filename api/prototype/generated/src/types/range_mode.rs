#[derive(serde::Deserialize)]
pub enum RangeMode {
    #[serde(rename = "center_to_center")]
    CenterToCenter,
    #[serde(rename = "bounding_box_to_bounding_box")]
    BoundingBoxToBoundingBox,
    #[serde(rename = "center_to_bounding_box")]
    CenterToBoundingBox,
}
