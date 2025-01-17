#[derive(serde::Deserialize)]
pub enum Animation4Way {
    #[serde(untagged)]
    Animation4Way {
        east: crate::types::Animation,
        north: crate::types::Animation,
        north_east: crate::types::Animation,
        north_west: crate::types::Animation,
        south: crate::types::Animation,
        south_east: crate::types::Animation,
        south_west: crate::types::Animation,
        west: crate::types::Animation,
    },
    #[serde(untagged)]
    Animation(Box<crate::types::Animation>),
}
