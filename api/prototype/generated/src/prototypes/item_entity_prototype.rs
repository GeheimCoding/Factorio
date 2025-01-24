#[derive(serde::Deserialize)]
pub struct ItemEntityPrototype {
    base_: crate::prototypes::EntityPrototype,
    // default: Empty = `{{0, 0}, {0, 0}}`
    collision_box: Option<crate::types::BoundingBox>,
}
