#[derive(Debug, serde::Deserialize)]
pub struct MarketPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    #[serde(default = "default_allow_access_to_all_forces")]
    allow_access_to_all_forces: bool,
    picture: Option<crate::types::Sprite>,
}
fn default_allow_access_to_all_forces() -> bool {
    true
}
