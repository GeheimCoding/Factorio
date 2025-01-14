pub struct AsteroidGraphicsSet {
    ambient_light: crate::types::Color,
    brightness: f32,
    light_width: f32,
    lights: AsteroidGraphicsSetLights,
    normal_strength: f32,
    rotation_speed: f32,
    specular_power: f32,
    specular_purity: f32,
    specular_strength: f32,
    sprite: crate::types::Sprite,
    sss_amount: f32,
    sss_contrast: f32,
    variations: AsteroidGraphicsSetVariations,
}
pub enum AsteroidGraphicsSetLights {
    LightProperties(Box<crate::types::LightProperties>),
    VecLightProperties(Vec<crate::types::LightProperties>),
}
pub enum AsteroidGraphicsSetVariations {
    AsteroidVariation(Box<crate::types::AsteroidVariation>),
    VecAsteroidVariation(Vec<crate::types::AsteroidVariation>),
}
