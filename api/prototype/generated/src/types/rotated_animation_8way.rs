#[derive(serde::Deserialize)]
pub enum RotatedAnimation8Way {
    #[serde(untagged)]
    RotatedAnimation8Way {
        east: crate::types::RotatedAnimation,
        north: crate::types::RotatedAnimation,
        north_east: crate::types::RotatedAnimation,
        north_west: crate::types::RotatedAnimation,
        south: crate::types::RotatedAnimation,
        south_east: crate::types::RotatedAnimation,
        south_west: crate::types::RotatedAnimation,
        west: crate::types::RotatedAnimation,
    },
    #[serde(untagged)]
    RotatedAnimation(Box<crate::types::RotatedAnimation>),
}
