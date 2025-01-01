pub enum Animation4Way {
    Animation4Way {
        east: Animation,
        north: Animation,
        north_east: Animation,
        north_west: Animation,
        south: Animation,
        south_east: Animation,
        south_west: Animation,
        west: Animation,
    },
    Animation(Box<Animation>),
}
