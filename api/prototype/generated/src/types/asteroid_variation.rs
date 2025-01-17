#[derive(serde::Deserialize)]
pub struct AsteroidVariation {
    color_texture: crate::types::Sprite,
    normal_map: crate::types::Sprite,
    roughness_map: crate::types::Sprite,
    shadow_shift: crate::types::Vector,
}
