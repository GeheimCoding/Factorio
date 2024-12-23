pub struct CranePart {
    allow_sprite_rotation: bool,
    dying_effect: CranePartDyingEffect,
    extendable_length: Vector3D,
    extendable_length_grappler: Vector3D,
    is_contractible_by_cropping: bool,
    layer: i8,
    name: String,
    orientation_shift: f32,
    relative_position: Vector3D,
    relative_position_grappler: Vector3D,
    rotated_sprite: RotatedSprite,
    rotated_sprite_reflection: RotatedSprite,
    rotated_sprite_shadow: RotatedSprite,
    scale_to_fit_model: bool,
    should_scale_for_perspective: bool,
    snap_end: f32,
    snap_end_arm_extent_multiplier: f32,
    snap_start: f32,
    sprite: Sprite,
    sprite_reflection: Sprite,
    sprite_shadow: Sprite,
    static_length: Vector3D,
    static_length_grappler: Vector3D,
}
