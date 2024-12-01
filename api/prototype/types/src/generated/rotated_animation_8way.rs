pub enum RotatedAnimation8Way {
    RotatedAnimation8Way {
        east: RotatedAnimation,
        north: RotatedAnimation,
        north_east: RotatedAnimation,
        north_west: RotatedAnimation,
        south: RotatedAnimation,
        south_east: RotatedAnimation,
        south_west: RotatedAnimation,
        west: RotatedAnimation,
    },
    RotatedAnimation(RotatedAnimation),
}
