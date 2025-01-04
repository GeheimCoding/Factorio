pub enum RotatedAnimation8Way {
    RotatedAnimation8Way {
        east: crate::types::RotatedAnimation,
        north: crate::types::RotatedAnimation,
        north_east: crate::types::RotatedAnimation,
        north_west: crate::types::RotatedAnimation,
        south: crate::types::RotatedAnimation,
        south_east: crate::types::RotatedAnimation,
        south_west: crate::types::RotatedAnimation,
        west: crate::types::RotatedAnimation,
    },
    RotatedAnimation(Box<crate::types::RotatedAnimation>),
}
