pub enum Sprite4Way {
    Sprite4Way {
        east: crate::types::Sprite,
        north: crate::types::Sprite,
        sheet: crate::types::SpriteNWaySheet,
        sheets: Vec<crate::types::SpriteNWaySheet>,
        south: crate::types::Sprite,
        west: crate::types::Sprite,
    },
    Sprite(Box<crate::types::Sprite>),
}
