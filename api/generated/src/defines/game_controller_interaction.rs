#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum GameControllerInteraction {
    Always = 0,
    Never = 2,
    Normal = 1,
}
