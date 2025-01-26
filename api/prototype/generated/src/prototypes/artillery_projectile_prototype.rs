#[derive(Debug, serde::Deserialize)]
pub struct ArtilleryProjectilePrototype {
    base_: crate::prototypes::EntityPrototype,
    action: Option<crate::types::Trigger>,
    chart_picture: Option<crate::types::Sprite>,
    // default: Empty = `{{0, 0}, {0, 0}}`
    collision_box: Option<crate::types::BoundingBox>,
    final_action: Option<crate::types::Trigger>,
    #[serde(default = "default_height_from_ground")]
    height_from_ground: f32,
    map_color: crate::types::Color,
    picture: Option<crate::types::Sprite>,
    reveal_map: bool,
    #[serde(default = "default_rotatable")]
    rotatable: bool,
    shadow: Option<crate::types::Sprite>,
}
fn default_height_from_ground() -> f32 {
    1.0
}
fn default_rotatable() -> bool {
    true
}
