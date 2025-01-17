#[derive(serde::Deserialize)]
pub struct SpaceTileEffectParameters {
    nebula_brightness: f32,
    nebula_saturation: f32,
    nebula_scale: f32,
    scroll_factor: f32,
    star_brightness: f32,
    star_density: f32,
    star_parallax: f32,
    star_saturations: f32,
    star_scale: f32,
    star_shape: f32,
    zoom_base_factor: f32,
    zoom_base_offset: f32,
    zoom_exponent: f32,
    zoom_factor: f32,
    zoom_offset: f32,
}
