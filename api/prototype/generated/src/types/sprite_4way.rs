#[derive(serde::Deserialize)]
pub enum Sprite4Way {
    #[serde(untagged)]
    Sprite4Way {
        east: crate::types::Sprite,
        north: crate::types::Sprite,
        sheet: crate::types::SpriteNWaySheet,
        sheets: Vec<crate::types::SpriteNWaySheet>,
        south: crate::types::Sprite,
        west: crate::types::Sprite,
    },
    #[serde(untagged)]
    Sprite(Box<crate::types::Sprite>),
}
