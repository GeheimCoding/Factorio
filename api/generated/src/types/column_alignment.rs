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
    #[serde(rename = "top-left")]
    TopLeft,
    #[serde(rename = "middle-left")]
    MiddleLeft,
    #[serde(rename = "bottom-left")]
    BottomLeft,
    #[serde(rename = "top-center")]
    TopCenter,
    #[serde(rename = "middle-center")]
    MiddleCenter,
    #[serde(rename = "bottom-center")]
    BottomCenter,
    #[serde(rename = "top-right")]
    TopRight,
    #[serde(rename = "middle-right")]
    MiddleRight,
    #[serde(rename = "bottom-right")]
    BottomRight,
}
