#[derive(Debug, serde::Deserialize)]
pub struct FusionReactorConnectionGraphics {
    fusion_effect_uv_map: Option<crate::types::Sprite>,
    pictures: Option<crate::types::Animation>,
    working_light_pictures: Option<crate::types::Animation>,
}
