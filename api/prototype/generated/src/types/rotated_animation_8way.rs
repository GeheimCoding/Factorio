#[derive(serde::Deserialize)]
pub enum RotatedAnimation8Way {
    #[serde(untagged)]
    RotatedAnimation8Way {
        east: Option<crate::types::RotatedAnimation>,
        north: crate::types::RotatedAnimation,
        north_east: Option<crate::types::RotatedAnimation>,
        north_west: Option<crate::types::RotatedAnimation>,
        south: Option<crate::types::RotatedAnimation>,
        south_east: Option<crate::types::RotatedAnimation>,
        south_west: Option<crate::types::RotatedAnimation>,
        west: Option<crate::types::RotatedAnimation>,
    },
    #[serde(untagged)]
    RotatedAnimation(Box<crate::types::RotatedAnimation>),
}
