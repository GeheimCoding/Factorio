#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum Direction {
    East = 4,
    Eastnortheast = 3,
    Eastsoutheast = 5,
    North = 0,
    Northeast = 2,
    Northnortheast = 1,
    Northnorthwest = 15,
    Northwest = 14,
    South = 8,
    Southeast = 6,
    Southsoutheast = 7,
    Southsouthwest = 9,
    Southwest = 10,
    West = 12,
    Westnorthwest = 13,
    Westsouthwest = 11,
}
