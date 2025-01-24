#[derive(serde::Deserialize)]
pub enum Sprite4Way {
    #[serde(untagged)]
    Sprite4Way {
        east: Option<crate::types::Sprite>,
        north: Option<crate::types::Sprite>,
        sheet: Option<crate::types::SpriteNWaySheet>,
        sheets: Option<Vec<crate::types::SpriteNWaySheet>>,
        south: Option<crate::types::Sprite>,
        west: Option<crate::types::Sprite>,
    },
    #[serde(untagged)]
    Sprite(Box<crate::types::Sprite>),
}
