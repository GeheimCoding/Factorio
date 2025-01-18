#[derive(serde::Deserialize)]
pub struct SimpleEntityWithForcePrototype {
    base_: crate::prototypes::SimpleEntityWithOwnerPrototype,
    #[serde(default = "default_is_military_target")]
    is_military_target: bool,
}
fn default_is_military_target() -> bool {
    true
}
