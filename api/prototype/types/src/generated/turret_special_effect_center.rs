pub enum TurretSpecialEffectCenter {
    TurretSpecialEffectCenter {
        default: Vector,
        east: Vector,
        north: Vector,
        north_east: Vector,
        north_west: Vector,
        south: Vector,
        south_east: Vector,
        south_west: Vector,
        west: Vector,
    },
    Vector(Vector),
}
