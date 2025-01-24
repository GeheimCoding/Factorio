#[derive(serde::Deserialize)]
pub struct RailPieceLayers {
    backplates: Option<crate::types::SpriteVariations>,
    metals: Option<crate::types::SpriteVariations>,
    segment_visualisation_middle: Option<crate::types::Sprite>,
    shadow_mask: Option<crate::types::Sprite>,
    shadow_subtract_mask: Option<crate::types::Sprite>,
    stone_path: Option<crate::types::SpriteVariations>,
    stone_path_background: Option<crate::types::SpriteVariations>,
    ties: Option<crate::types::SpriteVariations>,
    underwater_structure: Option<crate::types::Sprite>,
    water_reflection: Option<crate::types::Sprite>,
}
