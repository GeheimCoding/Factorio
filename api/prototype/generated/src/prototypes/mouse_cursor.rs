pub struct MouseCursor {
    filename: crate::types::FileName,
    hot_pixel_x: i16,
    hot_pixel_y: i16,
    name: String,
    system_cursor: MouseCursorSystemCursor,
    type_: String,
}
pub enum MouseCursorSystemCursor {
    Arrow,
    IBeam,
    Crosshair,
    WaitArrow,
    SizeAll,
    No,
    Hand,
}
