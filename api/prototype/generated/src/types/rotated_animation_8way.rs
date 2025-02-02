#[derive(Debug, serde::Deserialize)]
pub enum RotatedAnimation8Way {
    #[serde(untagged)]
    RotatedAnimation8Way {
        east: Option<Box<crate::types::RotatedAnimation>>,
        north: Box<crate::types::RotatedAnimation>,
        north_east: Option<Box<crate::types::RotatedAnimation>>,
        north_west: Option<Box<crate::types::RotatedAnimation>>,
        south: Option<Box<crate::types::RotatedAnimation>>,
        south_east: Option<Box<crate::types::RotatedAnimation>>,
        south_west: Option<Box<crate::types::RotatedAnimation>>,
        west: Option<Box<crate::types::RotatedAnimation>>,
    },
    #[serde(untagged)]
    RotatedAnimation(Box<crate::types::RotatedAnimation>),
}
