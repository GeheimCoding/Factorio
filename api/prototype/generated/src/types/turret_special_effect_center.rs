#[derive(serde::Deserialize)]
pub enum TurretSpecialEffectCenter {
    #[serde(untagged)]
    TurretSpecialEffectCenter {
        default: crate::types::Vector,
        east: crate::types::Vector,
        north: crate::types::Vector,
        north_east: crate::types::Vector,
        north_west: crate::types::Vector,
        south: crate::types::Vector,
        south_east: crate::types::Vector,
        south_west: crate::types::Vector,
        west: crate::types::Vector,
    },
    #[serde(untagged)]
    Vector(crate::types::Vector),
}
