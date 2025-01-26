#[derive(Debug, serde::Deserialize)]
pub struct CircuitConnectorSecondaryDrawOrder {
    #[serde(default = "default_east")]
    east: i8,
    #[serde(default = "default_north")]
    north: i8,
    #[serde(default = "default_south")]
    south: i8,
    #[serde(default = "default_west")]
    west: i8,
}
fn default_east() -> i8 {
    100
}
fn default_north() -> i8 {
    100
}
fn default_south() -> i8 {
    100
}
fn default_west() -> i8 {
    100
}
