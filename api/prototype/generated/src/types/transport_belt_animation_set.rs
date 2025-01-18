#[derive(serde::Deserialize)]
pub struct TransportBeltAnimationSet {
    #[serde(default = "default_alternate")]
    alternate: bool,
    animation_set: crate::types::RotatedAnimation,
    belt_reader: Vec<crate::types::BeltReaderLayer>,
    #[serde(default = "default_east_index")]
    east_index: u8,
    #[serde(default = "default_east_index_frozen")]
    east_index_frozen: u8,
    #[serde(default = "default_ending_east_index")]
    ending_east_index: u8,
    #[serde(default = "default_ending_east_index_frozen")]
    ending_east_index_frozen: u8,
    #[serde(default = "default_ending_north_index")]
    ending_north_index: u8,
    #[serde(default = "default_ending_north_index_frozen")]
    ending_north_index_frozen: u8,
    #[serde(default = "default_ending_south_index")]
    ending_south_index: u8,
    #[serde(default = "default_ending_south_index_frozen")]
    ending_south_index_frozen: u8,
    #[serde(default = "default_ending_west_index")]
    ending_west_index: u8,
    #[serde(default = "default_ending_west_index_frozen")]
    ending_west_index_frozen: u8,
    frozen_patch: crate::types::RotatedSprite,
    #[serde(default = "default_north_index")]
    north_index: u8,
    #[serde(default = "default_north_index_frozen")]
    north_index_frozen: u8,
    #[serde(default = "default_south_index")]
    south_index: u8,
    #[serde(default = "default_south_index_frozen")]
    south_index_frozen: u8,
    #[serde(default = "default_starting_east_index")]
    starting_east_index: u8,
    #[serde(default = "default_starting_east_index_frozen")]
    starting_east_index_frozen: u8,
    #[serde(default = "default_starting_north_index")]
    starting_north_index: u8,
    #[serde(default = "default_starting_north_index_frozen")]
    starting_north_index_frozen: u8,
    #[serde(default = "default_starting_south_index")]
    starting_south_index: u8,
    #[serde(default = "default_starting_south_index_frozen")]
    starting_south_index_frozen: u8,
    #[serde(default = "default_starting_west_index")]
    starting_west_index: u8,
    #[serde(default = "default_starting_west_index_frozen")]
    starting_west_index_frozen: u8,
    #[serde(default = "default_west_index")]
    west_index: u8,
    #[serde(default = "default_west_index_frozen")]
    west_index_frozen: u8,
}
fn default_alternate() -> bool {
    false
}
fn default_east_index() -> u8 {
    1
}
fn default_east_index_frozen() -> u8 {
    1
}
fn default_ending_east_index() -> u8 {
    20
}
fn default_ending_east_index_frozen() -> u8 {
    20
}
fn default_ending_north_index() -> u8 {
    18
}
fn default_ending_north_index_frozen() -> u8 {
    18
}
fn default_ending_south_index() -> u8 {
    14
}
fn default_ending_south_index_frozen() -> u8 {
    14
}
fn default_ending_west_index() -> u8 {
    16
}
fn default_ending_west_index_frozen() -> u8 {
    16
}
fn default_north_index() -> u8 {
    3
}
fn default_north_index_frozen() -> u8 {
    3
}
fn default_south_index() -> u8 {
    4
}
fn default_south_index_frozen() -> u8 {
    4
}
fn default_starting_east_index() -> u8 {
    19
}
fn default_starting_east_index_frozen() -> u8 {
    19
}
fn default_starting_north_index() -> u8 {
    17
}
fn default_starting_north_index_frozen() -> u8 {
    17
}
fn default_starting_south_index() -> u8 {
    13
}
fn default_starting_south_index_frozen() -> u8 {
    13
}
fn default_starting_west_index() -> u8 {
    15
}
fn default_starting_west_index_frozen() -> u8 {
    15
}
fn default_west_index() -> u8 {
    2
}
fn default_west_index_frozen() -> u8 {
    2
}
