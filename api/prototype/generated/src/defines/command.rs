#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum Command {
    Attack = 1,
    AttackArea = 5,
    BuildBase = 7,
    Compound = 3,
    Flee = 8,
    GoToLocation = 2,
    Group = 4,
    Stop = 9,
    Wander = 6,
}
