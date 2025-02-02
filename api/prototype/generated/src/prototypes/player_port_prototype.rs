#[derive(Debug, serde::Deserialize)]
pub struct PlayerPortPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
}
