#[derive(Debug, serde::Deserialize)]
pub struct TileGhostPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityPrototype,
}
