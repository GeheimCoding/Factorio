pub struct UndergroundBeltPrototype {
    base_: crate::prototypes::TransportBeltConnectablePrototype,
    max_distance: u8,
    max_distance_tint: crate::types::Color,
    max_distance_underground_remove_belts_sprite: crate::types::Sprite,
    structure: UndergroundBeltStructure,
    underground_collision_mask: crate::types::CollisionMaskConnector,
    underground_remove_belts_sprite: crate::types::Sprite,
    underground_sprite: crate::types::Sprite,
}
pub struct UndergroundBeltStructure {
    back_patch: crate::types::Sprite4Way,
    direction_in: crate::types::Sprite4Way,
    direction_in_side_loading: crate::types::Sprite4Way,
    direction_out: crate::types::Sprite4Way,
    direction_out_side_loading: crate::types::Sprite4Way,
    front_patch: crate::types::Sprite4Way,
    frozen_patch_in: crate::types::Sprite4Way,
    frozen_patch_out: crate::types::Sprite4Way,
}
