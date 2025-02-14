#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum Shooting {
    NotShooting = 0,
    ShootingEnemies = 1,
    ShootingSelected = 2,
}
