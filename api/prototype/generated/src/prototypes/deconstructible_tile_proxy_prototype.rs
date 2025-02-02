#[derive(Debug, serde::Deserialize)]
pub struct DeconstructibleTileProxyPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityPrototype,
}
