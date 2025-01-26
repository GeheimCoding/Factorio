#[derive(Debug, serde::Deserialize)]
pub struct UndergroundBeltPrototype {
    base_: crate::prototypes::TransportBeltConnectablePrototype,
    max_distance: u8,
    max_distance_tint: Option<crate::types::Color>,
    max_distance_underground_remove_belts_sprite: Option<crate::types::Sprite>,
    structure: Option<UndergroundBeltStructure>,
    // default: no masks
    underground_collision_mask: Option<crate::types::CollisionMaskConnector>,
    underground_remove_belts_sprite: Option<crate::types::Sprite>,
    underground_sprite: Option<crate::types::Sprite>,
}
#[derive(Debug, serde::Deserialize)]
pub struct UndergroundBeltStructure {
    back_patch: Option<crate::types::Sprite4Way>,
    direction_in: Option<crate::types::Sprite4Way>,
    direction_in_side_loading: Option<crate::types::Sprite4Way>,
    direction_out: Option<crate::types::Sprite4Way>,
    direction_out_side_loading: Option<crate::types::Sprite4Way>,
    front_patch: Option<crate::types::Sprite4Way>,
    frozen_patch_in: Option<crate::types::Sprite4Way>,
    frozen_patch_out: Option<crate::types::Sprite4Way>,
}
