#[derive(Debug, serde::Deserialize)]
pub enum TurretSpecialEffectCenter {
    #[serde(untagged)]
    TurretSpecialEffectCenter {
        default: Option<Box<crate::types::Vector>>,
        east: Option<Box<crate::types::Vector>>,
        north: Option<Box<crate::types::Vector>>,
        north_east: Option<Box<crate::types::Vector>>,
        north_west: Option<Box<crate::types::Vector>>,
        south: Option<Box<crate::types::Vector>>,
        south_east: Option<Box<crate::types::Vector>>,
        south_west: Option<Box<crate::types::Vector>>,
        west: Option<Box<crate::types::Vector>>,
    },
    #[serde(untagged)]
    Vector(Box<crate::types::Vector>),
}
