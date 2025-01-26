#[derive(Debug, serde::Deserialize)]
pub struct ColumnAlignment {
    alignment: ColumnAlignmentAlignment,
    column: u32,
}
#[derive(Debug, serde::Deserialize)]
pub enum ColumnAlignmentAlignment {
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "top_left")]
    TopLeft,
    #[serde(rename = "middle_left")]
    MiddleLeft,
    #[serde(rename = "bottom_left")]
    BottomLeft,
    #[serde(rename = "top_center")]
    TopCenter,
    #[serde(rename = "middle_center")]
    MiddleCenter,
    #[serde(rename = "bottom_center")]
    BottomCenter,
    #[serde(rename = "top_right")]
    TopRight,
    #[serde(rename = "middle_right")]
    MiddleRight,
    #[serde(rename = "bottom_right")]
    BottomRight,
}
