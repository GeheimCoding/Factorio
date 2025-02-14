#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum Controllers {
    Character = 1,
    Cutscene = 6,
    Editor = 4,
    Ghost = 0,
    God = 2,
    Remote = 7,
    Spectator = 5,
}
