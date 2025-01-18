#[derive(serde::Deserialize)]
pub struct SpaceTileEffectParameters {
    #[serde(default = "default_nebula_brightness")]
    nebula_brightness: f32,
    #[serde(default = "default_nebula_saturation")]
    nebula_saturation: f32,
    #[serde(default = "default_nebula_scale")]
    nebula_scale: f32,
    #[serde(default = "default_scroll_factor")]
    scroll_factor: f32,
    #[serde(default = "default_star_brightness")]
    star_brightness: f32,
    #[serde(default = "default_star_density")]
    star_density: f32,
    #[serde(default = "default_star_parallax")]
    star_parallax: f32,
    #[serde(default = "default_star_saturations")]
    star_saturations: f32,
    #[serde(default = "default_star_scale")]
    star_scale: f32,
    #[serde(default = "default_star_shape")]
    star_shape: f32,
    #[serde(default = "default_zoom_base_factor")]
    zoom_base_factor: f32,
    #[serde(default = "default_zoom_base_offset")]
    zoom_base_offset: f32,
    #[serde(default = "default_zoom_exponent")]
    zoom_exponent: f32,
    #[serde(default = "default_zoom_factor")]
    zoom_factor: f32,
    #[serde(default = "default_zoom_offset")]
    zoom_offset: f32,
}
fn default_nebula_brightness() -> f32 {
    0.5
}
fn default_nebula_saturation() -> f32 {
    0.9
}
fn default_nebula_scale() -> f32 {
    0.9
}
fn default_scroll_factor() -> f32 {
    0.2
}
fn default_star_brightness() -> f32 {
    1.0
}
fn default_star_density() -> f32 {
    0.0
}
fn default_star_parallax() -> f32 {
    0.1
}
fn default_star_saturations() -> f32 {
    0.5
}
fn default_star_scale() -> f32 {
    100.0
}
fn default_star_shape() -> f32 {
    1.7
}
fn default_zoom_base_factor() -> f32 {
    0.2
}
fn default_zoom_base_offset() -> f32 {
    0.8
}
fn default_zoom_exponent() -> f32 {
    1.0
}
fn default_zoom_factor() -> f32 {
    1.0
}
fn default_zoom_offset() -> f32 {
    0.0
}
