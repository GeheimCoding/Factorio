pub enum Sprite4Way {
    Sprite4Way {
        east: Sprite,
        north: Sprite,
        sheet: SpriteNWaySheet,
        sheets: Vec<SpriteNWaySheet>,
        south: Sprite,
        west: Sprite,
    },
    Sprite(Box<Sprite>),
}
