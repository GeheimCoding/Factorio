#[derive(serde::Deserialize)]
pub struct ArrowPrototype {
    base_: crate::prototypes::EntityPrototype,
    arrow_picture: crate::types::Sprite,
    blinking: bool,
    circle_picture: crate::types::Sprite,
}
