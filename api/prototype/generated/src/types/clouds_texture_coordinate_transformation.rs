#[derive(serde::Deserialize)]
pub struct CloudsTextureCoordinateTransformation {
    scale: f32,
    #[serde(default = "default_wind_speed_factor")]
    wind_speed_factor: f32,
}
fn default_wind_speed_factor() -> f32 {
    1.0
}
