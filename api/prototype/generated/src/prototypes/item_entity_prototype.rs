#[derive(serde::Deserialize)]
pub struct ItemEntityPrototype {
    base_: crate::prototypes::EntityPrototype,
    collision_box: crate::types::BoundingBox,
}
