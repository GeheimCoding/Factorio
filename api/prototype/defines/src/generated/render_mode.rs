#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum RenderMode {
    Chart = 2,
    ChartZoomedIn = 3,
    Game = 1,
}
