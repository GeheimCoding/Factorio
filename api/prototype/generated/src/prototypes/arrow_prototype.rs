#[derive(serde::Deserialize)]
pub struct ArrowPrototype {
    base_: crate::prototypes::EntityPrototype,
    arrow_picture: crate::types::Sprite,
    #[serde(default = "default_blinking")]
    blinking: bool,
    circle_picture: crate::types::Sprite,
}
fn default_blinking() -> bool {
    false
}
