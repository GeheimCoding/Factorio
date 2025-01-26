#[derive(Debug, serde::Deserialize)]
pub struct AsteroidGraphicsSet {
    // default: `{0.05, 0.05, 0.05, 1.0}`
    ambient_light: Option<crate::types::Color>,
    #[serde(default = "default_brightness")]
    brightness: f32,
    #[serde(default = "default_light_width")]
    light_width: f32,
    lights: Option<AsteroidGraphicsSetLights>,
    #[serde(default = "default_normal_strength")]
    normal_strength: f32,
    #[serde(default = "default_rotation_speed")]
    rotation_speed: f32,
    #[serde(default = "default_specular_power")]
    specular_power: f32,
    #[serde(default = "default_specular_purity")]
    specular_purity: f32,
    #[serde(default = "default_specular_strength")]
    specular_strength: f32,
    sprite: Option<crate::types::Sprite>,
    #[serde(default = "default_sss_amount")]
    sss_amount: f32,
    #[serde(default = "default_sss_contrast")]
    sss_contrast: f32,
    variations: Option<AsteroidGraphicsSetVariations>,
}
fn default_brightness() -> f32 {
    1.0
}
fn default_light_width() -> f32 {
    0.0
}
#[derive(Debug, serde::Deserialize)]
pub enum AsteroidGraphicsSetLights {
    #[serde(untagged)]
    LightProperties(Box<crate::types::LightProperties>),
    #[serde(untagged)]
    VecLightProperties(Vec<crate::types::LightProperties>),
}
fn default_normal_strength() -> f32 {
    1.0
}
fn default_rotation_speed() -> f32 {
    0.0
}
fn default_specular_power() -> f32 {
    6.0
}
fn default_specular_purity() -> f32 {
    0.0
}
fn default_specular_strength() -> f32 {
    5.0
}
fn default_sss_amount() -> f32 {
    5.0
}
fn default_sss_contrast() -> f32 {
    1.0
}
#[derive(Debug, serde::Deserialize)]
pub enum AsteroidGraphicsSetVariations {
    #[serde(untagged)]
    AsteroidVariation(Box<crate::types::AsteroidVariation>),
    #[serde(untagged)]
    VecAsteroidVariation(Vec<crate::types::AsteroidVariation>),
}
