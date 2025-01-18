#[derive(serde::Deserialize)]
pub struct BoxSpecification {
    #[serde(default = "default_is_whole_box")]
    is_whole_box: bool,
    max_side_length: f64,
    side_height: f64,
    side_length: f64,
    sprite: crate::types::Sprite,
}
fn default_is_whole_box() -> bool {
    false
}
