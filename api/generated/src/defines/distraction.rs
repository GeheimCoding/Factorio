#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum Distraction {
    ByAnything = 3,
    ByDamage = 4,
    ByEnemy = 1,
    None = 0,
}
