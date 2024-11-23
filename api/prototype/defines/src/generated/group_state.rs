#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum GroupState {
    AttackingDistraction = 2,
    AttackingTarget = 3,
    Finished = 4,
    Gathering = 0,
    Moving = 1,
    Pathfinding = 5,
    WanderInGroup = 6,
}
