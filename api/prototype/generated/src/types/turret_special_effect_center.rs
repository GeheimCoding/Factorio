#[derive(serde::Deserialize)]
pub enum TurretSpecialEffectCenter {
    #[serde(untagged)]
    TurretSpecialEffectCenter {
        default: Option<crate::types::Vector>,
        east: Option<crate::types::Vector>,
        north: Option<crate::types::Vector>,
        north_east: Option<crate::types::Vector>,
        north_west: Option<crate::types::Vector>,
        south: Option<crate::types::Vector>,
        south_east: Option<crate::types::Vector>,
        south_west: Option<crate::types::Vector>,
        west: Option<crate::types::Vector>,
    },
    #[serde(untagged)]
    Vector(crate::types::Vector),
}
