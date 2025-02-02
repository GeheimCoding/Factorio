#[derive(Debug, serde::Deserialize)]
pub struct CollisionLayerPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::Prototype,
}
