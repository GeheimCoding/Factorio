#[derive(Debug, serde::Deserialize)]
pub struct ConnectableEntityGraphics {
    corner_left_down: crate::types::SpriteVariations,
    corner_left_up: crate::types::SpriteVariations,
    corner_right_down: crate::types::SpriteVariations,
    corner_right_up: crate::types::SpriteVariations,
    cross: crate::types::SpriteVariations,
    ending_down: crate::types::SpriteVariations,
    ending_left: crate::types::SpriteVariations,
    ending_right: crate::types::SpriteVariations,
    ending_up: crate::types::SpriteVariations,
    single: crate::types::SpriteVariations,
    straight_horizontal: crate::types::SpriteVariations,
    straight_vertical: crate::types::SpriteVariations,
    t_down: crate::types::SpriteVariations,
    t_left: crate::types::SpriteVariations,
    t_right: crate::types::SpriteVariations,
    t_up: crate::types::SpriteVariations,
}
