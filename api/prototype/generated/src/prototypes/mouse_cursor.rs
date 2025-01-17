pub struct MouseCursor {
    filename: crate::types::FileName,
    hot_pixel_x: i16,
    hot_pixel_y: i16,
    name: String,
    system_cursor: MouseCursorSystemCursor,
    type_: String,
}
#[derive(serde::Deserialize)]
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
