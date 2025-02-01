#[derive(Debug, serde::Deserialize)]
pub struct MouseCursor {
    filename: Option<crate::types::FileName>,
    hot_pixel_x: Option<i16>,
    hot_pixel_y: Option<i16>,
    name: String,
    system_cursor: Option<MouseCursorSystemCursor>,
}
#[derive(Debug, serde::Deserialize)]
pub enum MouseCursorSystemCursor {
    #[serde(rename = "arrow")]
    Arrow,
    #[serde(rename = "i_beam")]
    IBeam,
    #[serde(rename = "crosshair")]
    Crosshair,
    #[serde(rename = "wait_arrow")]
    WaitArrow,
    #[serde(rename = "size_all")]
    SizeAll,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "hand")]
    Hand,
}
