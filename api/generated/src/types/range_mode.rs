#[derive(Debug, serde::Deserialize)]
pub enum RangeMode {
    #[serde(rename = "center-to-center")]
    CenterToCenter,
    #[serde(rename = "bounding-box-to-bounding-box")]
    BoundingBoxToBoundingBox,
    #[serde(rename = "center-to-bounding-box")]
    CenterToBoundingBox,
}
