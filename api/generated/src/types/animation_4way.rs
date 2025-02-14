#[derive(Debug, serde::Deserialize)]
pub enum Animation4Way {
    #[serde(untagged)]
    Animation4Way {
        east: Option<Box<crate::types::Animation>>,
        north: Box<crate::types::Animation>,
        north_east: Option<Box<crate::types::Animation>>,
        north_west: Option<Box<crate::types::Animation>>,
        south: Option<Box<crate::types::Animation>>,
        south_east: Option<Box<crate::types::Animation>>,
        south_west: Option<Box<crate::types::Animation>>,
        west: Option<Box<crate::types::Animation>>,
    },
    #[serde(untagged)]
    Animation(Box<crate::types::Animation>),
}
