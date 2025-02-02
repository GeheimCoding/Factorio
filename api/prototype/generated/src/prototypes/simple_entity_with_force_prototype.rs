#[derive(Debug, serde::Deserialize)]
pub struct SimpleEntityWithForcePrototype {
    #[serde(flatten)]
    base_: crate::prototypes::SimpleEntityWithOwnerPrototype,
    #[serde(default = "default_is_military_target")]
    is_military_target: bool,
}
fn default_is_military_target() -> bool {
    true
}
