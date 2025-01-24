#[derive(serde::Deserialize)]
pub struct RadiusVisualisationSpecification {
    #[serde(default = "default_distance")]
    distance: f64,
    #[serde(default = "default_draw_in_cursor")]
    draw_in_cursor: bool,
    #[serde(default = "default_draw_on_selection")]
    draw_on_selection: bool,
    offset: Option<crate::types::Vector>,
    sprite: Option<crate::types::Sprite>,
}
fn default_distance() -> f64 {
    0.0
}
fn default_draw_in_cursor() -> bool {
    true
}
fn default_draw_on_selection() -> bool {
    true
}
