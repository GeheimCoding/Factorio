#[derive(Debug, serde::Deserialize)]
pub enum Sprite4Way {
    #[serde(untagged)]
    Sprite4Way {
        east: Option<Box<crate::types::Sprite>>,
        north: Option<Box<crate::types::Sprite>>,
        sheet: Option<Box<crate::types::SpriteNWaySheet>>,
        sheets: Option<crate::vec::Vec<crate::types::SpriteNWaySheet>>,
        south: Option<Box<crate::types::Sprite>>,
        west: Option<Box<crate::types::Sprite>>,
    },
    #[serde(untagged)]
    Sprite(Box<crate::types::Sprite>),
}
