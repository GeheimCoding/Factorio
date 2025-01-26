#[derive(Debug, serde::Deserialize)]
pub enum Animation4Way {
    #[serde(untagged)]
    Animation4Way {
        east: Option<crate::types::Animation>,
        north: crate::types::Animation,
        north_east: Option<crate::types::Animation>,
        north_west: Option<crate::types::Animation>,
        south: Option<crate::types::Animation>,
        south_east: Option<crate::types::Animation>,
        south_west: Option<crate::types::Animation>,
        west: Option<crate::types::Animation>,
    },
    #[serde(untagged)]
    Animation(Box<crate::types::Animation>),
}
