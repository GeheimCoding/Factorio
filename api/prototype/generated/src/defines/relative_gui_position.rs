#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum RelativeGuiPosition {
    Bottom = 1,
    Left = 2,
    Right = 3,
    Top = 0,
}
