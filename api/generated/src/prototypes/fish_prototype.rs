#[derive(Debug, serde::Deserialize)]
pub struct FishPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithHealthPrototype,
    pictures: Option<crate::types::SpriteVariations>,
}
