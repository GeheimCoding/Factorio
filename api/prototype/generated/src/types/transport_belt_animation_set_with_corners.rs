#[derive(serde::Deserialize)]
pub struct TransportBeltAnimationSetWithCorners {
    base_: crate::types::TransportBeltAnimationSet,
    #[serde(default = "default_east_to_north_index")]
    east_to_north_index: u8,
    #[serde(default = "default_east_to_north_index_frozen")]
    east_to_north_index_frozen: u8,
    #[serde(default = "default_east_to_south_index")]
    east_to_south_index: u8,
    #[serde(default = "default_east_to_south_index_frozen")]
    east_to_south_index_frozen: u8,
    #[serde(default = "default_north_to_east_index")]
    north_to_east_index: u8,
    #[serde(default = "default_north_to_east_index_frozen")]
    north_to_east_index_frozen: u8,
    #[serde(default = "default_north_to_west_index")]
    north_to_west_index: u8,
    #[serde(default = "default_north_to_west_index_frozen")]
    north_to_west_index_frozen: u8,
    #[serde(default = "default_south_to_east_index")]
    south_to_east_index: u8,
    #[serde(default = "default_south_to_east_index_frozen")]
    south_to_east_index_frozen: u8,
    #[serde(default = "default_south_to_west_index")]
    south_to_west_index: u8,
    #[serde(default = "default_south_to_west_index_frozen")]
    south_to_west_index_frozen: u8,
    #[serde(default = "default_west_to_north_index")]
    west_to_north_index: u8,
    #[serde(default = "default_west_to_north_index_frozen")]
    west_to_north_index_frozen: u8,
    #[serde(default = "default_west_to_south_index")]
    west_to_south_index: u8,
    #[serde(default = "default_west_to_south_index_frozen")]
    west_to_south_index_frozen: u8,
}
fn default_east_to_north_index() -> u8 {
    5
}
fn default_east_to_north_index_frozen() -> u8 {
    5
}
fn default_east_to_south_index() -> u8 {
    10
}
fn default_east_to_south_index_frozen() -> u8 {
    10
}
fn default_north_to_east_index() -> u8 {
    6
}
fn default_north_to_east_index_frozen() -> u8 {
    6
}
fn default_north_to_west_index() -> u8 {
    8
}
fn default_north_to_west_index_frozen() -> u8 {
    8
}
fn default_south_to_east_index() -> u8 {
    9
}
fn default_south_to_east_index_frozen() -> u8 {
    9
}
fn default_south_to_west_index() -> u8 {
    11
}
fn default_south_to_west_index_frozen() -> u8 {
    11
}
fn default_west_to_north_index() -> u8 {
    7
}
fn default_west_to_north_index_frozen() -> u8 {
    7
}
fn default_west_to_south_index() -> u8 {
    12
}
fn default_west_to_south_index_frozen() -> u8 {
    12
}
