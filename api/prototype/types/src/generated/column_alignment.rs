pub struct ColumnAlignment {
    alignment: ColumnAlignmentAlignment,
    column: u32,
}
pub enum ColumnAlignmentAlignment {
    Center,
    Left,
    Right,
    TopLeft,
    MiddleLeft,
    BottomLeft,
    TopCenter,
    MiddleCenter,
    BottomCenter,
    TopRight,
    MiddleRight,
    BottomRight,
}
