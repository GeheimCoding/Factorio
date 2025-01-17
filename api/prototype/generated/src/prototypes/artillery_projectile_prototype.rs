#[derive(serde::Deserialize)]
pub struct ArtilleryProjectilePrototype {
    base_: crate::prototypes::EntityPrototype,
    action: crate::types::Trigger,
    chart_picture: crate::types::Sprite,
    collision_box: crate::types::BoundingBox,
    final_action: crate::types::Trigger,
    height_from_ground: f32,
    map_color: crate::types::Color,
    picture: crate::types::Sprite,
    reveal_map: bool,
    rotatable: bool,
    shadow: crate::types::Sprite,
}
