pub struct SetTileTriggerEffectItem {
    apply_on_space_platform: bool,
    apply_projection: bool,
    radius: f32,
    tile_collision_mask: CollisionMaskConnector,
    tile_name: TileID,
    type_: SetTile,
}
