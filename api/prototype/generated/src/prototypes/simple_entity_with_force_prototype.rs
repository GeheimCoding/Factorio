#[derive(serde::Deserialize)]
pub struct SimpleEntityWithForcePrototype {
    base_: crate::prototypes::SimpleEntityWithOwnerPrototype,
    is_military_target: bool,
}
