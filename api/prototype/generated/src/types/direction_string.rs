#[derive(serde::Deserialize)]
pub enum DirectionString {
    #[serde(rename = "north")]
    North,
    #[serde(rename = "north_north_east")]
    NorthNorthEast,
    #[serde(rename = "north_east")]
    NorthEast,
    #[serde(rename = "east_north_east")]
    EastNorthEast,
    #[serde(rename = "east")]
    East,
    #[serde(rename = "east_south_east")]
    EastSouthEast,
    #[serde(rename = "south_east")]
    SouthEast,
    #[serde(rename = "south_south_east")]
    SouthSouthEast,
    #[serde(rename = "south")]
    South,
    #[serde(rename = "south_south_west")]
    SouthSouthWest,
    #[serde(rename = "south_west")]
    SouthWest,
    #[serde(rename = "west_south_west")]
    WestSouthWest,
    #[serde(rename = "west")]
    West,
    #[serde(rename = "west_north_west")]
    WestNorthWest,
    #[serde(rename = "north_west")]
    NorthWest,
    #[serde(rename = "north_north_west")]
    NorthNorthWest,
}
